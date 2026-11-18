//! @title Computational Models & the RAM Model
//! @chapter 2.2 (1e–4e)
//! @editions 1,2,3,4
//! @summary Lesson 0: the sequential Random-Access Machine (RAM) abstract computer, statement-by-statement cost counting, cache/pointer-chasing gaps, and alternative models (Word-RAM, I/O, PRAM, work-span, circuits).
//! @order 5
use crate::common::Rng;
use crate::outln;

/// Line-by-line primitive instruction counts for CLRS Section 2.2 INSERTION-SORT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RamProfile {
    /// Total loop iterations `n` (line 1 header executions).
    pub outer_checks: usize,
    /// Inner while-test executions `sum_{j=2..n} t_j`.
    pub comparisons: usize,
    /// Array element reads (key loads + A[i] checks + shifts).
    pub mem_reads: usize,
    /// Array element writes (shifts + key insertions).
    pub mem_writes: usize,
    /// Primitive steps under unit-cost CLRS line weights `c_k = 1`.
    pub total_steps: usize,
}

/// Three-layer cost comparison bridging CLRS theory and real hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelCost {
    pub ram_steps: usize,
    pub machine_mem_ops: usize,
    pub cache_misses: usize,
}

/// Run CLRS INSERTION-SORT while recording exact Section 2.2 RAM statement counts.
/// Matches the textbook closed form with all statement costs `c_k = 1`:
/// `T(n) = 4n - 3 + 3 * sum_{j=2..n} (t_j - 1) + sum_{j=2..n} t_j`? Wait:
/// let's keep the exact per-line CLRS §2.2 counts:
/// - line 1 (`for j = 2 to n`): `n` times
/// - line 2 (`key = A[j]`): `n - 1` times (1 mem read)
/// - line 4 (`i = j - 1`): `n - 1` times
/// - line 5 (`while i > 0 and A[i] > key`): `sum_{j=2..n} t_j` times
/// - line 6 (`A[i + 1] = A[i]`): `sum_{j=2..n} (t_j - 1)` times (1 read + 1 write)
/// - line 7 (`i = i - 1`): `sum_{j=2..n} (t_j - 1)` times
/// - line 8 (`A[i + 1] = key`): `n - 1` times (1 mem write)
pub fn insertion_sort_ram_profile(a: &mut [i32]) -> RamProfile {
    let n = a.len();
    if n == 0 {
        return RamProfile::default();
    }
    let mut prof = RamProfile {
        outer_checks: n, // loop header checked n times (enters n - 1, exits on j = n + 1)
        comparisons: 0,
        mem_reads: 0,
        mem_writes: 0,
        total_steps: n,
    };
    for j in 1..n {
        let key = a[j]; // line 2: 1 step, 1 read
        prof.mem_reads += 1;
        prof.total_steps += 1;
        // line 4 (`i = j - 1`): 1 step
        prof.total_steps += 1;
        let mut i = j;
        loop {
            // line 5 (`while i > 0 and A[i] > key`): 1 comparison step
            prof.comparisons += 1;
            prof.total_steps += 1;
            if i == 0 {
                break;
            }
            prof.mem_reads += 1;
            if a[i - 1] <= key {
                break;
            }
            // line 6 (`A[i + 1] = A[i]`): 1 step, 1 read, 1 write
            a[i] = a[i - 1];
            prof.mem_reads += 1;
            prof.mem_writes += 1;
            prof.total_steps += 1;
            // line 7 (`i = i - 1`): 1 step
            i -= 1;
            prof.total_steps += 1;
        }
        // line 8 (`A[i + 1] = key`): 1 step, 1 write
        a[i] = key;
        prof.mem_writes += 1;
        prof.total_steps += 1;
    }
    prof
}

/// Exact CLRS Section 2.2 unit-cost closed forms for best-case (`t_j = 1`) and worst-case (`t_j = j`)
/// execution of Insertion Sort on `n >= 1` items (with line cost constants `c_k = 1`).
pub fn insertion_sort_clrs_closed_form(n: usize) -> (usize, usize) {
    if n == 0 {
        return (0, 0);
    }
    let m = n - 1;
    // sum_{j=2..n} 1 = n - 1 -> t_j - 1 = 0
    let best = n + 3 * m + m; // 5n - 4
    // sum_{j=2..n} j = n(n+1)/2 - 1; sum (t_j - 1) = n(n-1)/2
    let sum_tj = n * (n + 1) / 2 - 1;
    let sum_shift = n * m / 2;
    let worst = n + 3 * m + sum_tj + 2 * sum_shift;
    (best, worst)
}

/// Simulate a direct-mapped cache with `num_lines` cache lines of `block_elems` elements each
/// for any sequence of element indices accessed in memory.
pub fn simulate_direct_mapped_cache(accesses: &[usize], block_elems: usize, num_lines: usize) -> usize {
    assert!(block_elems > 0 && num_lines > 0);
    let mut tags: Vec<Option<usize>> = vec![None; num_lines];
    let mut misses = 0;
    for &idx in accesses {
        let block = idx / block_elems;
        let slot = block % num_lines;
        if tags[slot] != Some(block) {
            tags[slot] = Some(block);
            misses += 1;
        }
    }
    misses
}

/// Compare scanning `n` elements sequentially in a contiguous array versus chasing a shuffled
/// pointer chain (linked list nodes scattered across memory), measuring RAM steps vs. cache misses.
pub fn compare_traversal_costs(n: usize, block_elems: usize, cache_lines: usize, seed: u64) -> (ModelCost, ModelCost) {
    let seq_accesses: Vec<usize> = (0..n).collect();
    let mut perm: Vec<usize> = (0..n).collect();
    let mut rng = Rng::new(seed);
    rng.shuffle(&mut perm);
    // Walk nodes via `next` links in memory order `perm`
    let array_cost = ModelCost {
        ram_steps: n,
        machine_mem_ops: n,
        cache_misses: simulate_direct_mapped_cache(&seq_accesses, block_elems, cache_lines),
    };
    let list_cost = ModelCost {
        ram_steps: n,
        machine_mem_ops: 2 * n, // load value + load `next` pointer
        cache_misses: simulate_direct_mapped_cache(&perm, block_elems, cache_lines),
    };
    (array_cost, list_cost)
}

/// Compare Word-RAM multiplication count vs. Bit-Complexity schoolbook bit operations
/// when computing `2^(2^k)` by `k` repeated squarings (CLRS Ch. 2.2 / Ch. 31 warning on unit-cost RAM).
pub fn repeated_squaring_bit_cost(k: u32) -> (u32, u64) {
    let word_ram_mults = k;
    let mut bits = 2u64; // starting value 2 fits in 2 bits
    let mut total_bit_ops = 0u64;
    for _ in 0..k {
        total_bit_ops += bits * bits;
        bits *= 2;
    }
    (word_ram_mults, total_bit_ops)
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut sorted: Vec<i32> = (0..32).collect();
    let mut reversed: Vec<i32> = (0..32).rev().collect();
    let p_best = insertion_sort_ram_profile(&mut sorted);
    let p_worst = insertion_sort_ram_profile(&mut reversed);
    outln!(o, "CLRS 2.2 RAM statement profile for INSERTION-SORT (n = 32, unit weights):");
    outln!(o, "  already sorted (best ): steps = {}, reads = {}, writes = {}", p_best.total_steps, p_best.mem_reads, p_best.mem_writes);
    outln!(o, "  reverse sorted (worst): steps = {}, reads = {}, writes = {}", p_worst.total_steps, p_worst.mem_reads, p_worst.mem_writes);

    let (arr, list) = compare_traversal_costs(256, 8, 8, 202);
    outln!(o, "Array vs Linked-List traversal (n = 256, cache = 8 lines x 8 ints):");
    outln!(o, "  Contiguous array: RAM = O(n) ({} steps), cache misses = {}", arr.ram_steps, arr.cache_misses);
    outln!(o, "  Pointer-chasing : RAM = O(n) ({} steps), cache misses = {} ({:.1}x more stalls!)",
        list.ram_steps, list.cache_misses, list.cache_misses as f64 / arr.cache_misses as f64);

    let (w_ops, bit_ops) = repeated_squaring_bit_cost(10);
    outln!(o, "Word-RAM vs Bit Complexity for 2^(2^10): {w_ops} multiplications vs {bit_ops} bit operations");
    o
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insertion_sort_profile_matches_clrs_closed_forms() {
        for n in 0..=30usize {
            let mut best_in: Vec<i32> = (0..n as i32).collect();
            let mut worst_in: Vec<i32> = (0..n as i32).rev().collect();
            let pb = insertion_sort_ram_profile(&mut best_in);
            let pw = insertion_sort_ram_profile(&mut worst_in);
            let (want_best, want_worst) = insertion_sort_clrs_closed_form(n);
            assert_eq!(pb.total_steps, want_best, "n={n} best");
            assert_eq!(pw.total_steps, want_worst, "n={n} worst");
            if n > 0 {
                assert_eq!(pb.comparisons, n - 1);
                assert_eq!(pb.mem_writes, n - 1);
                assert_eq!(pw.comparisons, n * (n + 1) / 2 - 1);
                assert_eq!(pw.mem_writes, n * (n - 1) / 2 + (n - 1));
            }
        }
    }

    #[test]
    fn array_traversal_beats_pointer_chasing_in_cache_misses() {
        let (arr, list) = compare_traversal_costs(512, 8, 8, 77);
        assert_eq!(arr.ram_steps, list.ram_steps);
        // Sequential array scan hits exactly one cold miss per 8-element cache line.
        assert_eq!(arr.cache_misses, 512 / 8);
        // Scattered pointer traversal misses far more often than spatial-locality array access.
        assert!(list.cache_misses > 5 * arr.cache_misses, "arr={} list={}", arr.cache_misses, list.cache_misses);
    }

    #[test]
    fn bit_complexity_grows_exponentially_relative_to_word_ram_count() {
        let (k4, b4) = repeated_squaring_bit_cost(4);
        let (k8, b8) = repeated_squaring_bit_cost(8);
        assert_eq!(k4, 4);
        assert_eq!(k8, 8);
        assert!(b8 > 200 * b4);
    }
}
