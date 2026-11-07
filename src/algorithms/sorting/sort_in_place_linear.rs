//! @title Sorting in Place in Linear Time
//! @chapter Problem 8-2 (3e)
//! @editions 3,4
//! @summary Sorting n records with keys 0 or 1: a stable O(n) sort needs extra space (counting sort), an in-place O(n) sort is unstable (partition), and a stable in-place sort via rotations costs O(n lg n).
//! @order 214
use crate::outln;

/// A record with a 0/1 key and an identity that lets us observe stability.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rec { pub key: u8, pub id: usize }

/// (a)+(c) O(n) time, in place, but **not stable**: the two-pointer partition of quicksort.
pub fn partition_01(a: &mut [Rec]) {
    if a.is_empty() { return; }
    let (mut i, mut j) = (0usize, a.len() - 1);
    loop {
        while i < j && a[i].key == 0 { i += 1; }
        while i < j && a[j].key == 1 { j -= 1; }
        if i >= j { break; }
        a.swap(i, j);
    }
}

/// (a)+(b) O(n) time and **stable**, but uses Θ(n) extra space: counting sort specialised to two keys.
pub fn counting_01(a: &[Rec]) -> Vec<Rec> {
    let mut out: Vec<Rec> = a.iter().copied().filter(|r| r.key == 0).collect();
    out.extend(a.iter().copied().filter(|r| r.key == 1));
    out
}

/// (b)+(c) **stable and in place** (O(lg n) stack): sort both halves recursively, then swap the 1s of the left half with the 0s of the right half
/// by a block rotation. O(n lg n) time — Problem 8-2(e) asks whether stable, in-place and linear-time are simultaneously possible for keys 0/1
/// (they are, with more intricate block algorithms; here we show the simple O(n lg n) trade-off).
pub fn stable_in_place_01(a: &mut [Rec]) {
    if a.len() <= 1 { return; }
    let mid = a.len() / 2;
    stable_in_place_01(&mut a[..mid]);
    stable_in_place_01(&mut a[mid..]);
    let ones_left = a[..mid].iter().filter(|r| r.key == 1).count(); // the left half ends with this many 1s
    let zeros_right = a[mid..].iter().filter(|r| r.key == 0).count(); // the right half starts with this many 0s
    a[mid - ones_left..mid + zeros_right].rotate_left(ones_left);   // [1s of left | 0s of right] → [0s of right | 1s of left]
}

pub fn is_sorted_by_key(a: &[Rec]) -> bool { a.windows(2).all(|w| w[0].key <= w[1].key) }
pub fn is_stable(a: &[Rec]) -> bool { a.windows(2).all(|w| w[0].key != w[1].key || w[0].id < w[1].id) }

pub fn demo() -> String {
    let mut o = String::new();
    let input: Vec<Rec> = [1u8, 0, 1, 1, 0, 0, 1, 0].iter().enumerate().map(|(id, &key)| Rec { key, id }).collect();
    let show = |v: &[Rec]| v.iter().map(|r| format!("{}{}", r.key, ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][r.id])).collect::<Vec<_>>().join(" ");
    outln!(o, "input             : {}", show(&input));
    let mut p = input.clone();
    partition_01(&mut p);
    outln!(o, "partition (in place, O(n))   : {}  stable? {}", show(&p), is_stable(&p));
    outln!(o, "counting (stable, O(n) space): {}  stable? {}", show(&counting_01(&input)), is_stable(&counting_01(&input)));
    let mut s = input.clone();
    stable_in_place_01(&mut s);
    outln!(o, "rotations (stable, in place) : {}  stable? {}", show(&s), is_stable(&s));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_input(r: &mut Rng, n: usize) -> Vec<Rec> { (0..n).map(|id| Rec { key: r.range(0, 1) as u8, id }).collect() }
    #[test]
    fn all_three_sort_and_the_stable_ones_are_stable() {
        let mut r = Rng::new(491);
        let mut partition_was_unstable = false;
        for _ in 0..500 {
            let n = r.range(0, 40);
            let input = random_input(&mut r, n);
            let mut p = input.clone();
            partition_01(&mut p);
            assert!(is_sorted_by_key(&p));
            partition_was_unstable |= !is_stable(&p);
            let c = counting_01(&input);
            assert!(is_sorted_by_key(&c) && is_stable(&c));
            let mut s = input.clone();
            stable_in_place_01(&mut s);
            assert_eq!(s, c, "stable in-place must equal the stable counting sort");
            let mut ids: Vec<usize> = p.iter().map(|x| x.id).collect();
            ids.sort();
            assert_eq!(ids, (0..n).collect::<Vec<_>>());
        }
        assert!(partition_was_unstable, "the partition-based sort is not stable");
    }
}
