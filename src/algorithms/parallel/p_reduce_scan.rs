//! @title Multithreading Reductions and Prefix Computations
//! @chapter Problem 27-4 (3e)
//! @editions 3,4
//! @summary P-REDUCE by a spawn tree, and three scan algorithms for any associative operator: P-SCAN-1 (work Θ(n²), span Θ(lg n)), P-SCAN-2 (work Θ(n lg n), span Θ(lg² n)) and P-SCAN-3 (work Θ(n), span Θ(lg n)).
//! @order 16
use crate::outln;
use std::thread;

/// P-REDUCE(x, i, j): tree reduction of x[i..=j] with an associative operator; the two halves are reduced by parallel strands.
pub fn p_reduce<T: Clone + Send + Sync>(x: &[T], op: &(impl Fn(&T, &T) -> T + Sync)) -> T {
    if x.len() == 1 { return x[0].clone(); }
    let mid = x.len() / 2;
    let (l, r) = thread::scope(|s| { let h = s.spawn(|| p_reduce(&x[..mid], op)); let r = p_reduce(&x[mid..], op); (h.join().unwrap(), r) });
    op(&l, &r)
}

/// Cost accounting for the sequentially executed models of the three scans: (operator applications, span).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Cost { pub work: usize, pub span: usize }

fn lg_ceil(n: usize) -> usize { (usize::BITS - (n.max(1) - 1).leading_zeros()) as usize }

/// P-SCAN-1: y[i] = x[0] ⊗ … ⊗ x[i], every prefix computed by its own P-REDUCE inside a parallel for. Work Θ(n²), span Θ(lg n).
pub fn p_scan_1<T: Clone>(x: &[T], op: &impl Fn(&T, &T) -> T) -> (Vec<T>, Cost) {
    let n = x.len();
    let mut cost = Cost::default();
    let y = (0..n).map(|i| { let mut acc = x[0].clone(); for v in &x[1..=i] { acc = op(&acc, v); cost.work += 1; } acc }).collect();
    cost.span = lg_ceil(n) + lg_ceil(n); // parallel-for tree + the longest reduction tree
    (y, cost)
}

/// P-SCAN-2: scan both halves recursively in parallel, then add the left total to every element of the right half with a parallel for.
/// Work Θ(n lg n), span T(n) = T(n/2) + Θ(lg n) = Θ(lg² n).
pub fn p_scan_2<T: Clone>(x: &[T], op: &impl Fn(&T, &T) -> T) -> (Vec<T>, Cost) {
    if x.len() == 1 { return (x.to_vec(), Cost::default()); }
    let mid = x.len() / 2;
    let ((mut l, cl), (r, cr)) = (p_scan_2(&x[..mid], op), p_scan_2(&x[mid..], op));
    let left_total = l.last().unwrap().clone();
    l.extend(r.iter().map(|v| op(&left_total, v)));
    (l, Cost { work: cl.work + cr.work + r.len(), span: cl.span.max(cr.span) + lg_ceil(r.len()) + 1 })
}

/// P-SCAN-3: an up-sweep that stores subtree totals t[mid] and a down-sweep that passes the prefix `v` of everything to the left.
/// Work Θ(n), span Θ(lg n). `identity` is the neutral element of the operator.
pub fn p_scan_3<T: Clone>(x: &[T], op: &impl Fn(&T, &T) -> T, identity: T) -> (Vec<T>, Cost) {
    fn up<T: Clone>(x: &[T], t: &mut Vec<Option<T>>, i: usize, j: usize, op: &impl Fn(&T, &T) -> T, c: &mut Cost) -> (T, usize) {
        if i == j { return (x[i].clone(), 0); }
        let mid = (i + j) / 2;
        let (l, dl) = up(x, t, i, mid, op, c);
        let (r, dr) = up(x, t, mid + 1, j, op, c);
        c.work += 1;
        t[mid] = Some(l.clone());
        (op(&l, &r), 1 + dl.max(dr))
    }
    fn down<T: Clone>(v: &T, x: &[T], t: &[Option<T>], y: &mut Vec<Option<T>>, i: usize, j: usize, op: &impl Fn(&T, &T) -> T, c: &mut Cost) -> usize {
        if i == j { y[i] = Some(op(v, &x[i])); c.work += 1; return 0; }
        let mid = (i + j) / 2;
        let right_v = op(v, t[mid].as_ref().unwrap());
        c.work += 1;
        let dl = down(v, x, t, y, i, mid, op, c);
        let dr = down(&right_v, x, t, y, mid + 1, j, op, c);
        1 + dl.max(dr)
    }
    let n = x.len();
    let (mut t, mut y) = (vec![None; n], vec![None; n]);
    let mut cost = Cost::default();
    let (_, du) = up(x, &mut t, 0, n - 1, op, &mut cost);
    let dd = down(&identity, x, &t, &mut y, 0, n - 1, op, &mut cost);
    cost.span = du + dd;
    (y.into_iter().map(|v| v.unwrap()).collect(), cost)
}

pub fn demo() -> String {
    let mut o = String::new();
    let add = |a: &i64, b: &i64| a + b;
    let x: Vec<i64> = (1..=16).collect();
    outln!(o, "P-REDUCE(1..16) = {}", p_reduce(&x, &add));
    outln!(o, "   n | scan-1 work/span | scan-2 work/span | scan-3 work/span");
    for n in [8usize, 64, 512] {
        let x: Vec<i64> = (0..n as i64).collect();
        let (a, b, c) = (p_scan_1(&x, &add).1, p_scan_2(&x, &add).1, p_scan_3(&x, &add, 0).1);
        outln!(o, "{n:>4} | {:>8} / {:<6} | {:>8} / {:<6} | {:>8} / {:<6}", a.work, a.span, b.work, b.span, c.work, c.span);
    }
    let words: Vec<String> = ["a", "b", "c", "d", "e"].iter().map(|s| s.to_string()).collect();
    let cat = |a: &String, b: &String| format!("{a}{b}");
    outln!(o, "scan with string concatenation (not commutative): {:?}", p_scan_3(&words, &cat, String::new()).0);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn all_scans_agree_with_the_serial_prefix_even_for_a_non_commutative_operator() {
        let cat = |a: &String, b: &String| format!("{a}{b}");
        let add = |a: &i64, b: &i64| a + b;
        let mut r = Rng::new(500);
        for _ in 0..100 {
            let n = r.range(1, 40);
            let x: Vec<i64> = r.vec(n, -50, 50);
            let want: Vec<i64> = x.iter().scan(0, |s, &v| { *s += v; Some(*s) }).collect();
            assert_eq!(p_reduce(&x, &add), *want.last().unwrap());
            assert_eq!(p_scan_1(&x, &add).0, want);
            assert_eq!(p_scan_2(&x, &add).0, want);
            assert_eq!(p_scan_3(&x, &add, 0).0, want);
            let words: Vec<String> = (0..n).map(|i| ((b'a' + (i % 26) as u8) as char).to_string()).collect();
            let wantw: Vec<String> = (0..n).map(|i| words[..=i].concat()).collect();
            assert_eq!(p_scan_2(&words, &cat).0, wantw);
            assert_eq!(p_scan_3(&words, &cat, String::new()).0, wantw);
        }
    }
    #[test]
    fn work_and_span_scale_as_claimed() {
        let add = |a: &i64, b: &i64| a + b;
        let cost = |n: usize| { let x = vec![1i64; n]; (p_scan_1(&x, &add).1, p_scan_2(&x, &add).1, p_scan_3(&x, &add, 0).1) };
        let ((a1, b1, c1), (a2, b2, c2)) = (cost(256), cost(4096));
        assert!((a2.work as f64 / a1.work as f64) > 200.0, "scan-1 work is quadratic"); // ×16 in n → ×256
        assert!((c2.work as f64 / c1.work as f64) < 17.0, "scan-3 work is linear");
        assert!((b2.work as f64 / b1.work as f64) < 30.0 && b2.work > c2.work, "scan-2 work is n lg n");
        assert!(c2.span <= 3 * 12 && b2.span > c2.span, "scan-3 span is O(lg n) while scan-2's is Θ(lg² n)");
    }
}
