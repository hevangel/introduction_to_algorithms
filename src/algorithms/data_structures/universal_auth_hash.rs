//! @title k-Universal Hashing and Carter–Wegman Authentication
//! @chapter Problem 11-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Construct strongly 2-universal and k-universal degree-(k-1) polynomial hash families over GF(p) and one-time Carter-Wegman message authentication tags.
//! @order 75
use crate::common::Rng;
use crate::outln;

/// Degree-(k-1) polynomial hash over GF(p): h_a(x) = (sum_{j=0}^{k-1} a_j x^j) mod p.
/// Uniform choice of a in Z_p^k makes this family k-universal (k-wise independent).
#[derive(Debug, Clone)]
pub struct PolyHash {
    pub p: u64,
    pub coeffs: Vec<u64>,
}

impl PolyHash {
    pub fn new(p: u64, coeffs: Vec<u64>) -> Self {
        assert!(p >= 2 && !coeffs.is_empty());
        Self { p, coeffs: coeffs.into_iter().map(|c| c % p).collect() }
    }

    pub fn random(p: u64, k: usize, rng: &mut Rng) -> Self {
        let coeffs = (0..k).map(|_| rng.next_u64() % p).collect();
        Self { p, coeffs }
    }

    /// Horner evaluation of sum_{j=0}^{k-1} a_j x^j mod p using u128 to prevent overflow.
    pub fn eval(&self, x: u64) -> u64 {
        let p = self.p as u128;
        let xm = (x as u128) % p;
        let mut acc = 0u128;
        for &c in self.coeffs.iter().rev() {
            acc = (acc * xm + c as u128) % p;
        }
        acc as u64
    }
}

/// Strongly 2-universal hashing into {0, ..., m-1}: ((a*x + b) mod p) mod m.
pub fn strongly_2_universal(p: u64, m: u64, a: u64, b: u64, x: u64) -> u64 {
    let v = ((a as u128 * (x as u128 % p as u128) + b as u128) % p as u128) as u64;
    v % m
}

/// Carter-Wegman one-time-pad authenticated tag over GF(p): tag = (h(msg) + pad) mod p.
pub fn cw_authenticate(h: &PolyHash, msg: u64, pad: u64) -> u64 {
    ((h.eval(msg) as u128 + pad as u128) % h.p as u128) as u64
}

pub fn cw_verify(h: &PolyHash, msg: u64, pad: u64, tag: u64) -> bool {
    cw_authenticate(h, msg, pad) == tag % h.p
}

pub fn demo() -> String {
    let mut o = String::new();
    let p = 11u64;
    let mut counts = [[0usize; 11]; 11];
    for a0 in 0..p {
        for a1 in 0..p {
            let h = PolyHash::new(p, vec![a0, a1]);
            counts[h.eval(2) as usize][h.eval(7) as usize] += 1;
        }
    }
    let all_one = counts.iter().all(|row| row.iter().all(|&c| c == 1));
    outln!(o, "GF({p}), k=2: across all {}^2 = {} functions, every pair (h(2), h(7)) hits once? {}", p, p * p, all_one);
    let mut rng = Rng::new(114);
    let big_p = 2_147_483_647u64; // Mersenne prime 2^31 - 1
    let key = PolyHash::random(big_p, 3, &mut rng);
    let pad = rng.next_u64() % big_p;
    let tag = cw_authenticate(&key, 42_195, pad);
    outln!(o, "Carter-Wegman tag (msg=42195, p={big_p}) = {tag}, valid={}", cw_verify(&key, 42_195, pad, tag));
    outln!(o, "tampered msg=42196 valid? {}", cw_verify(&key, 42_196, pad, tag));
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_pairwise_uniformity_over_gf11() {
        let p = 11u64;
        for x in 0..p {
            for y in (x + 1)..p {
                let mut joint = vec![vec![0usize; p as usize]; p as usize];
                for a0 in 0..p {
                    for a1 in 0..p {
                        let h = PolyHash::new(p, vec![a0, a1]);
                        joint[h.eval(x) as usize][h.eval(y) as usize] += 1;
                    }
                }
                assert!(joint.iter().all(|row| row.iter().all(|&c| c == 1)), "failed for x={x}, y={y}");
            }
        }
    }

    #[test]
    fn exact_3_universal_uniformity_over_gf5() {
        let p = 5u64;
        let (x, y, z) = (1, 2, 4);
        let mut joint = vec![0usize; (p * p * p) as usize];
        for a0 in 0..p {
            for a1 in 0..p {
                for a2 in 0..p {
                    let h = PolyHash::new(p, vec![a0, a1, a2]);
                    let idx = (h.eval(x) * p * p + h.eval(y) * p + h.eval(z)) as usize;
                    joint[idx] += 1;
                }
            }
        }
        assert!(joint.iter().all(|&c| c == 1));
    }

    #[test]
    fn carter_wegman_auth_and_forging_uniformity() {
        let p = 11u64;
        let (m1, m2) = (3, 8);
        let (pad1, pad2) = (5, 9);
        // Given observed (m1, tag1), any guessed tag2 for m2 matches with probability exactly 1/p^2 per secret guess, or 1/p overall.
        let mut rng = Rng::new(77);
        let key = PolyHash::random(p, 2, &mut rng);
        let t1 = cw_authenticate(&key, m1, pad1);
        assert!(cw_verify(&key, m1, pad1, t1));
        assert!(!cw_verify(&key, m1, pad1, (t1 + 1) % p));
        let mut consistent_t2 = [0usize; 11];
        let mut consistent_keys = 0usize;
        for a0 in 0..p {
            for a1 in 0..p {
                let cand = PolyHash::new(p, vec![a0, a1]);
                if cw_verify(&cand, m1, pad1, t1) {
                    consistent_keys += 1;
                    consistent_t2[cw_authenticate(&cand, m2, pad2) as usize] += 1;
                }
            }
        }
        assert_eq!(consistent_keys, p as usize);
        assert!(consistent_t2.iter().all(|&c| c == 1));
    }
}
