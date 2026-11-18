//! @title Sorting in Place in Linear Time
//! @chapter Problem 8-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Compare the three O(n) constrained-key trade-offs: O(n) worst-case, stability, and O(1) auxiliary space (in-place counting-sort cycle chasing).
//! @order 125
use crate::outln;

/// (a) O(n + k) worst-case, stable, O(n + k) auxiliary space (counting sort).
pub fn stable_linear_sort<T: Clone>(items: &[(usize, T)], k: usize) -> Vec<(usize, T)> {
    super::counting_sort::counting_sort_by_key(items, k, |r| r.0)
}

/// (b) O(n) worst-case and O(1) auxiliary space for binary keys in {0, 1} (unstable Hoare partition).
pub fn in_place_binary_partition<T>(items: &mut [(usize, T)]) {
    let mut lo = 0usize;
    let mut hi = items.len();
    while lo < hi {
        while lo < hi && items[lo].0 == 0 {
            lo += 1;
        }
        while lo < hi && items[hi - 1].0 != 0 {
            hi -= 1;
        }
        if lo < hi {
            items.swap(lo, hi - 1);
            lo += 1;
            hi -= 1;
        }
    }
}

/// (c) Stable and O(1) auxiliary space for {0, 1} keys: insertion-like block shifting takes Θ(n²) worst-case time.
pub fn stable_in_place_binary_sort<T>(items: &mut [(usize, T)]) {
    let mut write_zero = 0usize;
    for i in 0..items.len() {
        if items[i].0 == 0 {
            items[write_zero..=i].rotate_right(1);
            write_zero += 1;
        }
    }
}

/// (d) Unstable O(n + k) cycle-chasing counting sort using only O(k) count/cursor space (O(1) when k = O(1)).
pub fn in_place_counting_sort<T>(items: &mut [(usize, T)], k: usize) {
    let n = items.len();
    if n <= 1 {
        return;
    }
    let mut counts = vec![0usize; k + 1];
    for &(key, _) in items.iter() {
        counts[key] += 1;
    }
    let mut pos = vec![0usize; k + 1];
    let mut limit = vec![0usize; k + 1];
    let mut acc = 0usize;
    for v in 0..=k {
        pos[v] = acc;
        acc += counts[v];
        limit[v] = acc;
    }
    for v in 0..=k {
        while pos[v] < limit[v] {
            let i = pos[v];
            let kv = items[i].0;
            if kv == v {
                pos[v] += 1;
            } else {
                let dst = pos[kv];
                pos[kv] += 1;
                items.swap(i, dst);
            }
        }
    }
}

/// (e) In-place MSD binary radix sort over `bits`-bit keys: O(b n) time, O(b) recursion stack, O(1) heap space, unstable.
pub fn in_place_msd_radix_sort<T>(items: &mut [(u64, T)], bits: u32) {
    fn recurse<T>(slice: &mut [(u64, T)], bit: u32) {
        if slice.len() <= 1 {
            return;
        }
        let mask = 1u64 << bit;
        let mut lo = 0usize;
        let mut hi = slice.len();
        while lo < hi {
            while lo < hi && (slice[lo].0 & mask) == 0 {
                lo += 1;
            }
            while lo < hi && (slice[hi - 1].0 & mask) != 0 {
                hi -= 1;
            }
            if lo < hi {
                slice.swap(lo, hi - 1);
                lo += 1;
                hi -= 1;
            }
        }
        if bit > 0 {
            let (zeros, ones) = slice.split_at_mut(lo);
            recurse(zeros, bit - 1);
            recurse(ones, bit - 1);
        }
    }
    if bits > 0 {
        recurse(items, bits - 1);
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let sample = vec![(2, 'a'), (0, 'b'), (1, 'c'), (2, 'd'), (0, 'e'), (1, 'f')];
    outln!(o, "input records (key, tag): {:?}", sample);
    outln!(o, "(a) stable O(n+k)       : {:?}", stable_linear_sort(&sample, 2));
    let mut c_in_place = sample.clone();
    in_place_counting_sort(&mut c_in_place, 2);
    outln!(o, "(d) in-place O(k)-space : {:?}", c_in_place);
    let bin = vec![(1, 'p'), (0, 'q'), (1, 'r'), (0, 's'), (0, 't')];
    let mut b_part = bin.clone();
    in_place_binary_partition(&mut b_part);
    let mut b_stab = bin.clone();
    stable_in_place_binary_sort(&mut b_stab);
    outln!(o, "(b) 0-1 partition O(n)  : {:?}", b_part);
    outln!(o, "(c) 0-1 stable O(1) spc : {:?}", b_stab);
    let mut msd = vec![(5u64, 'a'), (2, 'b'), (7, 'c'), (1, 'd'), (4, 'e'), (2, 'f')];
    in_place_msd_radix_sort(&mut msd, 3);
    outln!(o, "(e) in-place MSD radix  : {:?}", msd);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn small_key_variants_match_sorted_order() {
        let mut r = Rng::new(8201);
        for k in [0usize, 1, 5, 12] {
            let n = r.range(0, 80);
            let recs: Vec<(usize, usize)> = (0..n).map(|i| (r.range(0, k), i)).collect();
            let stab = stable_linear_sort(&recs, k);
            assert!(stab.windows(2).all(|w| w[0].0 < w[1].0 || (w[0].0 == w[1].0 && w[0].1 < w[1].1)));

            let mut cyc = recs.clone();
            in_place_counting_sort(&mut cyc, k);
            assert!(cyc.windows(2).all(|w| w[0].0 <= w[1].0));
            let mut cyc_sorted = cyc.clone();
            cyc_sorted.sort();
            let mut expected = recs.clone();
            expected.sort();
            assert_eq!(cyc_sorted, expected);
        }
    }

    #[test]
    fn binary_and_msd_radix_variants() {
        let mut r = Rng::new(8202);
        for _ in 0..60 {
            let n = r.range(0, 60);
            let bin: Vec<(usize, usize)> = (0..n).map(|i| (r.range(0, 1), i)).collect();
            let mut part = bin.clone();
            in_place_binary_partition(&mut part);
            assert!(part.windows(2).all(|w| w[0].0 <= w[1].0));

            let mut stab = bin.clone();
            stable_in_place_binary_sort(&mut stab);
            let mut expected = bin.clone();
            expected.sort_by_key(|x| x.0);
            assert_eq!(stab, expected);

            let bits = 10u32;
            let words: Vec<(u64, usize)> = (0..n).map(|i| (r.next_u64() & ((1 << bits) - 1), i)).collect();
            let mut msd = words.clone();
            in_place_msd_radix_sort(&mut msd, bits);
            assert!(msd.windows(2).all(|w| w[0].0 <= w[1].0));
            let mut msd_canon = msd;
            msd_canon.sort();
            let mut words_canon = words;
            words_canon.sort();
            assert_eq!(msd_canon, words_canon);
        }
    }
}
