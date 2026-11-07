//! @title Radix Sort
//! @chapter 8.3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Sort d-digit keys with d passes of a stable sort, least-significant digit first.
//! @order 110
use crate::outln;

fn stable_by_digit(a: &[u64], base: u64, exp: u64) -> Vec<u64> {
    let mut c = vec![0usize; base as usize + 1];
    for &x in a { c[((x / exp) % base) as usize + 1] += 1; }
    for i in 1..c.len() { c[i] += c[i - 1]; }
    let mut b = vec![0u64; a.len()];
    for &x in a {
        let d = ((x / exp) % base) as usize;
        b[c[d]] = x;
        c[d] += 1;
    }
    b
}

/// RADIX-SORT(A, d) using a stable counting sort on each digit in the given base.
pub fn radix_sort(a: &[u64], base: u64) -> Vec<u64> {
    assert!(base >= 2);
    let mx = a.iter().copied().max().unwrap_or(0);
    let mut out = a.to_vec();
    let mut exp = 1u64;
    loop {
        out = stable_by_digit(&out, base, exp);
        match exp.checked_mul(base) {
            Some(e) if mx / e > 0 => exp = e,
            _ => break,
        }
    }
    out
}

/// Exercise 8.3-4-style helper: sort equal-length strings (bytes) by LSD radix sort.
pub fn radix_sort_strings(a: &[&str]) -> Vec<String> {
    let len = a.iter().map(|s| s.len()).max().unwrap_or(0);
    let mut v: Vec<Vec<u8>> = a.iter().map(|s| {
        let mut b = s.as_bytes().to_vec();
        b.resize(len, 0);
        b
    }).collect();
    for pos in (0..len).rev() {
        let keys = super::counting_sort::counting_sort_by_key(&v, 255, |s| s[pos] as usize);
        v = keys;
    }
    v.into_iter().map(|b| String::from_utf8(b.into_iter().filter(|&c| c != 0).collect()).unwrap()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let a = [329, 457, 657, 839, 436, 720, 355];
    outln!(o, "input          : {:?}", a);
    outln!(o, "radix (base 10): {:?}", radix_sort(&a, 10));
    outln!(o, "radix (base 256): {:?}", radix_sort(&a, 256));
    outln!(o, "strings        : {:?}", radix_sort_strings(&["pear", "fig", "apple", "kiwi", "banana", "date"]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn matches_std_sort_for_several_bases() {
        let mut r = Rng::new(34);
        let v: Vec<u64> = r.vec(500, 0, 1_000_000).into_iter().map(|x| x as u64).collect();
        let mut e = v.clone();
        e.sort();
        for base in [2, 10, 16, 256, 1000] {
            assert_eq!(radix_sort(&v, base), e, "base {base}");
        }
        assert_eq!(radix_sort(&[], 10), Vec::<u64>::new());
        assert_eq!(radix_sort(&[0, 0], 10), vec![0, 0]);
    }
    #[test]
    fn handles_u64_extremes() {
        let v = vec![u64::MAX, 0, 1 << 63, 5];
        assert_eq!(radix_sort(&v, 256), vec![0, 5, 1 << 63, u64::MAX]);
    }
    #[test]
    fn sorts_strings() {
        let out = radix_sort_strings(&["pear", "fig", "apple", "kiwi"]);
        assert_eq!(out, vec!["apple", "fig", "kiwi", "pear"]);
    }
}
