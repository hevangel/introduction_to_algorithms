//! @title Inventory Planning
//! @chapter Problem 15-11 (3e) / 14-11 (4e)
//! @editions 3,4
//! @summary Meet monthly demand with regular capacity m, costly overtime and a holding cost h(j): DP over (month, inventory) in O(nD²).
//! @order 260
use crate::outln;

/// Minimum total cost and the production plan. `demand[i]` is the demand in month i, `capacity` = m machines per month at no extra cost,
/// `overtime` = c dollars for each machine beyond m, `hold[j]` = cost of holding j machines over a month end (hold[0] = 0, non-decreasing).
pub fn plan(demand: &[usize], capacity: usize, overtime: i64, hold: &[i64]) -> (i64, Vec<usize>) {
    let total: usize = demand.iter().sum();
    assert!(hold.len() > total && hold[0] == 0);
    let inf = i64::MAX / 4;
    let n = demand.len();
    // f[i][inv]: cheapest way to handle months 0..i and end month i−1 holding `inv` machines
    let mut f = vec![vec![inf; total + 1]; n + 1];
    let mut from = vec![vec![0usize; total + 1]; n + 1];
    f[0][0] = 0;
    for i in 1..=n {
        for prev in 0..=total {
            if f[i - 1][prev] >= inf { continue; }
            for inv in 0..=total {
                if prev + (0usize.max(0)) > total { continue; }
                // produced x = inv + demand − prev must be ≥ 0
                let x = inv as i64 + demand[i - 1] as i64 - prev as i64;
                if x < 0 { continue; }
                let c = f[i - 1][prev] + overtime * (x - capacity as i64).max(0) + hold[inv];
                if c < f[i][inv] { f[i][inv] = c; from[i][inv] = prev; }
            }
        }
    }
    let (best_inv, &best) = f[n].iter().enumerate().min_by_key(|&(_, &c)| c).unwrap();
    let (mut plan, mut inv) = (vec![0usize; n], best_inv);
    for i in (1..=n).rev() { let prev = from[i][inv]; plan[i - 1] = (inv + demand[i - 1] - prev) as usize; inv = prev; }
    (best, plan)
}

/// Cost of a given production plan (None if it ever sells machines it does not have).
pub fn evaluate(demand: &[usize], plan: &[usize], capacity: usize, overtime: i64, hold: &[i64]) -> Option<i64> {
    let (mut inv, mut cost) = (0i64, 0);
    for (d, &x) in demand.iter().zip(plan) {
        inv += x as i64 - *d as i64;
        if inv < 0 { return None; }
        cost += overtime * (x as i64 - capacity as i64).max(0) + hold[inv as usize];
    }
    Some(cost)
}

pub fn demo() -> String {
    let mut o = String::new();
    let demand = [3, 8, 2, 7, 4];
    let hold: Vec<i64> = (0..=24).map(|j| 2 * j as i64).collect();
    let (cost, p) = plan(&demand, 5, 10, &hold);
    outln!(o, "demand {:?}, regular capacity 5/month, overtime $10/machine, holding $2/machine/month", demand);
    outln!(o, "optimal plan {:?} costs ${cost}", p);
    outln!(o, "'produce exactly the demand' costs ${:?}", evaluate(&demand, &demand, 5, 10, &hold));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(demand: &[usize], cap: usize, c: i64, hold: &[i64], i: usize, inv: usize) -> i64 {
        if i == demand.len() { return 0; }
        let total: usize = demand.iter().sum();
        let mut best = i64::MAX / 4;
        for x in 0..=total {
            if inv + x < demand[i] { continue; }
            let ninv = inv + x - demand[i];
            if ninv > total { continue; }
            best = best.min(c * (x as i64 - cap as i64).max(0) + hold[ninv] + brute(demand, cap, c, hold, i + 1, ninv));
        }
        best
    }
    #[test]
    fn matches_exhaustive_search_and_plan_is_consistent() {
        let mut r = Rng::new(342);
        for _ in 0..80 {
            let n = r.range(1, 4);
            let demand: Vec<usize> = (0..n).map(|_| r.range(0, 4)).collect();
            let total: usize = demand.iter().sum();
            let mut hold = vec![0i64];
            for _ in 0..total { let last = *hold.last().unwrap(); hold.push(last + r.range(0, 4) as i64); }
            let (cap, c) = (r.range(0, 4), r.range(1, 9) as i64);
            let (cost, p) = plan(&demand, cap, c, &hold);
            assert_eq!(cost, brute(&demand, cap, c, &hold, 0, 0), "{demand:?}");
            assert_eq!(evaluate(&demand, &p, cap, c, &hold), Some(cost));
        }
    }
    #[test]
    fn free_inventory_levels_production() {
        let hold = vec![0i64; 21];
        let (cost, p) = plan(&[0, 0, 12], 4, 100, &hold);
        assert_eq!((cost, p.iter().sum::<usize>()), (0, 12)); // build ahead at regular capacity: no overtime
    }
}
