//! @title List Ranking (Pointer Jumping)
//! @chapter 30.1 (1e)
//! @editions 1
//! @summary Compute every node's distance to the end of a linked list in O(lg n) PRAM steps by repeated pointer doubling.
//! @order 10
use crate::outln;

/// PRAM-style synchronous list ranking. `next[i]` = successor of i (None at the tail).
/// Returns (distance of each node to the tail, number of parallel rounds used).
pub fn list_rank(next: &[Option<usize>]) -> (Vec<usize>, usize) {
    let n = next.len();
    let mut d: Vec<usize> = next.iter().map(|x| if x.is_some() { 1 } else { 0 }).collect();
    let mut nxt = next.to_vec();
    let mut rounds = 0;
    while nxt.iter().any(|x| x.is_some()) {
        // all n processors read the old arrays, then all write: one synchronous PRAM step
        let (d_old, n_old) = (d.clone(), nxt.clone());
        for i in 0..n {
            if let Some(j) = n_old[i] {
                d[i] = d_old[i] + d_old[j];
                nxt[i] = n_old[j];
            }
        }
        rounds += 1;
    }
    (d, rounds)
}

/// Build the `next` array of a list that visits `order[0], order[1], …`.
pub fn list_from_order(order: &[usize]) -> Vec<Option<usize>> {
    let mut next = vec![None; order.len()];
    for w in order.windows(2) { next[w[0]] = Some(w[1]); }
    next
}

pub fn demo() -> String {
    let mut o = String::new();
    let order = [3, 6, 0, 4, 1, 5, 2];
    let (d, rounds) = list_rank(&list_from_order(&order));
    outln!(o, "list order: {:?}", order);
    outln!(o, "distance to tail per node: {:?}", d);
    outln!(o, "synchronous rounds: {} (⌈lg 7⌉ = 3)", rounds);
    for n in [16usize, 1024, 65536] {
        let ord: Vec<usize> = (0..n).collect();
        outln!(o, "n = {:>5}: {} rounds (lg n = {})", n, list_rank(&list_from_order(&ord)).1, n.ilog2());
    }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn ranks_match_sequential_walk_and_rounds_are_logarithmic() {
        let mut r = Rng::new(160);
        for n in [1usize, 2, 3, 8, 50, 257] {
            let mut order: Vec<usize> = (0..n).collect();
            r.shuffle(&mut order);
            let (d, rounds) = list_rank(&list_from_order(&order));
            for (pos, &node) in order.iter().enumerate() { assert_eq!(d[node], n - 1 - pos); }
            assert!(rounds <= (n as f64).log2().ceil() as usize, "n={n} rounds={rounds}");
        }
    }
}
