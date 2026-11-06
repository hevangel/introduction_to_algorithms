//! @title Chinese Remainder Theorem
//! @chapter 31.5 (1e–4e)
//! @editions 1,2,3,4
//! @summary Reconstruct x mod n₁n₂…n_k from residues modulo pairwise coprime moduli; also merging non-coprime congruences.
//! @order 40
use super::extended_euclid::mod_inverse;
use crate::outln;

/// CRT for pairwise relatively prime moduli: returns (x, n) with x ≡ aᵢ (mod nᵢ) for all i, n = Πnᵢ.
pub fn crt(residues: &[i64], moduli: &[i64]) -> Option<(i64, i64)> {
    let n: i64 = moduli.iter().product();
    let mut x: i128 = 0;
    for (&a, &ni) in residues.iter().zip(moduli) {
        let mi = n / ni;
        let inv = mod_inverse(mi % ni, ni)?; // cᵢ = mᵢ·(mᵢ⁻¹ mod nᵢ)
        x += a as i128 * (mi as i128 * inv as i128);
    }
    Some((x.rem_euclid(n as i128) as i64, n))
}

/// Merge x ≡ a₁ (mod n₁) and x ≡ a₂ (mod n₂) for arbitrary (possibly non-coprime) moduli → (x, lcm).
pub fn merge(a1: i64, n1: i64, a2: i64, n2: i64) -> Option<(i64, i64)> {
    let (d, p, _) = super::extended_euclid::extended_euclid(n1, n2);
    if (a2 - a1) % d != 0 { return None; }
    let lcm = n1 / d * n2;
    let t = ((a2 - a1) / d) as i128 * p as i128 % (n2 / d) as i128;
    let x = (a1 as i128 + n1 as i128 * t).rem_euclid(lcm as i128);
    Some((x as i64, lcm))
}

pub fn crt_general(residues: &[i64], moduli: &[i64]) -> Option<(i64, i64)> {
    let (mut x, mut n) = (0, 1);
    for (&a, &m) in residues.iter().zip(moduli) { (x, n) = merge(x, n, a.rem_euclid(m), m)?; }
    Some((x, n))
}

pub fn demo() -> String {
    let mut o = String::new();
    // Sun Tzu's puzzle: remainders 2, 3, 2 when divided by 3, 5, 7
    outln!(o, "x ≡ 2 (mod 3), x ≡ 3 (mod 5), x ≡ 2 (mod 7) → {:?}", crt(&[2, 3, 2], &[3, 5, 7]));
    outln!(o, "x ≡ 1 (mod 5), x ≡ 2 (mod 7) → {:?}", crt(&[1, 2], &[5, 7]));
    outln!(o, "non-coprime moduli: x ≡ 3 (mod 4), x ≡ 5 (mod 6) → {:?}", crt_general(&[3, 5], &[4, 6]));
    outln!(o, "inconsistent: x ≡ 1 (mod 4), x ≡ 2 (mod 6) → {:?}", crt_general(&[1, 2], &[4, 6]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sun_tzu() {
        assert_eq!(crt(&[2, 3, 2], &[3, 5, 7]), Some((23, 105)));
        assert_eq!(crt(&[1, 2], &[5, 7]), Some((16, 35)));
    }
    #[test]
    fn one_to_one_correspondence() {
        let (mods, n) = ([3i64, 5, 7], 105);
        for x in 0..n {
            let res: Vec<i64> = mods.iter().map(|m| x % m).collect();
            assert_eq!(crt(&res, &mods), Some((x, n)));
            assert_eq!(crt_general(&res, &mods), Some((x, n)));
        }
    }
    #[test]
    fn non_coprime_moduli() {
        assert_eq!(crt_general(&[3, 5], &[4, 6]), Some((11, 12)));
        assert_eq!(crt_general(&[1, 2], &[4, 6]), None);
        for (m1, m2) in [(4i64, 6), (6, 9), (12, 18)] {
            for a1 in 0..m1 { for a2 in 0..m2 {
                let brute = (0..m1 / super::super::euclid_gcd::gcd(m1 as u64, m2 as u64) as i64 * m2).find(|x| x % m1 == a1 && x % m2 == a2);
                assert_eq!(crt_general(&[a1, a2], &[m1, m2]).map(|r| r.0), brute);
            } }
        }
    }
}
