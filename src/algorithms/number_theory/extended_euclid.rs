//! @title Extended Euclid & Modular Inverses
//! @chapter 31.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary EXTENDED-EUCLID finds x, y with ax + by = gcd(a,b) (Bézout); gives modular inverses.
//! @order 20
use crate::outln;

/// EXTENDED-EUCLID(a, b) = (d, x, y) with d = gcd(a, b) = a·x + b·y.
pub fn extended_euclid(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { (a, 1, 0) } else {
        let (d, x, y) = extended_euclid(b, a % b);
        (d, y, x - (a / b) * y) // d = b·x + (a mod b)·y = a·y + b·(x − ⌊a/b⌋·y)
    }
}

/// a⁻¹ mod n if gcd(a, n) = 1.
pub fn mod_inverse(a: i64, n: i64) -> Option<i64> {
    let (d, x, _) = extended_euclid(a.rem_euclid(n), n);
    if d == 1 { Some(x.rem_euclid(n)) } else { None }
}

pub fn demo() -> String {
    let mut o = String::new();
    for (a, b) in [(99, 78), (240, 46), (17, 5)] {
        let (d, x, y) = extended_euclid(a, b);
        outln!(o, "EXTENDED-EUCLID({a}, {b}) = (d={d}, x={x}, y={y}):  {a}·({x}) + {b}·({y}) = {}", a * x + b * y);
    }
    outln!(o, "inverse of 3 mod 11 = {:?}; inverse of 6 mod 9 = {:?}", mod_inverse(3, 11), mod_inverse(6, 9));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn bezout_identity() {
        let mut r = Rng::new(211);
        for _ in 0..500 {
            let (a, b) = (r.range(0, 100_000) as i64, r.range(0, 100_000) as i64);
            let (d, x, y) = extended_euclid(a, b);
            assert_eq!(a * x + b * y, d);
            assert_eq!(d as u64, super::super::euclid_gcd::gcd(a as u64, b as u64));
        }
        assert_eq!(extended_euclid(99, 78), (3, -11, 14));
    }
    #[test]
    fn inverses() {
        assert_eq!(mod_inverse(3, 11), Some(4));
        assert_eq!(mod_inverse(6, 9), None);
        for n in [7i64, 26, 101, 1000] {
            for a in 1..n.min(60) {
                if let Some(inv) = mod_inverse(a, n) { assert_eq!(a * inv % n, 1); }
            }
        }
    }
}
