//! @title Breaking a String
//! @chapter Problem 15-9 (3e) / 14-9 (4e)
//! @editions 3,4
//! @summary Cut a string of length n at m given positions; each cut costs the length of the piece being cut. Interval DP in O(m³).
//! @order 255
use crate::outln;

/// Minimum total cost and an order of cuts. `cuts` are positions in 1..n.
pub fn break_string(n: usize, cuts: &[usize]) -> (usize, Vec<usize>) {
    let mut b = vec![0];
    let mut c = cuts.to_vec();
    c.sort();
    b.extend(&c);
    b.push(n);
    let m = b.len();
    let mut cost = vec![vec![0usize; m]; m];
    let mut split = vec![vec![0usize; m]; m];
    for len in 2..m {
        for i in 0..m - len {
            let j = i + len;
            cost[i][j] = usize::MAX;
            for k in i + 1..j {
                let q = cost[i][k] + cost[k][j] + (b[j] - b[i]); // cutting piece [b_i, b_j] at b_k costs its length
                if q < cost[i][j] { cost[i][j] = q; split[i][j] = k; }
            }
        }
    }
    let mut order = vec![];
    fn walk(i: usize, j: usize, split: &[Vec<usize>], b: &[usize], order: &mut Vec<usize>) {
        if j - i < 2 { return; }
        let k = split[i][j];
        order.push(b[k]);
        walk(i, k, split, b, order);
        walk(k, j, split, b, order);
    }
    walk(0, m - 1, &split, &b, &mut order);
    (cost[0][m - 1], order)
}

/// Cost of performing the cuts in the given order on a string of length n.
pub fn cost_of_order(n: usize, order: &[usize]) -> usize {
    let mut pieces = vec![(0, n)];
    let mut total = 0;
    for &c in order {
        let i = pieces.iter().position(|&(l, r)| l < c && c < r).expect("cut position invalid");
        let (l, r) = pieces.remove(i);
        total += r - l;
        pieces.push((l, c));
        pieces.push((c, r));
    }
    total
}

pub fn demo() -> String {
    let mut o = String::new();
    let (cost, order) = break_string(20, &[2, 8, 10]);
    outln!(o, "length-20 string, cuts at 2, 8, 10: optimal cost {cost} with order {:?}", order);
    outln!(o, "cutting left to right instead costs {}", cost_of_order(20, &[2, 8, 10]));
    outln!(o, "cutting right to left costs {}", cost_of_order(20, &[10, 8, 2]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn perms(v: &[usize]) -> Vec<Vec<usize>> {
        if v.len() <= 1 { return vec![v.to_vec()]; }
        let mut out = vec![];
        for i in 0..v.len() { let mut rest = v.to_vec(); let x = rest.remove(i); for mut p in perms(&rest) { p.insert(0, x); out.push(p); } }
        out
    }
    #[test]
    fn book_example() { assert_eq!(break_string(20, &[2, 8, 10]).0, 38); }
    #[test]
    fn matches_all_cut_orders() {
        let mut r = Rng::new(341);
        for _ in 0..60 {
            let n = r.range(2, 30);
            let mut cuts: Vec<usize> = (1..n).collect();
            r.shuffle(&mut cuts);
            cuts.truncate(r.range(0, 6.min(n - 1)));
            let (c, order) = break_string(n, &cuts);
            assert_eq!(cost_of_order(n, &order), c);
            let brute = perms(&cuts).iter().map(|p| cost_of_order(n, p)).min().unwrap();
            assert_eq!(c, brute, "n={n} cuts={cuts:?}");
        }
    }
}
