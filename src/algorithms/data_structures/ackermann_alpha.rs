//! @title The Ackermann Function and its Inverse α(n)
//! @chapter 21.4 (1e–4e)
//! @editions 1,2,3,4
//! @summary A_k(j) grows faster than any primitive recursive function; α(n), its inverse, is ≤ 4 for every n below 2^2048 — the amortised cost per union-find operation.
//! @order 25
use crate::outln;

/// A_k(j) of Section 21.4: A₀(j) = j + 1 and A_k(j) = A_{k−1} applied (j + 1) times to j. `None` if the value exceeds `u128`.
pub fn ackermann(k: u32, j: u128) -> Option<u128> {
    match k {
        0 => j.checked_add(1),
        1 => j.checked_mul(2)?.checked_add(1),                                  // closed form 2j + 1
        2 => { if j + 1 >= 128 { return None; } (1u128 << (j + 1)).checked_mul(j + 1)?.checked_sub(1) } // closed form 2^(j+1)(j+1) − 1
        _ => { let mut x = j; for _ in 0..=j { x = ackermann(k - 1, x)?; } Some(x) }
    }
}

/// The literal definition by iteration (no closed forms), usable for tiny arguments only.
pub fn ackermann_by_definition(k: u32, j: u64) -> u128 {
    if k == 0 { return j as u128 + 1; }
    let mut x = j as u128;
    for _ in 0..=j { x = ackermann_by_definition(k - 1, x as u64); }
    x
}

/// α(n) = min { k : A_k(1) ≥ n }.
pub fn alpha(n: u128) -> u32 { (0..).find(|&k| ackermann(k, 1).is_none_or(|v| v >= n)).unwrap() }

pub fn demo() -> String {
    let mut o = String::new();
    for k in 0..4u32 { outln!(o, "A_{k}(1) = {:?}", ackermann(k, 1)); }
    outln!(o, "A_4(1) = {:?}  (= 2^2048 − 3 … far beyond u128)", ackermann(4, 1));
    for n in [1u128, 2, 3, 7, 8, 2047, 2048, u128::MAX] { outln!(o, "α({n}) = {}", alpha(n)); }
    outln!(o, "A_2(3) = {:?}, A_3(2) = {:?}", ackermann(2, 3), ackermann(3, 2));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closed_forms_agree_with_the_definition() {
        for j in 0..6u64 {
            assert_eq!(ackermann(1, j as u128), Some(ackermann_by_definition(1, j)));
            assert_eq!(ackermann(2, j as u128), Some(ackermann_by_definition(2, j)));
        }
        assert_eq!(ackermann_by_definition(3, 1), 2047);
        assert_eq!(ackermann(3, 1), Some(2047));
        assert_eq!((ackermann(0, 1), ackermann(1, 1), ackermann(2, 1)), (Some(2), Some(3), Some(7)));
    }
    #[test]
    fn inverse_thresholds() {
        let table = [(0u128, 0u32), (1, 0), (2, 0), (3, 1), (4, 2), (7, 2), (8, 3), (2047, 3), (2048, 4), (u128::MAX, 4)];
        for (n, a) in table { assert_eq!(alpha(n), a, "α({n})"); }
        assert_eq!(ackermann(4, 1), None);
        assert_eq!(ackermann(3, 2), None);
    }
}
