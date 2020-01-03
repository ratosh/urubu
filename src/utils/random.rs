pub struct Random(i64);

impl Random {
    const MULTIPLIER: i64 = 0x2545_F491_4F6C_DD1D;
    const INITIAL_SEED: i64 = 0x10_5524;

    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(Random::MULTIPLIER) as u64
    }
}

impl Default for Random {
    fn default() -> Self {
        Random(Random::INITIAL_SEED)
    }
}

#[cfg(test)]
mod test {
    use crate::utils::random::Random;

    #[test]
    fn generate_same_first_random() {
        let mut random1 = Random::default();
        let mut random2 = Random::default();
        assert_eq!(random1.next(), random2.next());
    }

    #[test]
    fn first_differ_from_second() {
        let mut random = Random::default();
        assert_ne!(random.next(), random.next());
        assert_ne!(random.next(), random.next());
    }
}
