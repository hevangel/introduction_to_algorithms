//! @title Bonnie and Clyde (Dividing Loot)
//! @chapter Problem 34-2 (3e)
//! @editions 3,4
//! @summary Splitting loot evenly: easy for coins of two denominations (O(n)) and for powers of two (greedy), NP-complete for arbitrary checks (PARTITION), and still NP-complete when a difference of up to $100 is tolerated.
//! @order 90
use crate::outln;

/// (a) n coins of two denominations: `cx` coins worth `x` and `cy` coins worth `y`. Returns how many of each Bonnie takes for an exactly even split. O(n).
pub fn split_two_denominations(cx: usize, x: u64, cy: usize, y: u64) -> Option<(usize, usize)> {
    let total = cx as u64 * x + cy as u64 * y;
    if total % 2 != 0 { return None; }
    let half = total / 2;
    for a in 0..=cx {
        let Some(rest) = half.checked_sub(a as u64 * x) else { break };
        if rest % y == 0 && rest / y <= cy as u64 { return Some((a, (rest / y) as usize)); }
    }
    None
}

/// (b) coins that are all powers of two: sort in decreasing order and always give the next coin to the poorer partner; an exactly even split exists iff
/// this greedy ends with equal piles. Returns the two piles.
pub fn split_powers_of_two(coins: &[u64]) -> Option<(Vec<u64>, Vec<u64>)> {
    assert!(coins.iter().all(|c| c.is_power_of_two()));
    let mut sorted = coins.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    let (mut bonnie, mut clyde, mut sb, mut sc) = (vec![], vec![], 0u64, 0u64);
    for c in sorted { if sb <= sc { sb += c; bonnie.push(c); } else { sc += c; clyde.push(c); } }
    (sb == sc).then_some((bonnie, clyde))
}

/// (c) arbitrary checks: exactly even split — the PARTITION problem (NP-complete). Pseudo-polynomial DP over reachable sums, O(n·total).
/// Returns the indices of Bonnie's checks.
pub fn exact_split(values: &[u64]) -> Option<Vec<usize>> {
    let total: u64 = values.iter().sum();
    if total % 2 != 0 { return None; }
    let half = (total / 2) as usize;
    let mut first_use = vec![usize::MAX; half + 1]; // first_use[s] = index of the item that first made sum s reachable
    let mut reach = vec![false; half + 1];
    reach[0] = true;
    for (i, &v) in values.iter().enumerate() {
        for s in (v as usize..=half).rev() { if !reach[s] && reach[s - v as usize] { reach[s] = true; first_use[s] = i; } }
    }
    if !reach[half] { return None; }
    let (mut s, mut picked) = (half, vec![]);
    while s > 0 { let i = first_use[s]; picked.push(i); s -= values[i] as usize; }
    picked.sort();
    Some(picked)
}

/// (d) The smallest possible difference between the two shares. An even split within a tolerance of $T is possible iff this is ≤ T.
pub fn min_difference(values: &[u64]) -> u64 {
    let total: u64 = values.iter().sum();
    let mut reach = vec![false; total as usize + 1];
    reach[0] = true;
    for &v in values { for s in (v as usize..=total as usize).rev() { if reach[s - v as usize] { reach[s] = true; } } }
    (0..=total).filter(|&s| reach[s as usize]).map(|s| total.abs_diff(2 * s)).min().unwrap()
}

pub fn within_tolerance(values: &[u64], tolerance: u64) -> bool { min_difference(values) <= tolerance }

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "(a) 7 coins of $3 and 4 coins of $5: Bonnie takes {:?} (x-coins, y-coins)", split_two_denominations(7, 3, 4, 5));
    outln!(o, "(b) coins {:?}: {:?}", [8u64, 4, 4, 2, 1, 1], split_powers_of_two(&[8, 4, 4, 2, 1, 1]));
    let checks = [31u64, 24, 18, 17, 12, 9, 5];
    outln!(o, "(c) checks {:?}: Bonnie's checks {:?}", checks, exact_split(&checks));
    let scaled: Vec<u64> = checks.iter().map(|v| v * 101).collect();
    outln!(o, "(d) scaling every check by 101 turns \"within $100\" into \"exactly even\": {} == {}", within_tolerance(&scaled, 100), exact_split(&checks).is_some());
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute_min_diff(v: &[u64]) -> u64 { let t: u64 = v.iter().sum(); (0u32..1 << v.len()).map(|m| { let s: u64 = (0..v.len()).filter(|&i| m >> i & 1 == 1).map(|i| v[i]).sum(); t.abs_diff(2 * s) }).min().unwrap() }
    #[test]
    fn two_denominations_matches_brute_force() {
        let mut r = Rng::new(510);
        for _ in 0..300 {
            let (cx, cy, x, y) = (r.range(0, 8), r.range(0, 8), r.range(1, 9) as u64, r.range(1, 9) as u64);
            let brute = (0..=cx).flat_map(|a| (0..=cy).map(move |b| (a, b))).find(|&(a, b)| 2 * (a as u64 * x + b as u64 * y) == cx as u64 * x + cy as u64 * y);
            assert_eq!(split_two_denominations(cx, x, cy, y).is_some(), brute.is_some());
            if let Some((a, b)) = split_two_denominations(cx, x, cy, y) { assert!(a <= cx && b <= cy && 2 * (a as u64 * x + b as u64 * y) == cx as u64 * x + cy as u64 * y); }
        }
    }
    #[test]
    fn powers_of_two_greedy_is_exactly_right() {
        let mut r = Rng::new(511);
        for _ in 0..500 {
            let coins: Vec<u64> = (0..r.range(0, 11)).map(|_| 1u64 << r.range(0, 5)).collect();
            let brute = brute_min_diff(&coins) == 0;
            match split_powers_of_two(&coins) {
                Some((b, c)) => { assert!(brute); assert_eq!(b.iter().sum::<u64>(), c.iter().sum::<u64>()); assert_eq!(b.len() + c.len(), coins.len()); }
                None => assert!(!brute, "{coins:?}"),
            }
        }
    }
    #[test]
    fn checks_exact_and_tolerance_versions() {
        let mut r = Rng::new(512);
        for _ in 0..300 {
            let v: Vec<u64> = (0..r.range(0, 12)).map(|_| r.range(1, 40) as u64).collect();
            assert_eq!(min_difference(&v), brute_min_diff(&v));
            match exact_split(&v) {
                Some(idx) => { assert_eq!(idx.iter().map(|&i| v[i]).sum::<u64>() * 2, v.iter().sum::<u64>()); }
                None => assert!(brute_min_diff(&v) > 0),
            }
            // the NP-hardness reduction of part (d): scaling by 101 makes any non-zero difference exceed $100
            let scaled: Vec<u64> = v.iter().map(|x| x * 101).collect();
            assert_eq!(within_tolerance(&scaled, 100), brute_min_diff(&v) == 0);
        }
    }
}
