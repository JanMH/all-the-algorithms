pub trait PrnGenerator {
    fn next_byte(&mut self) -> u8;
}
