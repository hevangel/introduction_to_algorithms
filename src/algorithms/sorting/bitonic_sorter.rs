//! @title Bitonic Sorting Network
//! @chapter 27.3–27.4 (1e, 2e)
//! @editions 1,2
//! @summary Half-cleaners turn a bitonic sequence into two bitonic halves; depth lg n·(lg n+1)/2.
//! @order 140
use super::comparison_network::Network;
use crate::outln;

fn half_cleaner(net: &mut Network, lo: usize, n: usize) {
    for i in 0..n / 2 { net.add(lo + i, lo + i + n / 2); }
}

/// BITONIC-SORTER[n]: sorts a bitonic sequence on wires lo..lo+n.
fn bitonic_sorter(net: &mut Network, lo: usize, n: usize) {
    if n > 1 {
        half_cleaner(net, lo, n);
        bitonic_sorter(net, lo, n / 2);
        bitonic_sorter(net, lo + n / 2, n / 2);
    }
}

/// MERGER[n]: merges two sorted n/2 sequences (the first comparison stage flips the second half).
fn merger(net: &mut Network, lo: usize, n: usize) {
    for i in 0..n / 2 { net.add(lo + i, lo + n - 1 - i); }
    bitonic_sorter(net, lo, n / 2);
    bitonic_sorter(net, lo + n / 2, n / 2);
}

/// SORTER[n] for n a power of two.
fn sorter(net: &mut Network, lo: usize, n: usize) {
    if n > 1 {
        sorter(net, lo, n / 2);
        sorter(net, lo + n / 2, n / 2);
        merger(net, lo, n);
    }
}

/// Build the full bitonic sorting network on n = 2^k wires.
pub fn bitonic_sorting_network(n: usize) -> Network {
    assert!(n.is_power_of_two());
    let mut net = Network::new(n);
    sorter(&mut net, 0, n);
    net
}

/// Is the sequence bitonic (increasing then decreasing, up to a cyclic shift)?
pub fn is_bitonic(a: &[i64]) -> bool {
    let n = a.len();
    let changes = (0..n).filter(|&i| {
        let (p, c, nx) = (a[(i + n - 1) % n], a[i], a[(i + 1) % n]);
        (c > p && c > nx) || (c < p && c < nx)
    }).count();
    changes <= 2
}

pub fn demo() -> String {
    let mut o = String::new();
    for k in 1..=5 {
        let n = 1usize << k;
        let net = bitonic_sorting_network(n);
        outln!(o, "n={:>2}: comparators={:>3} depth={:>2} (lg n·(lg n+1)/2 = {})", n, net.size(), net.depth(), k * (k + 1) / 2);
    }
    let mut v = [7, 3, 9, 1, 8, 2, 6, 4];
    bitonic_sorting_network(8).apply(&mut v);
    outln!(o, "sorted by the 8-wire network: {:?}", v);
    outln!(o, "is_bitonic([1,4,6,5,2]) = {}   is_bitonic([1,5,2,6,3]) = {}", is_bitonic(&[1, 4, 6, 5, 2]), is_bitonic(&[1, 5, 2, 6, 3]));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    #[test]
    fn is_a_sorting_network_for_powers_of_two() {
        for k in 0..=4 {
            assert!(bitonic_sorting_network(1 << k).is_sorting_network(), "n={}", 1 << k);
        }
    }
    #[test]
    fn depth_and_size_formulas() {
        for k in 1..=6usize {
            let n = 1usize << k;
            let net = bitonic_sorting_network(n);
            assert_eq!(net.depth(), k * (k + 1) / 2);
            assert_eq!(net.size(), n / 2 * (k * (k + 1) / 2));
        }
    }
    #[test]
    fn sorts_random_integers() {
        let net = bitonic_sorting_network(32);
        let mut r = Rng::new(38);
        for _ in 0..50 {
            let mut v = r.vec(32, -500, 500);
            let mut e = v.clone();
            e.sort();
            net.apply(&mut v);
            assert_eq!(v, e);
        }
    }
    #[test]
    fn bitonic_predicate() {
        assert!(is_bitonic(&[1, 3, 5, 4, 2]));
        assert!(is_bitonic(&[4, 2, 1, 3, 5]));
        assert!(!is_bitonic(&[1, 5, 2, 6, 3]));
    }
}
