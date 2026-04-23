use frand::Rand;
use frand::Random as R;
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

pub struct Random(Rand);

impl Random {
    #[inline]
    pub fn new() -> Self {
        Self(Rand::new())
    }

    #[inline(always)]
    pub fn with_word_seed(seed: &str) -> Self {
        let mut hasher = FxHasher::default();
        seed.hash(&mut hasher);
        Self::with_seed(hasher.finish())
    }

    #[inline(always)]
    pub fn with_seed(seed: u64) -> Self {
        Self(Rand::with_seed(seed))
    }

    #[inline(always)]
    pub fn next<T: R>(&mut self) -> T {
        self.0.r#gen::<T>()
    }
}