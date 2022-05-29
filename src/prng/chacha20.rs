use super::PrnGenerator;

pub struct ChaChaGenerator<const ROUNDS: usize = 20> {
    random_bytes: [u8; 64],
    key: [u8; 32],
    nonce: [u8; 12],
    next_random_byte: u8,
    counter: u32,
}

impl<const ROUNDS: usize> ChaChaGenerator<ROUNDS> {
    /// Constructs the ChaChaGenerator by reading bytes from the random device
    pub fn from_system() -> std::io::Result<ChaChaGenerator> {
        let mut key = [0; 32];
        let mut nonce = [0; 12];
        super::get_system_random_bytes(&mut key)?;
        super::get_system_random_bytes(&mut nonce)?;

        Ok(ChaChaGenerator::from_key(key, nonce))
    }

    pub fn from_key(key: [u8; 32], nonce: [u8; 12]) -> ChaChaGenerator<ROUNDS> {
        ChaChaGenerator::<ROUNDS> {
            random_bytes: [0; 64],
            key,
            nonce,
            next_random_byte: 64,
            counter: 1,
        }
    }

    fn perform_rounds(&mut self) {
        let working_vec: &mut [u32; 16] = unsafe { std::mem::transmute(&mut self.random_bytes) };
        // use random bytes as working state
        *working_vec = init_state(&self.key, self.counter, &self.nonce);
        for _ in 0..(ROUNDS / 2) {
            chacha_round(working_vec);
        }
        // if we are on a big endian system we need to flip each of the u32
        #[cfg(target_endian = "big")]
        for i in 0..16 {
            working_vec[i] = u32::to_le(working_vec[i]);
        }

        let to_add = init_state(&self.key, self.counter, &self.nonce);
        for i in 0..16 {
            working_vec[i] = working_vec[i].wrapping_add(to_add[i]);
        }

        self.counter += 1;
    }
}

impl<const ROUNDS: usize> PrnGenerator for ChaChaGenerator<ROUNDS> {
    fn next_byte(&mut self) -> u8 {
        if self.next_random_byte >= 64 {
            self.perform_rounds();
            self.next_random_byte = 0;
        }
        let result = self.random_bytes[self.next_random_byte as usize];

        self.next_random_byte += 1;
        result
    }
}

fn init_state(key: &[u8; 32], counter: u32, nonce: &[u8; 12]) -> [u32; 16] {
    let mut state: [u32; 16] = [0; 16];

    state[0] = 0x61707865;
    state[1] = 0x3320646e;
    state[2] = 0x79622d32;
    state[3] = 0x6b206574;

    for i in 0..8 {
        state[i + 4] = u32::from_le(unsafe { *(&key[i * 4] as *const u8 as *const u32) });
    }
    state[12] = counter;
    for i in 0..3 {
        state[i + 13] = u32::from_le(unsafe { *(&nonce[i * 4] as *const u8 as *const u32) });
    }
    state
}

macro_rules! chacha_quarter_round {
    ($a: expr, $b:expr, $c:expr, $d:expr) => {
        $a = $a.wrapping_add($b);
        $d ^= $a;
        $d = $d.rotate_left(16);
        $c = $c.wrapping_add($d);
        $b ^= $c;
        $b = $b.rotate_left(12);
        $a = $a.wrapping_add($b);
        $d ^= $a;
        $d = $d.rotate_left(8);
        $c = $c.wrapping_add($d);
        $b ^= $c;
        $b = $b.rotate_left(7);
    };
}

fn chacha_round(state: &mut [u32; 16]) {
    chacha_quarter_round!(state[0], state[4], state[8], state[12]);
    chacha_quarter_round!(state[1], state[5], state[9], state[13]);
    chacha_quarter_round!(state[2], state[6], state[10], state[14]);
    chacha_quarter_round!(state[3], state[7], state[11], state[15]);
    chacha_quarter_round!(state[0], state[5], state[10], state[15]);
    chacha_quarter_round!(state[1], state[6], state[11], state[12]);
    chacha_quarter_round!(state[2], state[7], state[8], state[13]);
    chacha_quarter_round!(state[3], state[4], state[9], state[14]);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_rfc7539_example_2_1_1() {
        let mut a: u32 = 0x11111111;
        let mut b: u32 = 0x01020304;
        let mut c: u32 = 0x9b8d6f43;
        let mut d: u32 = 0x01234567;
        chacha_quarter_round!(a, b, c, d);

        assert_eq!(a, 0xea2a92f4);
        assert_eq!(b, 0xcb1cf8ce);
        assert_eq!(c, 0x4581472e);
        assert_eq!(d, 0x5881c4bb);
    }

    #[test]
    fn test_rfc7539_example_2_1_2() {
        let mut state: [u32; 16] = [
            0x879531e0, 0xc5ecf37d, 0x516461b1, 0xc9a62f8a, 0x44c20ef3, 0x3390af7f, 0xd9fc690b,
            0x2a5f714c, 0x53372767, 0xb00a5631, 0x974c541a, 0x359e9963, 0x5c971061, 0x3d631689,
            0x2098d9d6, 0x91dbd320,
        ];

        chacha_quarter_round!(state[2], state[7], state[8], state[13]);

        let expected: [u32; 16] = [
            0x879531e0, 0xc5ecf37d, 0xbdb886dc, 0xc9a62f8a, 0x44c20ef3, 0x3390af7f, 0xd9fc690b,
            0xcfacafd2, 0xe46bea80, 0xb00a5631, 0x974c541a, 0x359e9963, 0x5c971061, 0xccc07c79,
            0x2098d9d6, 0x91dbd320,
        ];

        assert_eq!(state, expected);
    }

    const KEY: [u8; 32] = [
        00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d,
        0x1e, 0x1f,
    ];

    const NONCE: [u8; 12] = [
        00, 0x00, 0x00, 0x09, 0x00, 0x00, 0x00, 0x4a, 0x00, 0x00, 0x00, 0x00,
    ];

    const COUNTER: u32 = 1;

    #[test]
    fn test_rfc7539_example_2_3_2_setup() {
        let expected = [
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574, 0x03020100, 0x07060504, 0x0b0a0908,
            0x0f0e0d0c, 0x13121110, 0x17161514, 0x1b1a1918, 0x1f1e1d1c, 0x00000001, 0x09000000,
            0x4a000000, 0x00000000,
        ];
        let result = init_state(&KEY, COUNTER, &NONCE);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_rfc7539_example_2_3_2() {
        let mut generator = ChaChaGenerator::<20>::from_key(KEY.clone(), NONCE.clone());
        let mut result: [u8; 64] = [0; 64];
        for i in 0..64 {
            result[i] = generator.next_byte();
        }
        let expected = [
            0xe4e7f110, 0x15593bd1, 0x1fdd0f50, 0xc47120a3, 0xc7f4d1c7, 0x0368c033, 0x9aaa2204,
            0x4e6cd4c3, 0x466482d2, 0x09aa9f07, 0x05d7c214, 0xa2028bd9, 0xd19c12b5, 0xb94e16de,
            0xe883d0cb, 0x4e3c50a2,
        ];
        assert_eq!(
            unsafe { std::mem::transmute::<[u8; 64], [u32; 16]>(result) },
            expected
        );
    }

    
}
