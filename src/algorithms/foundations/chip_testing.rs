//! @title Chip Testing
//! @chapter Problem 4-5 (2e–4e)
//! @editions 2,3,4
//! @summary Find one good chip among n (more than half good) using pairwise tests in Θ(n): discard pairs, recurse on half.
//! @order 195
use crate::common::Rng;
use crate::outln;

/// How a bad chip answers when it is asked about another chip.
#[derive(Clone, Copy, Debug)]
pub enum Bad { Random, AlwaysGood, AlwaysBad }

/// A collection of chips; `good[i]` is the secret truth. Tests are counted.
pub struct Lab { good: Vec<bool>, bad: Bad, rng: Rng, pub tests: usize }

impl Lab {
    pub fn new(good: Vec<bool>, bad: Bad, seed: u64) -> Self { Lab { good, bad, rng: Rng::new(seed), tests: 0 } }
    pub fn len(&self) -> usize { self.good.len() }
    pub fn is_empty(&self) -> bool { self.good.is_empty() }
    pub fn is_good(&self, i: usize) -> bool { self.good[i] }
    /// Chip `i` is asked whether chip `j` is good. Good chips tell the truth; bad ones answer per policy.
    pub fn ask(&mut self, i: usize, j: usize) -> bool {
        self.tests += 1;
        if self.good[i] { self.good[j] } else {
            match self.bad { Bad::AlwaysGood => true, Bad::AlwaysBad => false, Bad::Random => self.rng.next_u64() & 1 == 1 }
        }
    }
    /// Pairwise test: both chips are asked about each other; true iff both say "good".
    pub fn pair_test(&mut self, i: usize, j: usize) -> bool { let a = self.ask(i, j); let b = self.ask(j, i); a && b }
}

/// Problem 4-5(c): with strictly more than n/2 good chips, find one good chip using Θ(n) pairwise tests.
pub fn find_good_chip(lab: &mut Lab) -> usize {
    let mut cand: Vec<usize> = (0..lab.len()).collect();
    loop {
        if cand.len() <= 2 { return cand[0]; }
        if cand.len() % 2 == 1 {
            // odd: test the leftover chip against everybody; a good chip is endorsed by at least (n−1)/2 of the others
            let x = cand.pop().unwrap();
            let n = cand.len() + 1;
            let endorsements = cand.iter().filter(|&&c| lab.ask(c, x)).count();
            if endorsements >= (n - 1) / 2 { return x; }
        }
        // even: pair up; keep one chip from each pair that vouches for each other, throw the other pairs away
        cand = cand.chunks(2).filter(|p| lab.pair_test(p[0], p[1])).map(|p| p[0]).collect();
        if cand.is_empty() { unreachable!("strict majority invariant violated"); }
    }
}

/// Problem 4-5(d): once one good chip is known, n−1 tests identify every chip.
pub fn identify_all(lab: &mut Lab, good: usize) -> Vec<bool> { (0..lab.len()).map(|j| j == good || lab.ask(good, j)).collect() }

pub fn demo() -> String {
    let mut o = String::new();
    for (n, goods) in [(9usize, 5usize), (16, 9), (101, 51), (1000, 501)] {
        let mut pattern: Vec<bool> = (0..n).map(|i| i < goods).collect();
        Rng::new(n as u64).shuffle(&mut pattern);
        let mut lab = Lab::new(pattern, Bad::Random, 1);
        let g = find_good_chip(&mut lab);
        let used = lab.tests;
        let all = identify_all(&mut lab, g);
        outln!(o, "n = {:>4}, good = {:>3}: found good chip #{:<4} ({}) using {} tests (≤ 3n = {}); full classification correct: {}", n, goods, g, lab.is_good(g), used, 3 * n, all.iter().enumerate().all(|(i, &b)| b == lab.is_good(i)));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn always_finds_a_good_chip_with_linear_tests() {
        let mut r = Rng::new(308);
        for trial in 0..400 {
            let n = r.range(1, 60);
            let g = n / 2 + 1 + r.range(0, n - n / 2 - 1);
            let mut pattern: Vec<bool> = (0..n).map(|i| i < g).collect();
            r.shuffle(&mut pattern);
            for bad in [Bad::Random, Bad::AlwaysGood, Bad::AlwaysBad] {
                let mut lab = Lab::new(pattern.clone(), bad, trial);
                let c = find_good_chip(&mut lab);
                assert!(lab.is_good(c), "n={n} good={g} {bad:?}");
                assert!(lab.tests <= 3 * n, "{} tests for n={n}", lab.tests);
                let all = identify_all(&mut lab, c);
                assert!(all.iter().enumerate().all(|(i, &b)| b == lab.is_good(i)));
            }
        }
    }
    #[test]
    fn pair_tests_semantics() {
        let mut lab = Lab::new(vec![true, true, false], Bad::AlwaysGood, 1);
        assert!(lab.pair_test(0, 1));
        assert!(!lab.pair_test(0, 2));
        assert!(!lab.pair_test(2, 0)); // the good chip denies the bad one
        assert_eq!(lab.tests, 6);
    }
}
