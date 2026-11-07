//! @title Matching Water Jugs
//! @chapter Problem 8-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Match n red jugs with n blue jugs when only red-vs-blue comparisons are allowed: a quicksort-style randomized algorithm needs Θ(n lg n) expected comparisons, and Ω(n lg n) are required.
//! @order 215
use crate::common::Rng;
use crate::outln;

/// Capacities of red and blue jugs; every red jug has exactly one blue jug of the same capacity.
/// Returns the pairs (red index, blue index) and the number of red-vs-blue comparisons used (no red-red or blue-blue comparison is ever made).
pub fn match_jugs(red: &[u32], blue: &[u32], rng: &mut Rng) -> (Vec<(usize, usize)>, usize) {
    let mut cmp = 0;
    let mut out = vec![];
    solve((0..red.len()).collect(), (0..blue.len()).collect(), red, blue, rng, &mut cmp, &mut out);
    (out, cmp)
}

fn solve(r: Vec<usize>, b: Vec<usize>, red: &[u32], blue: &[u32], rng: &mut Rng, cmp: &mut usize, out: &mut Vec<(usize, usize)>) {
    if r.is_empty() { return; }
    let pivot_red = r[rng.range(0, r.len() - 1)];
    // compare the red pivot with every blue jug: smaller / equal / larger
    let (mut bs, mut bl, mut bm) = (vec![], vec![], None);
    for &j in &b {
        *cmp += 1;
        match red[pivot_red].cmp(&blue[j]) { std::cmp::Ordering::Greater => bs.push(j), std::cmp::Ordering::Less => bl.push(j), std::cmp::Ordering::Equal => bm = Some(j) }
    }
    let m = bm.expect("every red jug has a matching blue jug");
    out.push((pivot_red, m));
    // use the matching blue jug to split the red jugs
    let (mut rs, mut rl) = (vec![], vec![]);
    for &i in &r {
        if i == pivot_red { continue; }
        *cmp += 1;
        if red[i] < blue[m] { rs.push(i); } else { rl.push(i); }
    }
    solve(rs, bs, red, blue, rng, cmp, out);
    solve(rl, bl, red, blue, rng, cmp, out);
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut rng = Rng::new(7);
    let red = [5u32, 2, 9, 4, 7];
    let blue = [7u32, 4, 2, 5, 9];
    let (pairs, c) = match_jugs(&red, &blue, &mut rng);
    outln!(o, "red {:?}, blue {:?}", red, blue);
    outln!(o, "pairs (red idx, blue idx): {:?}, {c} comparisons", pairs);
    for n in [100usize, 1000, 10000] {
        let mut caps: Vec<u32> = (0..n as u32).collect();
        rng.shuffle(&mut caps);
        let mut other = caps.clone();
        rng.shuffle(&mut other);
        let (_, c) = match_jugs(&caps, &other, &mut rng);
        outln!(o, "n = {n:>6}: {c} comparisons = {:.2} · n lg n", c as f64 / (n as f64 * (n as f64).log2()));
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_pair_has_equal_capacity_and_comparisons_are_n_log_n() {
        let mut rng = Rng::new(440);
        for n in [1usize, 2, 3, 10, 100, 1000] {
            let mut red: Vec<u32> = (0..n as u32).map(|x| x * 3 + 1).collect();
            let mut blue = red.clone();
            rng.shuffle(&mut red);
            rng.shuffle(&mut blue);
            let (pairs, c) = match_jugs(&red, &blue, &mut rng);
            assert_eq!(pairs.len(), n);
            let (mut ur, mut ub) = (vec![false; n], vec![false; n]);
            for &(i, j) in &pairs { assert_eq!(red[i], blue[j]); ur[i] = true; ub[j] = true; }
            assert!(ur.iter().all(|&x| x) && ub.iter().all(|&x| x));
            assert!(c as f64 <= 6.0 * n as f64 * (n as f64).log2() + 4.0, "n={n} comparisons {c}");
        }
    }
}
