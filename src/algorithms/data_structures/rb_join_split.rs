//! @title Persistent Red-Black Trees: JOIN and SPLIT
//! @chapter Problems 13-1, 13-2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Immutable red-black trees (old versions stay valid) with O(lg n) JOIN of two trees and SPLIT around a key.
//! @order 125
use crate::outln;
use std::rc::Rc;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color { Red, Black }
use Color::*;

#[derive(Debug)]
pub struct Node { pub c: Color, pub l: Tree, pub k: i64, pub r: Tree, bh: usize }
pub type Tree = Option<Rc<Node>>;

fn bh(t: &Tree) -> usize { t.as_ref().map_or(0, |n| n.bh) }
fn color(t: &Tree) -> Color { t.as_ref().map_or(Black, |n| n.c) }
/// Build a node; its black-height is the black-height of the children plus one if black.
fn mk(c: Color, l: Tree, k: i64, r: Tree) -> Tree { let h = bh(&l) + (c == Black) as usize; Some(Rc::new(Node { c, l, k, r, bh: h })) }
fn blacken(t: &Tree) -> Tree { match t { Some(n) if n.c == Red => mk(Black, n.l.clone(), n.k, n.r.clone()), _ => t.clone() } }

/// bh(tl) > bh(tr): walk down the right spine of `tl` to a black node of black-height bh(tr) and hang `(that, k, tr)` there as a red node.
fn join_right(tl: &Tree, k: i64, tr: &Tree) -> Tree {
    match tl {
        None => mk(Red, None, k, tr.clone()),
        Some(n) => {
            if n.c == Black && n.bh == bh(tr) { return mk(Red, tl.clone(), k, tr.clone()); }
            let nr = join_right(&n.r, k, tr);
            if n.c == Black && color(&nr) == Red && color(&nr.as_ref().unwrap().r) == Red {
                // black(l, k', red(a, kq, red(b, kr, c)))  →  red(black(l, k', a), kq, black(b, kr, c))
                let (x, y) = (nr.as_ref().unwrap(), nr.as_ref().unwrap().r.as_ref().unwrap());
                return mk(Red, mk(Black, n.l.clone(), n.k, x.l.clone()), x.k, mk(Black, y.l.clone(), y.k, y.r.clone()));
            }
            mk(n.c, n.l.clone(), n.k, nr)
        }
    }
}
/// Mirror image of `join_right`.
fn join_left(tl: &Tree, k: i64, tr: &Tree) -> Tree {
    match tr {
        None => mk(Red, tl.clone(), k, None),
        Some(n) => {
            if n.c == Black && n.bh == bh(tl) { return mk(Red, tl.clone(), k, tr.clone()); }
            let nl = join_left(tl, k, &n.l);
            if n.c == Black && color(&nl) == Red && color(&nl.as_ref().unwrap().l) == Red {
                let (x, y) = (nl.as_ref().unwrap(), nl.as_ref().unwrap().l.as_ref().unwrap());
                return mk(Red, mk(Black, y.l.clone(), y.k, y.r.clone()), x.k, mk(Black, x.r.clone(), n.k, n.r.clone()));
            }
            mk(n.c, nl, n.k, n.r.clone())
        }
    }
}

/// RB-JOIN(T1, k, T2): every key of T1 < k < every key of T2. Returns a red-black tree of all keys in O(|bh(T1) − bh(T2)| + 1) = O(lg n).
pub fn join(tl: &Tree, k: i64, tr: &Tree) -> Tree {
    let (tl, tr) = (blacken(tl), blacken(tr));
    let t = if bh(&tl) > bh(&tr) { join_right(&tl, k, &tr) } else if bh(&tr) > bh(&tl) { join_left(&tl, k, &tr) } else { mk(Red, tl, k, tr) };
    blacken(&t)
}

/// RB-SPLIT: (keys < k, whether k was present, keys > k), in O(lg n) via a chain of joins.
pub fn split(t: &Tree, k: i64) -> (Tree, bool, Tree) {
    match t {
        None => (None, false, None),
        Some(n) if k < n.k => { let (a, f, b) = split(&n.l, k); (a, f, join(&b, n.k, &n.r)) }
        Some(n) if k > n.k => { let (a, f, b) = split(&n.r, k); (join(&n.l, n.k, &a), f, b) }
        Some(n) => (blacken(&n.l), true, blacken(&n.r)),
    }
}

pub fn max_key(t: &Tree) -> Option<i64> { let mut x = t.as_ref()?; while let Some(r) = &x.r { x = r; } Some(x.k) }

/// Concatenate two trees with all keys of `a` below all keys of `b`.
pub fn join2(a: &Tree, b: &Tree) -> Tree {
    match max_key(a) { None => blacken(b), Some(m) => { let (a2, _, _) = split(a, m); join(&a2, m, b) } }
}

/// Persistent INSERT: returns a new tree; the old one is untouched (and shares most nodes).
pub fn insert(t: &Tree, k: i64) -> Tree { let (a, f, b) = split(t, k); if f { t.clone() } else { join(&a, k, &b) } }
/// Persistent DELETE.
pub fn delete(t: &Tree, k: i64) -> Tree { let (a, _, b) = split(t, k); join2(&a, &b) }
pub fn contains(t: &Tree, k: i64) -> bool { let mut x = t; while let Some(n) = x { if k == n.k { return true; } x = if k < n.k { &n.l } else { &n.r }; } false }
pub fn inorder(t: &Tree) -> Vec<i64> { fn go(t: &Tree, o: &mut Vec<i64>) { if let Some(n) = t { go(&n.l, o); o.push(n.k); go(&n.r, o); } } let mut v = vec![]; go(t, &mut v); v }
pub fn height(t: &Tree) -> usize { t.as_ref().map_or(0, |n| 1 + height(&n.l).max(height(&n.r))) }

/// Check the red-black properties (and BST order); returns the black-height.
pub fn validate(t: &Tree) -> Result<usize, String> {
    if color(t) != Black { return Err("root is red".into()); }
    fn go(t: &Tree, lo: Option<i64>, hi: Option<i64>) -> Result<usize, String> {
        let Some(n) = t else { return Ok(0) };
        if lo.is_some_and(|l| n.k <= l) || hi.is_some_and(|h| n.k >= h) { return Err(format!("order violated at {}", n.k)); }
        if n.c == Red && (color(&n.l) == Red || color(&n.r) == Red) { return Err(format!("red-red at {}", n.k)); }
        let (a, b) = (go(&n.l, lo, Some(n.k))?, go(&n.r, Some(n.k), hi)?);
        if a != b { return Err(format!("black-height mismatch at {}", n.k)); }
        if n.bh != a + (n.c == Black) as usize { return Err(format!("stale bh at {}", n.k)); }
        Ok(n.bh)
    }
    go(t, None, None)
}

pub fn from_slice(keys: &[i64]) -> Tree { keys.iter().fold(None, |t, &k| insert(&t, k)) }

pub fn demo() -> String {
    let mut o = String::new();
    let t: Tree = from_slice(&(1..=20).collect::<Vec<_>>());
    outln!(o, "tree of 1..=20: height {}, black-height {}", height(&t), validate(&t).unwrap());
    let (l, found, r) = split(&t, 8);
    outln!(o, "split at 8 → left {:?}, found = {found}, right {:?}", inorder(&l), inorder(&r));
    outln!(o, "both halves valid red-black trees: {} / {}", validate(&l).is_ok(), validate(&r).is_ok());
    let big = from_slice(&(100..110).collect::<Vec<_>>());
    let j = join(&l, 50, &big);
    outln!(o, "join(left, 50, 100..110) = {:?}", inorder(&j));
    let t2 = insert(&t, 99);
    outln!(o, "persistence: old version still has {} keys, new version has {}", inorder(&t).len(), inorder(&t2).len());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn insert_delete_keep_invariants_and_match_a_set() {
        let mut r = Rng::new(321);
        let mut t: Tree = None;
        let mut s = std::collections::BTreeSet::new();
        for _ in 0..600 {
            let k = r.range(0, 120) as i64;
            if r.range(0, 2) < 2 { t = insert(&t, k); s.insert(k); } else { t = delete(&t, k); s.remove(&k); }
            validate(&t).unwrap();
            assert_eq!(contains(&t, k), s.contains(&k));
        }
        assert_eq!(inorder(&t), s.iter().copied().collect::<Vec<_>>());
    }
    #[test]
    fn split_and_join_roundtrip() {
        let mut r = Rng::new(322);
        for _ in 0..100 {
            let len = r.range(0, 60);
            let keys: Vec<i64> = r.vec(len, 0, 200);
            let t = from_slice(&keys);
            let pivot = r.range(0, 200) as i64;
            let (a, f, b) = split(&t, pivot);
            validate(&a).unwrap();
            validate(&b).unwrap();
            assert_eq!(f, keys.contains(&pivot));
            assert!(inorder(&a).iter().all(|&x| x < pivot) && inorder(&b).iter().all(|&x| x > pivot));
            let back = if f { join(&a, pivot, &b) } else { join2(&a, &b) };
            if !f { assert_eq!(inorder(&back).len(), inorder(&a).len() + inorder(&b).len()); } else { assert_eq!(inorder(&back), inorder(&t)); }
            validate(&back).unwrap();
        }
    }
    #[test]
    fn join_with_wildly_different_heights() {
        let small = from_slice(&[2000]);
        let big = from_slice(&(0..1000).collect::<Vec<_>>());
        let j1 = join(&big, 1500, &small);
        validate(&j1).unwrap();
        assert_eq!(inorder(&j1).len(), 1002);
        let j2 = join(&from_slice(&[-5]), -1, &big);
        validate(&j2).unwrap();
        assert!(height(&j2) as f64 <= 2.0 * (1002f64).log2() + 2.0);
        let (e, k) = (None, 5);
        validate(&join(&e, k, &e)).unwrap();
    }
    #[test]
    fn old_versions_are_persistent() {
        let t1 = from_slice(&[5, 3, 8, 1, 4]);
        let t2 = insert(&t1, 7);
        let t3 = delete(&t2, 3);
        assert_eq!(inorder(&t1), vec![1, 3, 4, 5, 8]);
        assert_eq!(inorder(&t2), vec![1, 3, 4, 5, 7, 8]);
        assert_eq!(inorder(&t3), vec![1, 4, 5, 7, 8]);
        for t in [&t1, &t2, &t3] { validate(t).unwrap(); }
    }
}
