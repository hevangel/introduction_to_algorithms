//! @title Graph Representations
//! @chapter 22.1 (3e) / 20.1 (4e) / 23.1 (1e–2e)
//! @editions 1,2,3,4
//! @summary Adjacency lists and matrices, transposing a graph, squaring a graph, and the O(V) universal-sink test.
//! @order 10
use crate::outln;

pub type AdjList = Vec<Vec<usize>>;
pub type AdjMatrix = Vec<Vec<bool>>;

pub fn list_to_matrix(adj: &AdjList) -> AdjMatrix {
    let n = adj.len();
    let mut m = vec![vec![false; n]; n];
    for (u, vs) in adj.iter().enumerate() { for &v in vs { m[u][v] = true; } }
    m
}
pub fn matrix_to_list(m: &AdjMatrix) -> AdjList {
    m.iter().map(|row| row.iter().enumerate().filter(|(_, &b)| b).map(|(v, _)| v).collect()).collect()
}
/// Exercise 22.1-3: G^T in Θ(V+E) with adjacency lists.
pub fn transpose(adj: &AdjList) -> AdjList {
    let mut t = vec![vec![]; adj.len()];
    for (u, vs) in adj.iter().enumerate() { for &v in vs { t[v].push(u); } }
    t
}
/// Exercise 22.1-5: the square G² (edge (u,w) iff G has a path of length ≤ 2 from u to w), Θ(V·E).
pub fn square(adj: &AdjList) -> AdjList {
    let n = adj.len();
    let mut out = vec![vec![]; n];
    for u in 0..n {
        let mut seen = vec![false; n];
        for &v in &adj[u] {
            seen[v] = true;
            for &w in &adj[v] { seen[w] = true; }
        }
        out[u] = (0..n).filter(|&w| seen[w]).collect();
    }
    out
}
/// Exercise 22.1-6: find a universal sink (in-degree |V|−1, out-degree 0) in O(V) matrix probes.
pub fn universal_sink(m: &AdjMatrix) -> Option<usize> {
    let n = m.len();
    let (mut i, mut j) = (0, 0);
    while i < n && j < n {
        if m[i][j] { i += 1; } else { j += 1; } // edge i→j: i can't be a sink; otherwise j can't be a universal sink
    }
    if i >= n { return None; }
    let ok = (0..n).all(|k| !m[i][k] && (k == i || m[k][i]));
    if ok { Some(i) } else { None }
}
pub fn degrees(adj: &AdjList) -> (Vec<usize>, Vec<usize>) {
    let n = adj.len();
    let mut ind = vec![0; n];
    for vs in adj { for &v in vs { ind[v] += 1; } }
    (ind, adj.iter().map(|v| v.len()).collect())
}

pub fn demo() -> String {
    let mut o = String::new();
    let adj: AdjList = vec![vec![1, 3], vec![4], vec![4, 5], vec![1], vec![3], vec![5]];
    outln!(o, "adjacency list : {:?}", adj);
    let m = list_to_matrix(&adj);
    for row in &m { outln!(o, "  {}", row.iter().map(|&b| if b { '1' } else { '0' }).collect::<String>()); }
    outln!(o, "round trip equal: {}", matrix_to_list(&m) == adj);
    outln!(o, "transpose      : {:?}", transpose(&adj));
    outln!(o, "square G²      : {:?}", square(&adj));
    let sink = vec![vec![false, false, false], vec![true, false, true], vec![true, false, false]]; // vertex 0 has no out-edges? use row semantic
    outln!(o, "universal sink of {:?}: {:?}", sink, universal_sink(&sink));
    let (ind, outd) = degrees(&adj);
    outln!(o, "in-degrees {:?}, out-degrees {:?}", ind, outd);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn random_graph(n: usize, p: f64, r: &mut Rng) -> AdjList {
        (0..n).map(|u| (0..n).filter(|&v| u != v && r.f64() < p).collect()).collect()
    }
    #[test]
    fn conversions_roundtrip_and_transpose_twice() {
        let mut r = Rng::new(120);
        for n in [0, 1, 5, 12] {
            let g = random_graph(n, 0.3, &mut r);
            assert_eq!(matrix_to_list(&list_to_matrix(&g)), g);
            let mut tt = transpose(&transpose(&g));
            tt.iter_mut().for_each(|v| v.sort());
            assert_eq!(tt, g);
        }
    }
    #[test]
    fn square_matches_matrix_definition() {
        let mut r = Rng::new(121);
        let g = random_graph(10, 0.2, &mut r);
        let m = list_to_matrix(&g);
        let sq = list_to_matrix(&square(&g));
        for u in 0..10 { for w in 0..10 {
            let want = m[u][w] || (0..10).any(|v| m[u][v] && m[v][w]);
            assert_eq!(sq[u][w], want || (u == w && (0..10).any(|v| m[u][v] && m[v][u])));
        } }
    }
    #[test]
    fn universal_sink_found_in_linear_probes() {
        let n = 8;
        for sink in 0..n {
            let mut m = vec![vec![false; n]; n];
            for i in 0..n { if i != sink { m[i][sink] = true; } }
            m[0][3] = sink != 0 && sink != 3 || m[0][3];
            assert_eq!(universal_sink(&m), Some(sink));
        }
        let none = vec![vec![false; 4]; 4];
        assert_eq!(universal_sink(&none), None);
    }
}
