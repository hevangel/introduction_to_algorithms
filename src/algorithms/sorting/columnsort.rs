//! @title Columnsort
//! @chapter Problem 8-7 (2e–4e)
//! @editions 2,3,4
//! @summary Leighton's 8-step sorting network on an r×s matrix (r ≥ 2(s−1)², s | r): four column sorts plus fixed permutations.
//! @order 210
use crate::outln;

/// Columnsort of `r·s` numbers viewed as an r-row, s-column matrix (stored column-major). Requires r even, s | r and r ≥ 2(s−1)².
/// The output is sorted in column-major order.
pub fn columnsort(data: &[i64], r: usize, s: usize) -> Vec<i64> {
    assert_eq!(data.len(), r * s);
    assert!(r % 2 == 0 && r % s == 0 && r >= 2 * (s - 1) * (s - 1), "columnsort needs r even, s | r, r ≥ 2(s−1)²");
    let sort_columns = |v: &mut Vec<i64>| for c in v.chunks_mut(r) { c.sort(); };
    let mut a = data.to_vec();
    sort_columns(&mut a);                                                    // 1. sort each column
    let mut t = vec![0; r * s];                                              // 2. "transpose": pick up column-major, lay down row-major
    for (k, &x) in a.iter().enumerate() { t[(k % s) * r + k / s] = x; }
    a = t;
    sort_columns(&mut a);                                                    // 3. sort each column
    let mut t = vec![0; r * s];                                              // 4. inverse of step 2
    for k in 0..r * s { t[k] = a[(k % s) * r + k / s]; }
    a = t;
    sort_columns(&mut a);                                                    // 5. sort each column
    let mut shifted = vec![i64::MIN; r / 2];                                 // 6. shift by r/2 into s+1 columns, padding with ∓∞
    shifted.extend(&a);
    shifted.extend(vec![i64::MAX; r / 2]);
    sort_columns(&mut shifted);                                              // 7. sort each column
    shifted[r / 2..r / 2 + r * s].to_vec()                                   // 8. unshift
}

/// Smallest legal number of rows for s columns: the least even multiple of s that is ≥ 2(s−1)².
pub fn min_rows(s: usize) -> usize {
    let mut r = (2 * (s - 1) * (s - 1)).max(2);
    while r % s != 0 || r % 2 != 0 { r += 1; }
    r
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut rng = crate::common::Rng::new(5);
    for s in [2usize, 3, 4, 6] {
        let r = min_rows(s);
        let v = rng.vec(r * s, 0, 99);
        let mut e = v.clone();
        e.sort();
        outln!(o, "s = {s}, r = {r} ({} numbers): columnsort == sort: {}", r * s, columnsort(&v, r, s) == e);
    }
    let v: Vec<i64> = (0..36).rev().collect();
    outln!(o, "reverse 35..0 as 12×3: {:?}", columnsort(&v, 12, 3));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn sorts_random_inputs_for_legal_shapes() {
        let mut rng = Rng::new(311);
        for s in 2..=7usize {
            let r = min_rows(s);
            for _ in 0..40 {
                let v = rng.vec(r * s, -20, 20);
                let mut e = v.clone();
                e.sort();
                assert_eq!(columnsort(&v, r, s), e, "r={r} s={s}");
            }
            let rev: Vec<i64> = (0..(r * s) as i64).rev().collect();
            assert_eq!(columnsort(&rev, r, s), (0..(r * s) as i64).collect::<Vec<_>>());
        }
    }
    #[test]
    fn taller_matrices_also_work_and_bad_shapes_are_rejected() {
        let mut rng = Rng::new(312);
        let v = rng.vec(40 * 4, 0, 1000);
        let mut e = v.clone();
        e.sort();
        assert_eq!(columnsort(&v, 40, 4), e);
        assert!(std::panic::catch_unwind(|| columnsort(&[0; 12], 6, 2).len() + columnsort(&[0; 12], 4, 3).len()).is_err());
        assert_eq!(min_rows(3), 12);
    }
}
