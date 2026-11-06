//! @title Edit Distance
//! @chapter Problem 15-5 (2e–3e), 14-5 (4e)
//! @editions 2,3,4
//! @summary Transform x into y with copy / replace / delete / insert / twiddle / kill at minimum cost.
//! @order 70
use crate::outln;

#[derive(Debug, Clone, Copy)]
pub struct Costs { pub copy: i64, pub replace: i64, pub delete: i64, pub insert: i64, pub twiddle: i64, pub kill: i64 }

impl Costs {
    /// Classic Levenshtein costs (no twiddle, no kill): copy 0, others 1.
    pub fn levenshtein() -> Self { Costs { copy: 0, replace: 1, delete: 1, insert: 1, twiddle: i64::MAX / 4, kill: i64::MAX / 4 } }
    /// Costs of Problem 15-5's DNA-alignment-style example.
    pub fn with_twiddle_and_kill() -> Self { Costs { copy: 0, replace: 2, delete: 2, insert: 2, twiddle: 1, kill: 3 } }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op { Copy(char), Replace(char, char), Delete(char), Insert(char), Twiddle(char, char), Kill(usize) }

/// Minimum cost to transform `x` into `y`, together with an optimal operation sequence.
pub fn edit_distance(x: &str, y: &str, c: &Costs) -> (i64, Vec<Op>) {
    let (xs, ys): (Vec<char>, Vec<char>) = (x.chars().collect(), y.chars().collect());
    let (m, n) = (xs.len(), ys.len());
    const INF: i64 = i64::MAX / 2;
    // dp[i][j]: cost of turning xs[..i] into ys[..j]; choice[i][j]: op that achieved it
    let mut dp = vec![vec![INF; n + 1]; m + 1];
    let mut choice = vec![vec![0u8; n + 1]; m + 1];
    dp[0][0] = 0;
    for i in 0..=m {
        for j in 0..=n {
            if i == 0 && j == 0 { continue; }
            let mut best = INF;
            let mut ch = 0u8;
            let mut try_ = |v: i64, code: u8| if v < best { best = v; ch = code; };
            if i > 0 && j > 0 {
                if xs[i - 1] == ys[j - 1] { try_(dp[i - 1][j - 1] + c.copy, 1); }
                try_(dp[i - 1][j - 1] + c.replace, 2);
            }
            if i > 0 { try_(dp[i - 1][j] + c.delete, 3); }
            if j > 0 { try_(dp[i][j - 1] + c.insert, 4); }
            if i > 1 && j > 1 && xs[i - 1] == ys[j - 2] && xs[i - 2] == ys[j - 1] { try_(dp[i - 2][j - 2] + c.twiddle, 5); }
            dp[i][j] = best;
            choice[i][j] = ch;
        }
    }
    // kill: delete the remainder xs[i..] in one operation, only as the very last step
    let (mut best, mut kill_at) = (dp[m][n], m);
    for i in 0..m {
        if dp[i][n] + c.kill < best { best = dp[i][n] + c.kill; kill_at = i; }
    }
    let mut ops = vec![];
    if kill_at < m { ops.push(Op::Kill(m - kill_at)); }
    let (mut i, mut j) = (kill_at, n);
    while i > 0 || j > 0 {
        match choice[i][j] {
            1 => { ops.push(Op::Copy(xs[i - 1])); i -= 1; j -= 1; }
            2 => { ops.push(Op::Replace(xs[i - 1], ys[j - 1])); i -= 1; j -= 1; }
            3 => { ops.push(Op::Delete(xs[i - 1])); i -= 1; }
            4 => { ops.push(Op::Insert(ys[j - 1])); j -= 1; }
            5 => { ops.push(Op::Twiddle(xs[i - 2], xs[i - 1])); i -= 2; j -= 2; }
            _ => unreachable!(),
        }
    }
    ops.reverse();
    (best, ops)
}

/// Apply an operation sequence to `x` and return the resulting string (for verification).
pub fn apply(x: &str, ops: &[Op]) -> String {
    let xs: Vec<char> = x.chars().collect();
    let (mut i, mut out) = (0, String::new());
    for op in ops {
        match op {
            Op::Copy(c) => { assert_eq!(xs[i], *c); out.push(*c); i += 1; }
            Op::Replace(a, b) => { assert_eq!(xs[i], *a); out.push(*b); i += 1; }
            Op::Delete(a) => { assert_eq!(xs[i], *a); i += 1; }
            Op::Insert(b) => out.push(*b),
            Op::Twiddle(a, b) => { assert_eq!((xs[i], xs[i + 1]), (*a, *b)); out.push(*b); out.push(*a); i += 2; }
            Op::Kill(k) => { i += k; }
        }
    }
    assert_eq!(i, xs.len());
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let (d, ops) = edit_distance("kitten", "sitting", &Costs::levenshtein());
    outln!(o, "Levenshtein(kitten → sitting) = {d}");
    outln!(o, "  ops: {:?}", ops);
    let c = Costs::with_twiddle_and_kill();
    let (d, ops) = edit_distance("algorithm", "altruistic", &c);
    outln!(o, "problem 15-5 costs (algorithm → altruistic) = {d}");
    outln!(o, "  ops: {:?}", ops);
    let (d, ops) = edit_distance("recieve", "receive", &c);
    outln!(o, "twiddle fixes a typo: recieve → receive costs {d}: {:?}", ops);
    let (d, ops) = edit_distance("abcdefgh", "abc", &c);
    outln!(o, "kill the tail: abcdefgh → abc costs {d}: {:?}", ops);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn lev(a: &str, b: &str) -> i64 {
        let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
        let mut d = vec![vec![0i64; b.len() + 1]; a.len() + 1];
        for i in 0..=a.len() { d[i][0] = i as i64; }
        for j in 0..=b.len() { d[0][j] = j as i64; }
        for i in 1..=a.len() { for j in 1..=b.len() {
            d[i][j] = (d[i - 1][j] + 1).min(d[i][j - 1] + 1).min(d[i - 1][j - 1] + (a[i - 1] != b[j - 1]) as i64);
        } }
        d[a.len()][b.len()]
    }
    #[test]
    fn classic_values() {
        assert_eq!(edit_distance("kitten", "sitting", &Costs::levenshtein()).0, 3);
        assert_eq!(edit_distance("", "abc", &Costs::levenshtein()).0, 3);
        assert_eq!(edit_distance("same", "same", &Costs::levenshtein()).0, 0);
    }
    #[test]
    fn matches_reference_levenshtein_and_scripts_replay() {
        let mut r = Rng::new(105);
        for _ in 0..100 {
            let mk = |r: &mut Rng| -> String { (0..r.range(0, 8)).map(|_| (b'a' + r.range(0, 3) as u8) as char).collect() };
            let (x, y) = (mk(&mut r), mk(&mut r));
            let (d, ops) = edit_distance(&x, &y, &Costs::levenshtein());
            assert_eq!(d, lev(&x, &y));
            assert_eq!(apply(&x, &ops), y);
            let c = Costs::with_twiddle_and_kill();
            let (d2, ops2) = edit_distance(&x, &y, &c);
            assert_eq!(apply(&x, &ops2), y);
            assert!(d2 <= 2 * lev(&x, &y));
        }
    }
    #[test]
    fn twiddle_and_kill_are_used() {
        let c = Costs::with_twiddle_and_kill();
        let (d, ops) = edit_distance("ab", "ba", &c);
        assert_eq!(d, 1);
        assert_eq!(ops, vec![Op::Twiddle('a', 'b')]);
        let (_, ops) = edit_distance("abcdefgh", "abc", &c);
        assert!(matches!(ops.last(), Some(Op::Kill(5))));
    }
}
