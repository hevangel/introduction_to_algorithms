//! @title Push-Relabel & Relabel-to-Front
//! @chapter 26.4–26.5 (1e–3e)
//! @editions 1,2,3
//! @summary Preflow-push maximum flow: generic O(V²E) and relabel-to-front O(V³).
//! @order 180
use crate::outln;

struct State {
    n: usize,
    c: Vec<Vec<i64>>,
    f: Vec<Vec<i64>>,
    h: Vec<usize>,
    e: Vec<i64>,
    pushes: usize,
    relabels: usize,
}

impl State {
    /// INITIALIZE-PREFLOW(G, s)
    fn new(n: usize, edges: &[(usize, usize, i64)], s: usize) -> Self {
        let mut c = vec![vec![0i64; n]; n];
        for &(u, v, w) in edges { c[u][v] += w; }
        let mut st = State { n, c, f: vec![vec![0; n]; n], h: vec![0; n], e: vec![0; n], pushes: 0, relabels: 0 };
        st.h[s] = n;
        for v in 0..n {
            if st.c[s][v] > 0 {
                st.f[s][v] = st.c[s][v];
                st.f[v][s] = -st.c[s][v];
                st.e[v] += st.c[s][v];
                st.e[s] -= st.c[s][v];
            }
        }
        st
    }
    fn cf(&self, u: usize, v: usize) -> i64 { self.c[u][v] - self.f[u][v] }
    /// PUSH(u, v): send min(e[u], c_f(u,v)) units from u to v.
    fn push(&mut self, u: usize, v: usize) {
        let d = self.e[u].min(self.cf(u, v));
        self.f[u][v] += d;
        self.f[v][u] -= d;
        self.e[u] -= d;
        self.e[v] += d;
        self.pushes += 1;
    }
    /// RELABEL(u): lift u just above its lowest residual neighbour.
    fn relabel(&mut self, u: usize) {
        let m = (0..self.n).filter(|&v| self.cf(u, v) > 0).map(|v| self.h[v]).min().unwrap();
        self.h[u] = 1 + m;
        self.relabels += 1;
    }
}

/// GENERIC-PUSH-RELABEL: apply any applicable push or relabel until no vertex (other than s, t) overflows.
pub fn generic_push_relabel(n: usize, edges: &[(usize, usize, i64)], s: usize, t: usize) -> i64 {
    generic_with_stats(n, edges, s, t).0
}

/// Returns (max flow, pushes, relabels).
pub fn generic_with_stats(n: usize, edges: &[(usize, usize, i64)], s: usize, t: usize) -> (i64, usize, usize) {
    let mut st = State::new(n, edges, s);
    'outer: loop {
        for u in 0..n {
            if u == s || u == t || st.e[u] <= 0 { continue; }
            if let Some(v) = (0..n).find(|&v| st.cf(u, v) > 0 && st.h[u] == st.h[v] + 1) { st.push(u, v); } else { st.relabel(u); }
            continue 'outer;
        }
        return (st.e[t], st.pushes, st.relabels);
    }
}

/// RELABEL-TO-FRONT: discharge vertices from a list, moving relabelled vertices to the front. O(V³).
pub fn relabel_to_front(n: usize, edges: &[(usize, usize, i64)], s: usize, t: usize) -> i64 {
    let mut st = State::new(n, edges, s);
    let mut list: Vec<usize> = (0..n).filter(|&v| v != s && v != t).collect();
    let mut cur = vec![0usize; n]; // current-neighbour pointers
    let mut i = 0;
    while i < list.len() {
        let u = list[i];
        let old = st.h[u];
        // DISCHARGE(u)
        while st.e[u] > 0 {
            if cur[u] == n { st.relabel(u); cur[u] = 0; }
            else {
                let v = cur[u];
                if st.cf(u, v) > 0 && st.h[u] == st.h[v] + 1 { st.push(u, v); } else { cur[u] += 1; }
            }
        }
        if st.h[u] > old {
            list.remove(i);
            list.insert(0, u);
            i = 0;
        }
        i += 1;
    }
    st.e[t]
}

pub fn demo() -> String {
    let mut o = String::new();
    let edges = [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)];
    let (f, pushes, relabels) = generic_with_stats(6, &edges, 0, 5);
    outln!(o, "generic push-relabel: max flow {f} using {pushes} pushes and {relabels} relabels");
    outln!(o, "relabel-to-front    : max flow {}", relabel_to_front(6, &edges, 0, 5));
    let mut g = super::ford_fulkerson::clrs_network();
    outln!(o, "Edmonds-Karp check  : {}", g.edmonds_karp(0, 5));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn book_network() {
        let edges = [(0, 1, 16), (0, 2, 13), (1, 3, 12), (2, 1, 4), (2, 4, 14), (3, 2, 9), (3, 5, 20), (4, 3, 7), (4, 5, 4)];
        assert_eq!(generic_push_relabel(6, &edges, 0, 5), 23);
        assert_eq!(relabel_to_front(6, &edges, 0, 5), 23);
    }
    #[test]
    fn agrees_with_edmonds_karp_on_random_networks() {
        let mut r = Rng::new(139);
        for _ in 0..80 {
            let n = r.range(2, 9);
            let edges: Vec<(usize, usize, i64)> = (0..r.range(1, 25)).map(|_| (r.range(0, n - 1), r.range(0, n - 1), r.range(1, 20) as i64)).filter(|e| e.0 != e.1).collect();
            let mut g = super::super::ford_fulkerson::FlowNetwork::new(n);
            for &(u, v, c) in &edges { g.add_edge(u, v, c); }
            let want = g.edmonds_karp(0, n - 1);
            assert_eq!(generic_push_relabel(n, &edges, 0, n - 1), want, "{edges:?}");
            assert_eq!(relabel_to_front(n, &edges, 0, n - 1), want, "{edges:?}");
        }
    }
}
