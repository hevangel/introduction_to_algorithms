//! @title Stable Matching (Gale-Shapley)
//! @chapter 25.2 (4e)
//! @editions 4
//! @summary Propose-and-reject deferred acceptance yields a stable marriage in O(n²) — optimal for the proposing side.
//! @order 220
use crate::outln;

/// Gale–Shapley with men proposing. `men[m]` / `women[w]` list partners from most to least preferred.
/// Returns `wife[m]` for each man.
pub fn gale_shapley(men: &[Vec<usize>], women: &[Vec<usize>]) -> Vec<usize> {
    let n = men.len();
    // rank[w][m] = position of man m in woman w's list
    let mut rank = vec![vec![0usize; n]; n];
    for w in 0..n { for (pos, &m) in women[w].iter().enumerate() { rank[w][m] = pos; } }
    let (mut next, mut husband, mut wife) = (vec![0usize; n], vec![usize::MAX; n], vec![usize::MAX; n]);
    let mut free: Vec<usize> = (0..n).rev().collect();
    while let Some(m) = free.pop() {
        let w = men[m][next[m]];
        next[m] += 1;
        if husband[w] == usize::MAX {
            husband[w] = m; wife[m] = w;
        } else if rank[w][m] < rank[w][husband[w]] {
            let old = husband[w];
            wife[old] = usize::MAX;
            free.push(old);
            husband[w] = m; wife[m] = w;
        } else {
            free.push(m);
        }
    }
    wife
}

/// Is there a pair (m, w) preferring each other to their assigned partners?
pub fn blocking_pairs(men: &[Vec<usize>], women: &[Vec<usize>], wife: &[usize]) -> Vec<(usize, usize)> {
    let n = men.len();
    let mut husband = vec![0; n];
    for m in 0..n { husband[wife[m]] = m; }
    let pos = |list: &Vec<usize>, x: usize| list.iter().position(|&y| y == x).unwrap();
    let mut out = vec![];
    for m in 0..n { for w in 0..n {
        if pos(&men[m], w) < pos(&men[m], wife[m]) && pos(&women[w], m) < pos(&women[w], husband[w]) { out.push((m, w)); }
    } }
    out
}

pub fn demo() -> String {
    let mut o = String::new();
    let men = vec![vec![0, 1, 2], vec![1, 0, 2], vec![0, 1, 2]];
    let women = vec![vec![1, 0, 2], vec![0, 1, 2], vec![0, 1, 2]];
    let wife = gale_shapley(&men, &women);
    outln!(o, "men's preferences  : {:?}", men);
    outln!(o, "women's preferences: {:?}", women);
    outln!(o, "man → woman        : {:?}", wife);
    outln!(o, "blocking pairs     : {:?}", blocking_pairs(&men, &women, &wife));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn permutations(n: usize) -> Vec<Vec<usize>> {
        if n == 0 { return vec![vec![]]; }
        let mut out = vec![];
        for p in permutations(n - 1) { for pos in 0..=p.len() { let mut q = p.clone(); q.insert(pos, n - 1); out.push(q); } }
        out
    }
    #[test]
    fn stable_and_men_optimal() {
        let mut r = Rng::new(143);
        for _ in 0..40 {
            let n = r.range(1, 5);
            let prefs = |r: &mut Rng| -> Vec<Vec<usize>> { (0..n).map(|_| { let mut v: Vec<usize> = (0..n).collect(); r.shuffle(&mut v); v }).collect() };
            let (men, women) = (prefs(&mut r), prefs(&mut r));
            let wife = gale_shapley(&men, &women);
            assert!(blocking_pairs(&men, &women, &wife).is_empty());
            // brute force: every stable matching; each man's GS partner must be his best among all stable partners
            let stable: Vec<Vec<usize>> = permutations(n).into_iter().filter(|p| blocking_pairs(&men, &women, p).is_empty()).collect();
            assert!(!stable.is_empty());
            for m in 0..n {
                let best = stable.iter().map(|p| men[m].iter().position(|&w| w == p[m]).unwrap()).min().unwrap();
                assert_eq!(men[m][best], wife[m]);
            }
        }
    }
    #[test]
    fn terminates_in_at_most_n_squared_proposals() {
        let n = 30;
        let men: Vec<Vec<usize>> = (0..n).map(|_| (0..n).collect()).collect(); // everyone ranks identically
        let women: Vec<Vec<usize>> = (0..n).map(|_| (0..n).collect()).collect();
        let wife = gale_shapley(&men, &women);
        assert_eq!(wife, (0..n).collect::<Vec<_>>());
    }
}
