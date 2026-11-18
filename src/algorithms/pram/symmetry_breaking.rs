//! @title PRAM Work-Efficient Prefix & Symmetry Breaking
//! @chapter 30.4–30.5 (1e)
//! @editions 1
//! @summary Deterministic coin-tossing 6-coloring in O(lg* n) rounds and O(1) reduction to 3-coloring plus randomized EREW list prefix.
//! @order 60
use crate::common::Rng;
use crate::outln;

/// SIX-COLOR(L): deterministic EREW PRAM 6-coloring of a collection of directed paths `next[u]`.
/// Tail nodes (`next[u] == None`) serve as roots with fixed color 0; non-tails start at `u + 1`.
/// Returns `(colors_in_0..6, rounds_taken)`.
pub fn six_color_list(next: &[Option<usize>]) -> (Vec<usize>, usize) {
    let n = next.len();
    if n == 0 {
        return (vec![], 0);
    }
    let mut c: Vec<usize> = (0..n)
        .map(|u| if next[u].is_none() { 0 } else { u + 1 })
        .collect();
    let mut rounds = 0;
    while c.iter().any(|&col| col >= 6) {
        let old = c.clone();
        for u in 0..n {
            if let Some(v) = next[u] {
                let diff = old[u] ^ old[v];
                let k = diff.trailing_zeros() as usize;
                let bit = (old[u] >> k) & 1;
                c[u] = 2 * k + bit;
            }
        }
        rounds += 1;
    }
    (c, rounds)
}

/// Reduce a valid 6-coloring of directed paths to a valid 3-coloring in 3 parallel color-elimination rounds.
pub fn three_color_from_six(next: &[Option<usize>], mut c: Vec<usize>) -> Vec<usize> {
    let n = next.len();
    let mut prev = vec![None; n];
    for u in 0..n {
        if let Some(v) = next[u] {
            prev[v] = Some(u);
        }
    }
    for target in [5usize, 4, 3] {
        let old = c.clone();
        for u in 0..n {
            if old[u] == target {
                let p_col = prev[u].map(|p| old[p]);
                let n_col = next[u].map(|v| old[v]);
                c[u] = (0..3)
                    .find(|&cand| Some(cand) != p_col && Some(cand) != n_col)
                    .unwrap();
            }
        }
    }
    c
}

/// Greedily extract a maximal independent set (MIS) from a valid 3-coloring in 3 rounds.
pub fn mis_from_three_coloring(next: &[Option<usize>], c: &[usize]) -> Vec<bool> {
    let n = next.len();
    let mut prev = vec![None; n];
    for u in 0..n {
        if let Some(v) = next[u] {
            prev[v] = Some(u);
        }
    }
    let mut in_mis = vec![false; n];
    for color in 0..3 {
        for u in 0..n {
            if c[u] == color {
                let p_in = prev[u].is_some_and(|p| in_mis[p]);
                let n_in = next[u].is_some_and(|v| in_mis[v]);
                if !p_in && !n_in {
                    in_mis[u] = true;
                }
            }
        }
    }
    in_mis
}

/// RANDOMIZED-LIST-PREFIX: work-efficient EREW prefix sums by random-mate independent-set contraction
/// until ≤ O(n / lg n) active nodes remain, pointer jumping on the compressed list, and splicing back.
pub fn randomized_list_prefix(next: &[Option<usize>], vals: &[i64], seed: u64) -> Vec<i64> {
    let n = next.len();
    assert_eq!(n, vals.len());
    if n == 0 {
        return vec![];
    }
    let mut rng = Rng::new(seed);
    let mut cur_next = next.to_vec();
    let mut cur_val = vals.to_vec();
    let mut active = vec![true; n];
    let mut active_count = n;
    // Stack of `(spliced_out_node, predecessor_holding_its_value, original_next_after_node)`
    let mut splice_log: Vec<Vec<(usize, Option<usize>, usize)>> = Vec::new();
    let threshold = (n / (n.ilog2().max(1) as usize)).max(2);

    while active_count > threshold {
        // Compute reverse pointers for active nodes so we can splice a contracted node out in O(1).
        let mut prev = vec![None; n];
        for u in 0..n {
            if active[u] {
                if let Some(v) = cur_next[u] {
                    prev[v] = Some(u);
                }
            }
        }
        // Flip random coins: node u with coin=0 (head) whose active successor v has coin=1 (tail) contracts v into u.
        let coin: Vec<bool> = (0..n).map(|_| (rng.next_u64() & 1) == 0).collect();
        let mut step_log = Vec::new();
        for u in 0..n {
            if !active[u] || !coin[u] {
                continue;
            }
            if let Some(v) = cur_next[u] {
                if active[v] && !coin[v] && cur_next[v].is_some() {
                    // Fold v into u and splice v out: u keeps sum, v records that its prefix = prefix(u_old) + val(v)
                    // Wait: for suffix/prefix from head-to-tail: forward prefix[v] = prefix[u] + val[v] after u completes.
                    step_log.push((v, prev[u], u));
                }
            }
        }
        if step_log.is_empty() {
            // Fall back cleanly if coins failed to pair (still shrinks quickly in expectation).
            continue;
        }
        let mut round_records = Vec::new();
        for &(v, _pu, u) in &step_log {
            // Leave cur_val[v] intact as v's original value for splicing back.
            let after_v = cur_next[v];
            cur_val[u] += cur_val[v];
            cur_next[u] = after_v;
            active[v] = false;
            active_count -= 1;
            round_records.push((v, Some(u), v));
        }
        splice_log.push(round_records);
    }

    // Sequential/pointer-jumping prefix on the reduced active sublist (forward head-to-tail sum).
    let mut rev_prev = vec![None; n];
    for u in 0..n {
        if active[u] {
            if let Some(v) = cur_next[u] {
                rev_prev[v] = Some(u);
            }
        }
    }
    let mut prefix = vec![0i64; n];
    for start in 0..n {
        if active[start] && rev_prev[start].is_none() {
            let mut acc = 0i64;
            let mut cur = Some(start);
            while let Some(u) = cur {
                acc += cur_val[u];
                prefix[u] = acc;
                cur = cur_next[u];
            }
        }
    }

    // Splice contracted nodes back in reverse contraction order.
    // Wait: during contraction we folded `cur_val[u] += cur_val[v]`, so `prefix[u]` already equals
    // the full prefix sum THROUGH `v`! Thus `prefix[v] = prefix[u]`, and `u`'s pre-v prefix was `prefix[u] - orig_v_val`!
    for round in splice_log.into_iter().rev() {
        for (v, u_opt, _) in round {
            let u = u_opt.unwrap();
            let total_through_v = prefix[u];
            prefix[v] = total_through_v;
            prefix[u] = total_through_v - cur_val[v];
        }
    }
    prefix
}

pub fn demo() -> String {
    let mut o = String::new();
    let n = 16;
    let next: Vec<Option<usize>> = (0..n).map(|i| if i + 1 < n { Some(i + 1) } else { None }).collect();
    let (c6, rounds) = six_color_list(&next);
    let c3 = three_color_from_six(&next, c6.clone());
    let mis = mis_from_three_coloring(&next, &c3);
    outln!(o, "n={n} path 6-colored in {rounds} deterministic coin-tossing rounds:");
    outln!(o, "  c6  = {c6:?}");
    outln!(o, "  c3  = {c3:?}");
    outln!(o, "  MIS = {:?}", mis.iter().enumerate().filter(|&(_, &b)| b).map(|(i, _)| i).collect::<Vec<_>>());
    let vals: Vec<i64> = (1..=n as i64).collect();
    let pref = randomized_list_prefix(&next, &vals, 304);
    outln!(o, "randomized list prefix(1..={n}): {pref:?}");
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_and_three_coloring_are_proper_and_lg_star_fast() {
        let mut r = Rng::new(305);
        for n in [1, 2, 7, 32, 200, 500] {
            let mut order: Vec<usize> = (0..n).collect();
            r.shuffle(&mut order);
            let mut next = vec![None; n];
            for w in order.windows(2) {
                next[w[0]] = Some(w[1]);
            }
            let (c6, rounds) = six_color_list(&next);
            assert!(rounds <= 6, "O(lg* n) rounds took {rounds} for n={n}");
            assert!(c6.iter().all(|&c| c < 6));
            for u in 0..n {
                if let Some(v) = next[u] {
                    assert_ne!(c6[u], c6[v]);
                }
            }
            let c3 = three_color_from_six(&next, c6);
            assert!(c3.iter().all(|&c| c < 3));
            for u in 0..n {
                if let Some(v) = next[u] {
                    assert_ne!(c3[u], c3[v]);
                }
            }
            let mis = mis_from_three_coloring(&next, &c3);
            for u in 0..n {
                if let Some(v) = next[u] {
                    assert!(!(mis[u] && mis[v]));
                }
            }
        }
    }

    #[test]
    fn randomized_list_prefix_matches_sequential_walk() {
        let mut r = Rng::new(306);
        for n in [1, 5, 19, 64, 150] {
            let mut order: Vec<usize> = (0..n).collect();
            r.shuffle(&mut order);
            let mut next = vec![None; n];
            for w in order.windows(2) {
                next[w[0]] = Some(w[1]);
            }
            let vals = r.vec(n, -20, 20);
            let got = randomized_list_prefix(&next, &vals, 77 + n as u64);
            let mut expected = vec![0i64; n];
            let mut acc = 0;
            for &u in &order {
                acc += vals[u];
                expected[u] = acc;
            }
            assert_eq!(got, expected);
        }
    }
}
