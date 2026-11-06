//! @title Counting Binary Trees & BST Shapes
//! @chapter Problem 12-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary The number of binary trees with n nodes is the Catalan number C(2n, n)/(n+1); enumerate them and map permutations to their BSTs.
//! @order 105
use crate::outln;
use std::collections::BTreeSet;

/// bₙ = Σ_{k} b_k · b_{n−1−k} with b₀ = 1 (the recurrence of Problem 12-4).
pub fn count_by_recurrence(n: usize) -> u128 {
    let mut b = vec![1u128; n + 1];
    for m in 1..=n { b[m] = (0..m).map(|k| b[k] * b[m - 1 - k]).sum(); }
    b[n]
}

/// Closed form C(2n, n)/(n + 1).
pub fn catalan(n: usize) -> u128 {
    let mut c: u128 = 1;
    for i in 0..n as u128 { c = c * 2 * (2 * i + 1) / (i + 2); }
    c
}

/// All binary tree shapes with n nodes as parenthesised strings: "" is empty, "(L R)" a node with subtrees L and R.
pub fn all_shapes(n: usize) -> Vec<String> {
    if n == 0 { return vec![String::new()]; }
    let mut out = vec![];
    for k in 0..n { for l in all_shapes(k) { for r in all_shapes(n - 1 - k) { out.push(format!("({l} {r})")); } } }
    out
}

/// The shape of the binary search tree obtained by inserting the keys in this order.
pub fn bst_shape(keys: &[usize]) -> String {
    fn go(keys: &[usize]) -> String {
        match keys.split_first() {
            None => String::new(),
            Some((&root, rest)) => {
                let (l, r): (Vec<usize>, Vec<usize>) = (rest.iter().copied().filter(|&k| k < root).collect(), rest.iter().copied().filter(|&k| k > root).collect());
                format!("({} {})", go(&l), go(&r))
            }
        }
    }
    go(keys)
}

pub fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 { return vec![vec![]]; }
    let mut out = vec![];
    for p in permutations(n - 1) { for pos in 0..=p.len() { let mut q = p.clone(); q.insert(pos, n); out.push(q); } }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "n | recurrence | C(2n,n)/(n+1) | distinct BST shapes from the n! permutations");
    for n in 0..=7 {
        let shapes: BTreeSet<String> = permutations(n).iter().map(|p| bst_shape(p)).collect();
        outln!(o, "{n} | {:>10} | {:>13} | {:>6} (of {} permutations)", count_by_recurrence(n), catalan(n), shapes.len(), permutations(n).len());
    }
    outln!(o, "3-node shapes: {:?}", all_shapes(3));
    outln!(o, "asymptotically bₙ = 4ⁿ / (√π n^{{3/2}}) (1 + O(1/n)); b_30 = {}", catalan(30));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recurrence_equals_closed_form_and_known_values() {
        assert_eq!((0..10).map(count_by_recurrence).collect::<Vec<_>>(), vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]);
        for n in 0..30 { assert_eq!(count_by_recurrence(n), catalan(n)); }
    }
    #[test]
    fn enumeration_and_permutation_mapping() {
        for n in 0..=6 {
            let shapes = all_shapes(n);
            assert_eq!(shapes.len() as u128, catalan(n));
            let uniq: BTreeSet<&String> = shapes.iter().collect();
            assert_eq!(uniq.len(), shapes.len());
            // every BST shape arises from some permutation, and only valid shapes arise
            let from_perms: BTreeSet<String> = permutations(n).iter().map(|p| bst_shape(p)).collect();
            assert_eq!(from_perms, shapes.into_iter().collect());
        }
    }
}
