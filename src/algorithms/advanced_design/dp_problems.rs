//! @title More DP: Company Party & Viterbi
//! @chapter Problems 15-6, 15-7 (3e), 14-6, 14-7 (4e)
//! @editions 3,4
//! @summary Planning a company party (tree DP, maximum independent set) and Viterbi decoding of a speech-recognition graph.
//! @order 140
use crate::outln;

/// Problem 15-6: `children[v]` lists v's direct reports; `rating[v]` is v's conviviality.
/// Returns the maximum total rating of an invitation list that never contains an employee together with their direct supervisor.
pub fn company_party(children: &[Vec<usize>], rating: &[i64], root: usize) -> (i64, Vec<usize>) {
    // returns (best with v invited, best without v)
    fn go(v: usize, ch: &[Vec<usize>], r: &[i64], with: &mut Vec<i64>, without: &mut Vec<i64>) {
        with[v] = r[v];
        without[v] = 0;
        for &c in &ch[v] {
            go(c, ch, r, with, without);
            with[v] += without[c];
            without[v] += with[c].max(without[c]);
        }
    }
    let n = children.len();
    let (mut with, mut without) = (vec![0; n], vec![0; n]);
    go(root, children, rating, &mut with, &mut without);
    // reconstruct the guest list
    let mut guests = vec![];
    let mut stack = vec![(root, true)]; // (vertex, parent_not_invited)
    while let Some((v, free)) = stack.pop() {
        let invite = free && with[v] >= without[v];
        if invite { guests.push(v); }
        for &c in &children[v] { stack.push((c, !invite)); }
    }
    guests.sort();
    (with[root].max(without[root]), guests)
}

/// Problem 15-7: a directed graph whose edges carry a symbol and a probability.
/// Finds the most probable path from `start` whose edge labels spell `sigma`; None if none exists.
pub fn viterbi(edges: &[(usize, usize, char, f64)], n: usize, start: usize, sigma: &str) -> Option<(f64, Vec<usize>)> {
    let s: Vec<char> = sigma.chars().collect();
    let k = s.len();
    let mut best = vec![vec![0.0f64; n]; k + 1];
    let mut back = vec![vec![usize::MAX; n]; k + 1];
    best[0][start] = 1.0;
    for i in 1..=k {
        for &(u, v, c, p) in edges {
            if c == s[i - 1] && best[i - 1][u] * p > best[i][v] {
                best[i][v] = best[i - 1][u] * p;
                back[i][v] = u;
            }
        }
    }
    let end = (0..n).max_by(|&a, &b| best[k][a].partial_cmp(&best[k][b]).unwrap())?;
    if best[k][end] == 0.0 { return None; }
    let mut path = vec![end];
    let mut v = end;
    for i in (1..=k).rev() {
        v = back[i][v];
        path.push(v);
    }
    path.reverse();
    Some((best[k][end], path))
}

pub fn demo() -> String {
    let mut o = String::new();
    //        0 (5)
    //      /   |   \
    //   1(6)  2(2) 3(3)
    //   / \         |
    // 4(4) 5(1)    6(9)
    let children = vec![vec![1, 2, 3], vec![4, 5], vec![], vec![6], vec![], vec![], vec![]];
    let rating = [5, 6, 2, 3, 4, 1, 9];
    let (best, guests) = company_party(&children, &rating, 0);
    outln!(o, "company party: best total conviviality {} inviting employees {:?}", best, guests);
    let edges = [(0, 1, 'a', 0.5), (0, 2, 'a', 0.3), (1, 3, 'b', 0.9), (2, 3, 'b', 1.0), (3, 0, 'a', 0.6)];
    outln!(o, "viterbi 'ab' from state 0: {:?}", viterbi(&edges, 4, 0, "ab"));
    outln!(o, "viterbi 'abaab' from state 0: {:?}", viterbi(&edges, 4, 0, "abaab"));
    outln!(o, "viterbi 'bb' from state 0: {:?}", viterbi(&edges, 4, 0, "bb"));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn party_matches_brute_force() {
        let mut r = Rng::new(111);
        for _ in 0..50 {
            let n = r.range(1, 10);
            let mut children = vec![vec![]; n];
            let mut parent = vec![usize::MAX; n];
            for v in 1..n { let p = r.range(0, v - 1); children[p].push(v); parent[v] = p; }
            let rating = r.vec(n, -3, 9);
            let (best, guests) = company_party(&children, &rating, 0);
            let brute = (0..1u32 << n).filter(|m| (1..n).all(|v| !(m >> v & 1 == 1 && m >> parent[v] & 1 == 1)))
                .map(|m| (0..n).filter(|v| m >> v & 1 == 1).map(|v| rating[v]).sum::<i64>()).max().unwrap();
            assert_eq!(best, brute);
            assert_eq!(guests.iter().map(|&v| rating[v]).sum::<i64>(), best);
            assert!(guests.iter().all(|&v| v == 0 || !guests.contains(&parent[v])));
        }
    }
    #[test]
    fn viterbi_picks_most_probable_path() {
        let edges = [(0, 1, 'a', 0.5), (0, 2, 'a', 0.3), (1, 3, 'b', 0.9), (2, 3, 'b', 1.0), (3, 0, 'a', 0.6)];
        let (p, path) = viterbi(&edges, 4, 0, "ab").unwrap();
        assert!((p - 0.45).abs() < 1e-12);
        assert_eq!(path, vec![0, 1, 3]);
        assert!(viterbi(&edges, 4, 0, "bb").is_none());
        let (p, path) = viterbi(&edges, 4, 0, "abaab").unwrap();
        assert_eq!(path.len(), 6);
        assert!(p > 0.0);
    }
}
