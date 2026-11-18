//! @title Transposition & Permutation Networks
//! @chapter Problems 28-1 & 28-3 (1e); Problems 27-1 & 27-3 (2e)
//! @editions 1,2
//! @summary Odd-even transposition sorting in Θ(n) depth and recursive 2-switch Beneš permutation routing in O(lg n) depth and O(n lg n) switches.
//! @order 105
use crate::outln;

/// Odd-even transposition (brick) sort: alternates even-pair and odd-pair compare-exchange layers.
/// Returns the number of parallel layers executed (at most n).
pub fn odd_even_transposition_sort(a: &mut [i64]) -> usize {
    let n = a.len();
    if n <= 1 {
        return 0;
    }
    let mut layers = 0usize;
    for phase in 0..n {
        let mut swapped = false;
        let start = phase & 1;
        let mut i = start;
        while i + 1 < n {
            if a[i] > a[i + 1] {
                a.swap(i, i + 1);
                swapped = true;
            }
            i += 2;
        }
        layers += 1;
        if !swapped && phase > 0 {
            break;
        }
    }
    layers
}

/// Recursive Beneš / Waksman permutation network on n = 2^k wires (`2*k - 1` stages of 2-switches).
#[derive(Debug, Clone)]
pub struct BenesNetwork {
    pub n: usize,
    /// `stages[s][sw] == true` means 2-switch `sw` crosses its two wires; `false` means straight-through.
    pub input_cross: Vec<bool>,
    pub output_cross: Vec<bool>,
    pub sub: Option<(Box<BenesNetwork>, Box<BenesNetwork>)>,
}

impl BenesNetwork {
    /// Construct switch settings routing input wire `i` to output wire `perm[i]` for n = 2^k (k >= 1).
    pub fn from_permutation(perm: &[usize]) -> Self {
        let n = perm.len();
        assert!(n >= 2 && n.is_power_of_two(), "n must be a power of 2 >= 2");
        if n == 2 {
            assert!((perm[0] == 0 && perm[1] == 1) || (perm[0] == 1 && perm[1] == 0));
            return Self {
                n: 2,
                input_cross: vec![perm[0] == 1],
                output_cross: vec![],
                sub: None,
            };
        }
        let m = n / 2;
        let mut inv = vec![0usize; n];
        for (i, &d) in perm.iter().enumerate() {
            inv[d] = i;
        }
        // Bipartite 2-coloring of wires with constraints:
        // - input pair (2p, 2p+1) must take opposite colors (UPPER=0 vs LOWER=1)
        // - output pair mates (i, inv[perm[i] ^ 1]) must take opposite colors.
        let mut color: Vec<Option<u8>> = vec![None; n];
        for start in 0..n {
            if color[start].is_some() {
                continue;
            }
            let mut stack = vec![(start, 0u8)];
            while let Some((u, c)) = stack.pop() {
                if let Some(prev) = color[u] {
                    assert_eq!(prev, c, "conflict in Beneš 2-coloring");
                    continue;
                }
                color[u] = Some(c);
                stack.push((u ^ 1, c ^ 1));
                let out_mate = inv[perm[u] ^ 1];
                stack.push((out_mate, c ^ 1));
            }
        }
        let mut input_cross = vec![false; m];
        let mut upper_perm = vec![0usize; m];
        let mut lower_perm = vec![0usize; m];
        let mut output_cross = vec![false; m];
        for p in 0..m {
            let c0 = color[2 * p].unwrap();
            let (u_wire, l_wire) = if c0 == 0 {
                (2 * p, 2 * p + 1)
            } else {
                input_cross[p] = true;
                (2 * p + 1, 2 * p)
            };
            upper_perm[p] = perm[u_wire] / 2;
            lower_perm[p] = perm[l_wire] / 2;
            let q = perm[u_wire] / 2;
            // UPPER subnetwork outputs into top pin of output switch `q`;
            // if perm[u_wire] is odd (2q+1), output switch `q` must cross!
            output_cross[q] = (perm[u_wire] & 1) == 1;
        }
        let upper = Self::from_permutation(&upper_perm);
        let lower = Self::from_permutation(&lower_perm);
        Self {
            n,
            input_cross,
            output_cross,
            sub: Some((Box::new(upper), Box::new(lower))),
        }
    }

    pub fn stage_count(&self) -> usize {
        match &self.sub {
            None => 1,
            Some((u, _)) => 2 + u.stage_count(),
        }
    }

    pub fn route<T: Clone>(&self, inputs: &[T]) -> Vec<T> {
        assert_eq!(inputs.len(), self.n);
        if self.n == 2 {
            return if self.input_cross[0] {
                vec![inputs[1].clone(), inputs[0].clone()]
            } else {
                inputs.to_vec()
            };
        }
        let m = self.n / 2;
        let mut upper_in = Vec::with_capacity(m);
        let mut lower_in = Vec::with_capacity(m);
        for p in 0..m {
            if self.input_cross[p] {
                upper_in.push(inputs[2 * p + 1].clone());
                lower_in.push(inputs[2 * p].clone());
            } else {
                upper_in.push(inputs[2 * p].clone());
                lower_in.push(inputs[2 * p + 1].clone());
            }
        }
        let (u_net, l_net) = self.sub.as_ref().unwrap();
        let upper_out = u_net.route(&upper_in);
        let lower_out = l_net.route(&lower_in);
        let mut out = Vec::with_capacity(self.n);
        for q in 0..m {
            if self.output_cross[q] {
                out.push(lower_out[q].clone());
                out.push(upper_out[q].clone());
            } else {
                out.push(upper_out[q].clone());
                out.push(lower_out[q].clone());
            }
        }
        out
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut a = vec![8, 3, 7, 1, 6, 2, 5, 4];
    let layers = odd_even_transposition_sort(&mut a);
    outln!(o, "odd-even transposition sort in {} layers -> {:?}", layers, a);
    let perm = vec![5, 2, 7, 0, 4, 1, 6, 3];
    let net = BenesNetwork::from_permutation(&perm);
    let routed = net.route(&(0..8usize).collect::<Vec<_>>());
    let mut expected = vec![0usize; 8];
    for (i, &dest) in perm.iter().enumerate() {
        expected[dest] = i;
    }
    outln!(o, "Beneš n=8 stages = {}, perm = {:?}", net.stage_count(), perm);
    outln!(o, "routed wires = {:?} (exact = {})", routed, routed == expected);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn odd_even_transposition_sorts_all_inputs() {
        let mut r = Rng::new(2701);
        for n in 0..30 {
            let mut a: Vec<i64> = r.vec(n, -100, 100);
            let mut expected = a.clone();
            expected.sort();
            let layers = odd_even_transposition_sort(&mut a);
            assert_eq!(a, expected);
            assert!(layers <= n);
        }
    }

    #[test]
    fn benes_routes_arbitrary_power_of_two_permutations() {
        let mut r = Rng::new(2703);
        for k in 1..=6 {
            let n = 1usize << k;
            for _ in 0..40 {
                let mut perm: Vec<usize> = (0..n).collect();
                for i in (1..n).rev() {
                    let j = r.range(0, i);
                    perm.swap(i, j);
                }
                let net = BenesNetwork::from_permutation(&perm);
                assert_eq!(net.stage_count(), 2 * k - 1);
                let out = net.route(&(0..n).collect::<Vec<_>>());
                for (i, &dest) in perm.iter().enumerate() {
                    assert_eq!(out[dest], i, "failed for n={n}, perm={perm:?}");
                }
            }
        }
    }
}
