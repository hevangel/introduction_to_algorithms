//! @title Project Selection & Maximum-Weight Closure
//! @chapter Problem 26-3 (1e–3e); Problem 24-3 (4e)
//! @editions 1,2,3,4
//! @summary Reduce profit maximization with prerequisite costs (algorithmic consulting / Space Shuttle payloads) to s-t minimum cut in O(VE²) or push-relabel time.
//! @order 195
use super::ford_fulkerson::FlowNetwork;
use crate::outln;

/// Maximum-weight closure on a directed graph with arbitrary signed vertex weights `weights[v]`
/// and prerequisites `prereqs` = `(u, v)` meaning selecting `u` requires selecting `v`.
/// Returns `(max_total_weight, chosen_vertex_indices)`.
pub fn max_weight_closure(weights: &[i64], prereqs: &[(usize, usize)]) -> (i64, Vec<usize>) {
    let n = weights.len();
    let s = n;
    let t = n + 1;
    let mut net = FlowNetwork::new(n + 2);
    let mut positive_sum = 0i64;
    for (i, &w) in weights.iter().enumerate() {
        if w > 0 {
            net.add_edge(s, i, w);
            positive_sum += w;
        } else if w < 0 {
            net.add_edge(i, t, -w);
        }
    }
    let inf = positive_sum + 1;
    for &(u, v) in prereqs {
        net.add_edge(u, v, inf);
    }
    let min_cut = net.edmonds_karp(s, t);
    let side = net.min_cut_source_side(s);
    let chosen: Vec<usize> = (0..n).filter(|&i| side[i]).collect();
    (positive_sum - min_cut, chosen)
}

/// CLRS Problem 26-3 / 24-3 formulation: jobs `rewards[j] >= 0`, instruments `costs[k] >= 0`,
/// and bipartite requirements `(job_j, instrument_k)`.
/// Returns `(net_profit, chosen_jobs, chosen_instruments)`.
pub fn project_selection(
    rewards: &[i64],
    costs: &[i64],
    requires: &[(usize, usize)],
) -> (i64, Vec<usize>, Vec<usize>) {
    let nj = rewards.len();
    let ni = costs.len();
    let mut weights = Vec::with_capacity(nj + ni);
    weights.extend(rewards.iter().copied());
    weights.extend(costs.iter().map(|&c| -c));
    let prereqs: Vec<(usize, usize)> = requires.iter().map(|&(j, k)| (j, nj + k)).collect();
    let (profit, chosen) = max_weight_closure(&weights, &prereqs);
    let chosen_jobs = chosen.iter().copied().filter(|&x| x < nj).collect();
    let chosen_inst = chosen.iter().filter(|&&x| x >= nj).map(|&x| x - nj).collect();
    (profit, chosen_jobs, chosen_inst)
}

pub fn demo() -> String {
    let mut o = String::new();
    // 3 jobs with rewards [100, 120, 70], 3 instruments with costs [60, 80, 50].
    let rewards = [100, 120, 70];
    let costs = [60, 80, 50];
    let requires = [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)];
    let (profit, jobs, inst) = project_selection(&rewards, &costs, &requires);
    outln!(o, "rewards={:?}, costs={:?}", rewards, costs);
    outln!(o, "best net profit = {profit}, chosen jobs = {:?}, instruments = {:?}", jobs, inst);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn closure_matches_2_to_n_brute_force() {
        let mut r = Rng::new(2603);
        for _ in 0..80 {
            let n = r.range(1, 12);
            let weights = r.vec(n, -25, 25);
            let m = r.range(0, 20);
            let prereqs: Vec<(usize, usize)> = (0..m)
                .map(|_| (r.range(0, n - 1), r.range(0, n - 1)))
                .collect();
            let (got_profit, chosen) = max_weight_closure(&weights, &prereqs);
            let mut in_s = vec![false; n];
            for &u in &chosen {
                in_s[u] = true;
            }
            for &(u, v) in &prereqs {
                if in_s[u] {
                    assert!(in_s[v], "chosen set is not closed under prerequisite ({u}, {v})");
                }
            }
            assert_eq!(chosen.iter().map(|&i| weights[i]).sum::<i64>(), got_profit);

            let mut brute = 0i64; // empty set has weight 0
            for mask in 0..(1usize << n) {
                let valid = prereqs.iter().all(|&(u, v)| ((mask >> u) & 1 == 0) || ((mask >> v) & 1 == 1));
                if valid {
                    let w: i64 = (0..n).filter(|&i| (mask >> i) & 1 == 1).map(|i| weights[i]).sum();
                    brute = brute.max(w);
                }
            }
            assert_eq!(got_profit, brute);
        }
    }

    #[test]
    fn space_shuttle_consulting_example() {
        let rewards = [100, 120, 70];
        let costs = [60, 80, 50];
        let requires = [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)];
        // All three: 290 - 190 = 100. Jobs {1,2} need inst {1,2}: 190 - 130 = 60. Only {0}: 100-140 = -40.
        let (p, jobs, inst) = project_selection(&rewards, &costs, &requires);
        assert_eq!(p, 100);
        assert_eq!(jobs, vec![0, 1, 2]);
        assert_eq!(inst, vec![0, 1, 2]);
    }
}
