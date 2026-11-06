//! @title Random Permutations & Sampling
//! @chapter 5.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary PERMUTE-BY-SORTING, RANDOMIZE-IN-PLACE, random subsets, and RANDOM(a,b) from fair coin flips.
//! @order 150
use crate::common::Rng;
use crate::outln;

/// PERMUTE-BY-SORTING: give each element a random priority in [1, n³], then sort by priority.
pub fn permute_by_sorting<T: Clone>(a: &[T], rng: &mut Rng) -> Vec<T> {
    let n = a.len().max(1) as u64;
    let mut keyed: Vec<(u64, usize)> = (0..a.len()).map(|i| (rng.next_u64() % (n * n * n) + 1, i)).collect();
    keyed.sort();
    keyed.into_iter().map(|(_, i)| a[i].clone()).collect()
}

/// RANDOMIZE-IN-PLACE: Fisher–Yates/Knuth shuffle, Θ(n), uniform.
pub fn randomize_in_place<T>(a: &mut [T], rng: &mut Rng) {
    for i in 0..a.len() {
        let j = rng.range(i, a.len() - 1);
        a.swap(i, j);
    }
}

/// Exercise 5.3-7: RANDOM-SAMPLE(m, n) — a uniformly random m-subset of {1..n} (Floyd-style, as in the exercise).
pub fn random_sample(m: usize, n: usize, rng: &mut Rng) -> Vec<usize> {
    if m == 0 {
        return vec![];
    }
    let mut s = random_sample(m - 1, n - 1, rng);
    let i = rng.range(1, n);
    if s.contains(&i) {
        s.push(n);
    } else {
        s.push(i);
    }
    s
}

/// Exercise 5.1-2: RANDOM(a, b) using only fair coin flips (rejection sampling on lg(b-a+1) bits).
pub fn random_ab(a: usize, b: usize, rng: &mut Rng, coin_flips: &mut usize) -> usize {
    let range = b - a + 1;
    let bits = usize::BITS - (range - 1).leading_zeros();
    loop {
        let mut x = 0usize;
        for _ in 0..bits {
            *coin_flips += 1;
            x = (x << 1) | (rng.next_u64() & 1) as usize;
        }
        if x < range {
            return a + x;
        }
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = Rng::new(7);
    let base: Vec<u32> = (1..=8).collect();
    outln!(o, "permute_by_sorting : {:?}", permute_by_sorting(&base, &mut r));
    let mut b = base.clone();
    randomize_in_place(&mut b, &mut r);
    outln!(o, "randomize_in_place : {:?}", b);
    let mut s = random_sample(4, 20, &mut r);
    s.sort();
    outln!(o, "random_sample(4,20): {:?}", s);
    let mut flips = 0;
    let rolls: Vec<usize> = (0..10).map(|_| random_ab(1, 6, &mut r, &mut flips)).collect();
    outln!(o, "RANDOM(1,6) ×10    : {:?} using {} coin flips", rolls, flips);
    // uniformity check of randomize_in_place for 3 elements
    let mut counts = std::collections::BTreeMap::new();
    for _ in 0..6000 {
        let mut v = [1, 2, 3];
        randomize_in_place(&mut v, &mut r);
        *counts.entry(v).or_insert(0) += 1;
    }
    outln!(o, "frequencies of the 6 permutations of [1,2,3] over 6000 shuffles: {:?}", counts.values().collect::<Vec<_>>());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    fn is_perm(v: &[u32], n: u32) -> bool {
        let mut s = v.to_vec();
        s.sort();
        s == (1..=n).collect::<Vec<_>>()
    }
    #[test]
    fn outputs_are_permutations() {
        let mut r = Rng::new(12);
        let base: Vec<u32> = (1..=20).collect();
        assert!(is_perm(&permute_by_sorting(&base, &mut r), 20));
        let mut b = base.clone();
        randomize_in_place(&mut b, &mut r);
        assert!(is_perm(&b, 20));
    }
    #[test]
    fn shuffle_is_uniform_on_three_elements() {
        let mut r = Rng::new(13);
        let mut counts = std::collections::HashMap::new();
        for _ in 0..12000 {
            let mut v = [0, 1, 2];
            randomize_in_place(&mut v, &mut r);
            *counts.entry(v).or_insert(0usize) += 1;
        }
        assert_eq!(counts.len(), 6);
        for &c in counts.values() {
            assert!((1700..2300).contains(&c), "count {c}");
        }
    }
    #[test]
    fn sample_has_distinct_elements_in_range() {
        let mut r = Rng::new(14);
        for _ in 0..200 {
            let mut s = random_sample(5, 12, &mut r);
            s.sort();
            s.dedup();
            assert_eq!(s.len(), 5);
            assert!(s.iter().all(|&x| (1..=12).contains(&x)));
        }
    }
    #[test]
    fn random_ab_stays_in_range_and_covers_it() {
        let mut r = Rng::new(15);
        let mut seen = [false; 6];
        let mut f = 0;
        for _ in 0..500 {
            let x = random_ab(10, 15, &mut r, &mut f);
            assert!((10..=15).contains(&x));
            seen[x - 10] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }
}
