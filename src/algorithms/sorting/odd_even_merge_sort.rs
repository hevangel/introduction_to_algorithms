//! @title Odd-Even Merging Network
//! @chapter 27.4 / Problem 27-5 (1e, 2e)
//! @editions 1,2
//! @summary Batcher's odd-even merge sort: merge by recursively merging odd and even subsequences.
//! @order 150
use super::comparison_network::Network;
use crate::outln;

/// Merge the sorted halves of wires lo..=hi using comparison distance r (Batcher).
fn oddeven_merge(net: &mut Network, lo: usize, hi: usize, r: usize) {
    let step = r * 2;
    if step < hi - lo {
        oddeven_merge(net, lo, hi, step);
        oddeven_merge(net, lo + r, hi, step);
        let mut i = lo + r;
        while i + r <= hi {
            net.add(i, i + r);
            i += step;
        }
    } else {
        net.add(lo, lo + r);
    }
}

fn sort_range(net: &mut Network, lo: usize, hi: usize) {
    if hi > lo {
        let mid = lo + (hi - lo) / 2;
        sort_range(net, lo, mid);
        sort_range(net, mid + 1, hi);
        oddeven_merge(net, lo, hi, 1);
    }
}

/// Batcher's odd-even merge sorting network for n = 2^k wires.
pub fn odd_even_merge_sort_network(n: usize) -> Network {
    assert!(n.is_power_of_two());
    let mut net = Network::new(n);
    if n > 1 { sort_range(&mut net, 0, n - 1); }
    net
}

/// A network that only merges: wires 0..n/2 and n/2..n are each sorted on input.
pub fn odd_even_merging_network(n: usize) -> Network {
    assert!(n.is_power_of_two() && n >= 2);
    let mut net = Network::new(n);
    oddeven_merge(&mut net, 0, n - 1, 1);
    net
}

pub fn demo() -> String {
    let mut o = String::new();
    outln!(o, "   n | odd-even merge sort (size, depth) | bitonic (size, depth)");
    for k in 1..=5usize {
        let n = 1 << k;
        let a = odd_even_merge_sort_network(n);
        let b = super::bitonic_sorter::bitonic_sorting_network(n);
        outln!(o, "{:>4} | ({:>3}, {:>2})                          | ({:>3}, {:>2})", n, a.size(), a.depth(), b.size(), b.depth());
    }
    let mut v = [1, 4, 6, 9, 2, 3, 7, 8];
    odd_even_merging_network(8).apply(&mut v);
    outln!(o, "merging two sorted halves [1,4,6,9] [2,3,7,8] -> {:?}", v);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn full_sorter_passes_zero_one_test() {
        for k in 0..=4 {
            assert!(odd_even_merge_sort_network(1 << k).is_sorting_network(), "n={}", 1 << k);
        }
    }
    #[test]
    fn uses_fewer_comparators_than_bitonic() {
        for k in 2..=6 {
            let n = 1usize << k;
            let a = odd_even_merge_sort_network(n);
            assert_eq!(a.depth(), k * (k + 1) / 2);
            assert!(a.size() < super::super::bitonic_sorter::bitonic_sorting_network(n).size());
        }
    }
    #[test]
    fn merging_network_merges_sorted_halves() {
        let mut r = Rng::new(39);
        let net = odd_even_merging_network(16);
        for _ in 0..100 {
            let mut v = r.vec(16, -50, 50);
            v[..8].sort();
            v[8..].sort();
            let mut e = v.clone();
            e.sort();
            net.apply(&mut v);
            assert_eq!(v, e);
        }
    }
}
