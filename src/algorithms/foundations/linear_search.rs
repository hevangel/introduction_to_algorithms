//! @title Linear Search
//! @chapter Exercise 2.1-3 (1e–4e)
//! @editions 1,2,3,4
//! @summary Scan until the value is found; a loop-invariant warm-up exercise.
//! @order 20
use crate::outln;

/// LINEAR-SEARCH(A, v): index of the first element equal to `v`.
pub fn linear_search<T: PartialEq>(a: &[T], v: &T) -> Option<usize> {
    for i in 0..a.len() {
        if a[i] == *v {
            return Some(i);
        }
    }
    None
}

/// Sentinel version: one comparison per element instead of two.
pub fn sentinel_search(a: &mut Vec<i64>, v: i64) -> Option<usize> {
    let n = a.len();
    a.push(v); // sentinel
    let mut i = 0;
    while a[i] != v {
        i += 1;
    }
    a.pop();
    if i < n { Some(i) } else { None }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![31, 41, 59, 26, 41, 58];
    outln!(o, "A = {:?}", a);
    outln!(o, "linear_search(41)   = {:?}", linear_search(&a, &41));
    outln!(o, "linear_search(99)   = {:?}", linear_search(&a, &99));
    outln!(o, "sentinel_search(26) = {:?}", sentinel_search(&mut a, 26));
    outln!(o, "sentinel_search(7)  = {:?}", sentinel_search(&mut a, 7));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_first_occurrence() {
        let a = [3, 1, 4, 1, 5];
        assert_eq!(linear_search(&a, &1), Some(1));
        assert_eq!(linear_search(&a, &9), None);
        assert_eq!(linear_search::<i32>(&[], &1), None);
    }
    #[test]
    fn sentinel_matches_linear() {
        let mut a = vec![3, 1, 4, 1, 5];
        for v in 0..7 {
            let e = linear_search(&a, &v);
            assert_eq!(sentinel_search(&mut a, v), e);
        }
        assert_eq!(a.len(), 5);
    }
}
