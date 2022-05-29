use std::io::Read;

pub fn get_system_random_bytes(buffer: &mut [u8]) -> std::io::Result<()> {
    let mut file = std::fs::File::open("/dev/random")?;
    file.read_exact(buffer)?;
    Ok(())
}

#[test]
fn test_generates_bytes() -> std::io::Result<()> {
    let mut buffer = [0; 32];
    get_system_random_bytes(&mut buffer)?;
    let num_zeroes = buffer.iter().filter(|x| **x == 0).count();

    // likelyhood of this succeeding is 99.9999997 % calculated by summing the binomial distribution from k = 0 to 5 n =32 p = 1/256
    assert!(num_zeroes < 5);
    Ok(())
}