//! @title Karatsuba Multiplication
//! @chapter Problem 30-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Multiply two polynomials (or big numbers) with three half-size products instead of four: Θ(n^lg 3) = Θ(n^1.585), the divide-and-conquer step before the FFT.
//! @order 5
use crate::outln;

/// Product of two equal-length coefficient slices (length a power of two). `mults` counts scalar multiplications.
fn kara(a: &[i64], b: &[i64], mults: &mut usize) -> Vec<i64> {
    let n = a.len();
    if n == 1 { *mults += 1; return vec![a[0] * b[0]]; }
    let h = n / 2;
    let (a0, a1) = a.split_at(h); // A(x) = A0(x) + x^h · A1(x)
    let (b0, b1) = b.split_at(h);
    let z0 = kara(a0, b0, mults);
    let z2 = kara(a1, b1, mults);
    let sa: Vec<i64> = a0.iter().zip(a1).map(|(x, y)| x + y).collect();
    let sb: Vec<i64> = b0.iter().zip(b1).map(|(x, y)| x + y).collect();
    let z1 = kara(&sa, &sb, mults); // (A0 + A1)(B0 + B1) = A0B0 + A1B1 + (A0B1 + A1B0)
    let mut r = vec![0; 2 * n - 1];
    for i in 0..z0.len() {
        r[i] += z0[i];
        r[i + h] += z1[i] - z0[i] - z2[i]; // the middle term A0B1 + A1B0
        r[i + 2 * h] += z2[i];
    }
    r
}

/// Karatsuba polynomial multiplication; returns the product and the number of scalar multiplications used.
pub fn karatsuba(a: &[i64], b: &[i64]) -> (Vec<i64>, usize) {
    if a.is_empty() || b.is_empty() { return (vec![], 0); }
    let n = a.len().max(b.len()).next_power_of_two();
    let (mut pa, mut pb) = (a.to_vec(), b.to_vec());
    pa.resize(n, 0);
    pb.resize(n, 0);
    let mut mults = 0;
    let mut r = kara(&pa, &pb, &mut mults);
    r.truncate(a.len() + b.len() - 1);
    (r, mults)
}

/// Multiply two non-negative decimal numbers given as strings using Karatsuba on their digit arrays.
pub fn multiply_decimal(x: &str, y: &str) -> String {
    let digits = |s: &str| -> Vec<i64> { s.bytes().rev().map(|c| (c - b'0') as i64).collect() };
    let (mut prod, _) = karatsuba(&digits(x), &digits(y));
    let mut carry = 0;
    for d in prod.iter_mut() { *d += carry; carry = *d / 10; *d %= 10; }
    while carry > 0 { prod.push(carry % 10); carry /= 10; }
    while prod.len() > 1 && *prod.last().unwrap() == 0 { prod.pop(); }
    prod.iter().rev().map(|&d| (b'0' + d as u8) as char).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let (p, m) = karatsuba(&[1, 2, 3, 4], &[5, 6, 7, 8]);
    outln!(o, "(1+2x+3x²+4x³)(5+6x+7x²+8x³) = {:?} using {m} multiplications (schoolbook: 16)", p);
    for k in [4u32, 8, 12] { let n = 1usize << k; outln!(o, "n = {n:>5}: Karatsuba {:>9} multiplications, schoolbook {:>9} (ratio {:.1})", 3usize.pow(k), n * n, (n * n) as f64 / 3f64.powi(k as i32)); }
    outln!(o, "123456789123456789 × 987654321987654321 = {}", multiply_decimal("123456789123456789", "987654321987654321"));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn naive(a: &[i64], b: &[i64]) -> Vec<i64> { let mut r = vec![0; a.len() + b.len() - 1]; for i in 0..a.len() { for j in 0..b.len() { r[i + j] += a[i] * b[j]; } } r }
    #[test]
    fn matches_schoolbook_for_all_shapes() {
        let mut rng = Rng::new(448);
        for _ in 0..200 {
            let (la, lb) = (rng.range(1, 40), rng.range(1, 40));
            let (a, b) = (rng.vec(la, -50, 50), rng.vec(lb, -50, 50));
            assert_eq!(karatsuba(&a, &b).0, naive(&a, &b));
        }
    }
    #[test]
    fn uses_exactly_three_to_the_k_multiplications() {
        for k in 0..9u32 { let n = 1usize << k; assert_eq!(karatsuba(&vec![1; n], &vec![2; n]).1, 3usize.pow(k)); }
    }
    #[test]
    fn decimal_multiplication_matches_u128() {
        let mut rng = Rng::new(449);
        for _ in 0..200 {
            let (x, y) = (rng.next_u64() >> 4, rng.next_u64() >> 4);
            assert_eq!(multiply_decimal(&x.to_string(), &y.to_string()), (x as u128 * y as u128).to_string());
        }
        assert_eq!(multiply_decimal("0", "12345"), "0");
    }
}
