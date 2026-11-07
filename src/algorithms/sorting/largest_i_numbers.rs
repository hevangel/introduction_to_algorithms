//! @title The i Largest Numbers in Sorted Order
//! @chapter Problem 9-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Three ways to list the i largest of n numbers: sort (n lg n), heap (n + i lg n), order statistic (n + i lg i).
//! @order 215
use super::heapsort::{build_max_heap, max_heapify};
use super::select_linear_time::select;
use crate::outln;

/// (a) Sort, then take the top i: Θ(n lg n).
pub fn by_sorting(a: &[i64], i: usize) -> Vec<i64> {
    let mut v = a.to_vec();
    v.sort_by(|x, y| y.cmp(x));
    v.truncate(i);
    v
}

/// (b) Build a max-heap (Θ(n)) and call EXTRACT-MAX i times (Θ(i lg n)).
pub fn by_heap(a: &[i64], i: usize) -> Vec<i64> {
    let mut h = a.to_vec();
    build_max_heap(&mut h);
    let (mut size, mut out) = (h.len(), vec![]);
    for _ in 0..i.min(a.len()) {
        out.push(h[0]);
        h.swap(0, size - 1);
        size -= 1;
        max_heapify(&mut h, 0, size);
    }
    out
}

/// (c) Find the i-th largest with the linear-time SELECT, partition around it, sort only the i largest: Θ(n + i lg i).
pub fn by_order_statistic(a: &[i64], i: usize) -> Vec<i64> {
    if i == 0 || a.is_empty() { return vec![]; }
    let i = i.min(a.len());
    let threshold = select(a, a.len() - i + 1); // the i-th largest value
    let mut top: Vec<i64> = a.iter().copied().filter(|&x| x > threshold).collect();
    let ties = i - top.len();
    top.extend(std::iter::repeat_n(threshold, ties));
    top.sort_by(|x, y| y.cmp(x));
    top
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut r = crate::common::Rng::new(4);
    let v = r.vec(30, 0, 99);
    outln!(o, "numbers: {:?}", v);
    outln!(o, "top 5 by sorting       : {:?}", by_sorting(&v, 5));
    outln!(o, "top 5 by heap          : {:?}", by_heap(&v, 5));
    outln!(o, "top 5 by order statistic: {:?}", by_order_statistic(&v, 5));
    outln!(o, "costs for n = 10⁶, i = 20: sort ≈ n lg n = 2·10⁷; heap ≈ n + i lg n = 10⁶; select ≈ n + i lg i = 10⁶");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn all_three_agree_including_duplicates_and_extremes() {
        let mut r = Rng::new(313);
        for _ in 0..200 {
            let n = r.range(0, 60);
            let v = r.vec(n, -10, 10);
            for i in [0, 1, 3, n / 2, n, n + 3] {
                let want = by_sorting(&v, i);
                assert_eq!(by_heap(&v, i), want, "heap n={n} i={i}");
                assert_eq!(by_order_statistic(&v, i), want, "select n={n} i={i}");
            }
        }
    }
}
