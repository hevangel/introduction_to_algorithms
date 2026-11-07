//! @title Algorithmic Consulting (Project Selection by Min-Cut)
//! @chapter Problem 26-3 (3e)
//! @editions 3,4
//! @summary Choose which jobs to accept when each job pays a profit but needs experts who must be hired at a fee: maximum profit = total profit − minimum cut.
//! @order 200
use super::ford_fulkerson::FlowNetwork;
use crate::outln;

/// Jobs with `profit[j]`, each needing the experts listed in `needs[j]`; hiring expert `e` costs `cost[e]` once, however many jobs use them.
/// Returns (maximum net profit, accepted jobs, hired experts). Source → job (capacity profit), job → expert (∞), expert → sink (capacity cost).
pub fn max_net_profit(profit: &[i64], needs: &[Vec<usize>], cost: &[i64]) -> (i64, Vec<usize>, Vec<usize>) {
    let (nj, ne) = (profit.len(), cost.len());
    let (s, t) = (nj + ne, nj + ne + 1);
    let mut g = FlowNetwork::new(nj + ne + 2);
    for j in 0..nj {
        g.add_edge(s, j, profit[j]);
        for &e in &needs[j] { g.add_edge(j, nj + e, i64::MAX / 4); }
    }
    for e in 0..ne { g.add_edge(nj + e, t, cost[e]); }
    let cut = g.edmonds_karp(s, t);
    let side = g.min_cut_source_side(s); // jobs on the source side are accepted, experts on the source side are hired
    let jobs: Vec<usize> = (0..nj).filter(|&j| side[j]).collect();
    let experts: Vec<usize> = (0..ne).filter(|&e| side[nj + e]).collect();
    (profit.iter().sum::<i64>() - cut, jobs, experts)
}

pub fn demo() -> String {
    let mut o = String::new();
    let profit = [100, 200, 150, 60];
    let needs = vec![vec![0, 1], vec![1, 2], vec![0, 2], vec![3]];
    let cost = [90, 120, 80, 100];
    let (v, jobs, experts) = max_net_profit(&profit, &needs, &cost);
    outln!(o, "job profits {:?}, experts' fees {:?}, needs {:?}", profit, cost, needs);
    outln!(o, "best net profit {v}: accept jobs {:?}, hire experts {:?}", jobs, experts);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn value(profit: &[i64], needs: &[Vec<usize>], cost: &[i64], jobs: &[usize]) -> i64 {
        let mut hired = vec![false; cost.len()];
        for &j in jobs { for &e in &needs[j] { hired[e] = true; } }
        jobs.iter().map(|&j| profit[j]).sum::<i64>() - (0..cost.len()).filter(|&e| hired[e]).map(|e| cost[e]).sum::<i64>()
    }
    #[test]
    fn equals_exhaustive_selection_of_jobs() {
        let mut r = Rng::new(462);
        for _ in 0..200 {
            let (nj, ne) = (r.range(1, 7), r.range(1, 6));
            let profit: Vec<i64> = (0..nj).map(|_| r.range(0, 50) as i64).collect();
            let cost: Vec<i64> = (0..ne).map(|_| r.range(0, 40) as i64).collect();
            let needs: Vec<Vec<usize>> = (0..nj).map(|_| (0..ne).filter(|_| r.f64() < 0.4).collect()).collect();
            let best = (0u32..1 << nj).map(|m| value(&profit, &needs, &cost, &(0..nj).filter(|&j| m >> j & 1 == 1).collect::<Vec<_>>())).max().unwrap();
            let (v, jobs, _) = max_net_profit(&profit, &needs, &cost);
            assert_eq!(v, best);
            assert_eq!(value(&profit, &needs, &cost, &jobs), best);
        }
    }
}
