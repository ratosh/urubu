
pub struct Random(i64);

impl Random {

    const MULTIPLIER:i64 = 0x2545F4914F6CDD1D;
    const INITIAL_SEED:i64 = 0x105524;

    pub fn new() -> Self {
        Random(Random::INITIAL_SEED)
    }

    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        return self.0.wrapping_mul(Random::MULTIPLIER) as u64;
    }
}

#[cfg(test)]
mod test {
    use crate::utils::random::Random;

    #[test]
    fn generate_same_first_random() {
        let mut random1 = Random::new();
        let mut random2 = Random::new();
        assert_eq!(random1.next(), random2.next());
    }

    #[test]
    fn first_differ_from_second() {
        let mut random = Random::new();
        assert_ne!(random.next(), random.next());
        assert_ne!(random.next(), random.next());
    }
}