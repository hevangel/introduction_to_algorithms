//! @title Printing Neatly
//! @chapter Problem 15-4 (2e–3e), 14-4 (4e)
//! @editions 2,3,4
//! @summary Break a paragraph into lines minimising the sum of cubes of trailing blanks: DP over line breaks.
//! @order 110
use crate::outln;

/// `lens[i]` is the length of word i; lines hold at most `m` characters, words separated by one space.
/// Returns (minimum cost, the number of words on each line). The last line costs nothing.
pub fn print_neatly(lens: &[usize], m: usize) -> Option<(u64, Vec<usize>)> {
    let n = lens.len();
    if lens.iter().any(|&l| l > m) { return None; }
    let mut c = vec![u64::MAX; n + 1]; // c[j] = best cost of arranging the first j words
    let mut brk = vec![0usize; n + 1];
    c[0] = 0;
    for j in 1..=n {
        let mut width = 0usize;
        for i in (0..j).rev() {
            width += lens[i] + if i + 1 < j { 1 } else { 0 };
            if width > m { break; }
            let extra = (m - width) as u64;
            let cost = if j == n { 0 } else { extra.pow(3) };
            if c[i] != u64::MAX && c[i] + cost < c[j] { c[j] = c[i] + cost; brk[j] = i; }
        }
    }
    let mut lines = vec![];
    let mut j = n;
    while j > 0 { lines.push(j - brk[j]); j = brk[j]; }
    lines.reverse();
    Some((c[n], lines))
}

/// Typeset words with the computed breaks.
pub fn layout(words: &[&str], m: usize) -> Option<Vec<String>> {
    let lens: Vec<usize> = words.iter().map(|w| w.len()).collect();
    let (_, lines) = print_neatly(&lens, m)?;
    let mut out = vec![];
    let mut i = 0;
    for k in lines { out.push(words[i..i + k].join(" ")); i += k; }
    Some(out)
}

pub fn demo() -> String {
    let mut o = String::new();
    let words: Vec<&str> = "Dynamic programming solves problems by combining the solutions to overlapping subproblems".split(' ').collect();
    for m in [20, 30] {
        let lens: Vec<usize> = words.iter().map(|w| w.len()).collect();
        let (cost, _) = print_neatly(&lens, m).unwrap();
        outln!(o, "line width {m}: total cost {cost}");
        for l in layout(&words, m).unwrap() { outln!(o, "  |{:<width$}|", l, width = m); }
    }
    outln!(o, "a word longer than the line cannot be placed: {:?}", print_neatly(&[10], 5));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(lens: &[usize], m: usize) -> Option<u64> {
        let n = lens.len();
        let mut best: Option<u64> = None;
        for mask in 0..1u32 << n.saturating_sub(1) {
            let (mut cost, mut ok, mut width, mut count) = (0u64, true, 0usize, 0usize);
            for i in 0..n {
                width += lens[i] + if count > 0 { 1 } else { 0 };
                count += 1;
                if width > m { ok = false; break; }
                let end = i == n - 1 || mask >> i & 1 == 1;
                if end { if i != n - 1 { cost += ((m - width) as u64).pow(3); } width = 0; count = 0; }
            }
            if ok && best.is_none_or(|b| cost < b) { best = Some(cost); }
        }
        best
    }
    #[test]
    fn matches_exhaustive_search() {
        let mut r = Rng::new(109);
        for _ in 0..120 {
            let n = r.range(1, 9);
            let lens: Vec<usize> = (0..n).map(|_| r.range(1, 6)).collect();
            let m = r.range(6, 14);
            assert_eq!(print_neatly(&lens, m).map(|x| x.0), brute(&lens, m), "{lens:?} m={m}");
        }
    }
    #[test]
    fn layout_respects_width() {
        let words = ["a", "bb", "ccc", "dddd", "eeeee", "ff", "g"];
        for l in layout(&words, 8).unwrap() { assert!(l.len() <= 8); }
        assert_eq!(print_neatly(&[1, 1, 1], 10).unwrap().0, 0); // single (last) line is free
    }
}
