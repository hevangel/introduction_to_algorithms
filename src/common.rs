//! Tiny shared helpers: a deterministic RNG (no external crates) and the demo-log macro.

/// xorshift64* pseudo-random generator — deterministic, so demos and run logs are reproducible.
#[derive(Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.max(1) ^ 0x9E37_79B9_7F4A_7C15)
    }
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    /// Uniform integer in `lo..=hi`.
    pub fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + (self.next_u64() % (hi - lo + 1) as u64) as usize
    }
    /// Uniform float in `[0, 1)`.
    pub fn f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        for i in (1..a.len()).rev() {
            let j = self.range(0, i);
            a.swap(i, j);
        }
    }
    pub fn vec(&mut self, n: usize, lo: i64, hi: i64) -> Vec<i64> {
        (0..n).map(|_| lo + (self.next_u64() % (hi - lo + 1) as u64) as i64).collect()
    }
}

/// `outln!(buf, "fmt", args…)` appends a line to a `String` (used by every `demo()`).
#[macro_export]
macro_rules! outln {
    ($o:expr, $($arg:tt)*) => {{
        use std::fmt::Write as _;
        let _ = writeln!($o, $($arg)*);
    }};
}

pub fn is_sorted<T: PartialOrd>(a: &[T]) -> bool {
    a.windows(2).all(|w| w[0] <= w[1])
}
