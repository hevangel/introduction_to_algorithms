"""Computational Models & the RAM Model — CLRS 2.2 (1e–4e) (1,2,3,4).

Lesson 0: the sequential Random-Access Machine (RAM) abstract computer, statement-by-statement cost counting, cache/pointer-chasing gaps, and alternative models (Word-RAM, I/O, PRAM, work-span, circuits).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass(eq=True)
class RamProfile:
    outer_checks: int = 0
    comparisons: int = 0
    mem_reads: int = 0
    mem_writes: int = 0
    total_steps: int = 0


@dataclass(eq=True)
class ModelCost:
    ram_steps: int
    machine_mem_ops: int
    cache_misses: int


def insertion_sort_ram_profile(a: list[int]) -> RamProfile:
    """Run CLRS INSERTION-SORT while recording exact Section 2.2 RAM statement counts."""
    n = len(a)
    if n == 0:
        return RamProfile()
    prof = RamProfile(outer_checks=n, total_steps=n)
    for j in range(1, n):
        key = a[j]
        prof.mem_reads += 1
        prof.total_steps += 1
        prof.total_steps += 1  # i = j - 1
        i = j
        while True:
            prof.comparisons += 1
            prof.total_steps += 1
            if i == 0:
                break
            prof.mem_reads += 1
            if a[i - 1] <= key:
                break
            a[i] = a[i - 1]
            prof.mem_reads += 1
            prof.mem_writes += 1
            prof.total_steps += 1
            i -= 1
            prof.total_steps += 1
        a[i] = key
        prof.mem_writes += 1
        prof.total_steps += 1
    return prof


def insertion_sort_clrs_closed_form(n: int) -> tuple[int, int]:
    """Exact CLRS Section 2.2 unit-cost closed forms for best-case and worst-case INSERTION-SORT."""
    if n == 0:
        return (0, 0)
    m = n - 1
    best = n + 4 * m
    sum_tj = n * (n + 1) // 2 - 1
    sum_shift = n * m // 2
    worst = n + 3 * m + sum_tj + 2 * sum_shift
    return (best, worst)


def simulate_direct_mapped_cache(accesses: list[int], block_elems: int, num_lines: int) -> int:
    """Simulate a direct-mapped cache for a sequence of element indices."""
    assert block_elems > 0 and num_lines > 0
    tags: list[int | None] = [None] * num_lines
    misses = 0
    for idx in accesses:
        block = idx // block_elems
        slot = block % num_lines
        if tags[slot] != block:
            tags[slot] = block
            misses += 1
    return misses


def compare_traversal_costs(
    n: int, block_elems: int, cache_lines: int, seed: int
) -> tuple[ModelCost, ModelCost]:
    """Compare contiguous array scan vs. shuffled pointer-chasing traversal."""
    seq = list(range(n))
    perm = list(range(n))
    Rng(seed).shuffle(perm)
    arr = ModelCost(
        ram_steps=n,
        machine_mem_ops=n,
        cache_misses=simulate_direct_mapped_cache(seq, block_elems, cache_lines),
    )
    lst = ModelCost(
        ram_steps=n,
        machine_mem_ops=2 * n,
        cache_misses=simulate_direct_mapped_cache(perm, block_elems, cache_lines),
    )
    return (arr, lst)


def repeated_squaring_bit_cost(k: int) -> tuple[int, int]:
    """Compare Word-RAM multiplications vs. Bit-Complexity schoolbook ops for 2^(2^k)."""
    bits = 2
    total_bit_ops = 0
    for _ in range(k):
        total_bit_ops += bits * bits
        bits *= 2
    return (k, total_bit_ops)


def demo() -> str:
    sorted_a = list(range(32))
    reversed_a = list(range(31, -1, -1))
    p_best = insertion_sort_ram_profile(sorted_a)
    p_worst = insertion_sort_ram_profile(reversed_a)
    arr, lst = compare_traversal_costs(256, 8, 8, 202)
    w_ops, bit_ops = repeated_squaring_bit_cost(10)
    return "\n".join([
        "CLRS 2.2 RAM statement profile for INSERTION-SORT (n = 32, unit weights):",
        f"  already sorted (best ): steps = {p_best.total_steps}, reads = {p_best.mem_reads}, writes = {p_best.mem_writes}",
        f"  reverse sorted (worst): steps = {p_worst.total_steps}, reads = {p_worst.mem_reads}, writes = {p_worst.mem_writes}",
        "Array vs Linked-List traversal (n = 256, cache = 8 lines x 8 ints):",
        f"  Contiguous array: RAM = O(n) ({arr.ram_steps} steps), cache misses = {arr.cache_misses}",
        f"  Pointer-chasing : RAM = O(n) ({lst.ram_steps} steps), cache misses = {lst.cache_misses} "
        f"({lst.cache_misses / arr.cache_misses:.1f}x more stalls!)",
        f"Word-RAM vs Bit Complexity for 2^(2^10): {w_ops} multiplications vs {bit_ops} bit operations",
    ]) + "\n"


class Tests(unittest.TestCase):
    def test_insertion_sort_profile_matches_clrs_closed_forms(self) -> None:
        for n in range(31):
            pb = insertion_sort_ram_profile(list(range(n)))
            pw = insertion_sort_ram_profile(list(range(n - 1, -1, -1)))
            want_best, want_worst = insertion_sort_clrs_closed_form(n)
            self.assertEqual(pb.total_steps, want_best)
            self.assertEqual(pw.total_steps, want_worst)
            if n > 0:
                self.assertEqual(pb.comparisons, n - 1)
                self.assertEqual(pb.mem_writes, n - 1)
                self.assertEqual(pw.comparisons, n * (n + 1) // 2 - 1)
                self.assertEqual(pw.mem_writes, n * (n - 1) // 2 + (n - 1))

    def test_array_traversal_beats_pointer_chasing_in_cache_misses(self) -> None:
        arr, lst = compare_traversal_costs(512, 8, 8, 77)
        self.assertEqual(arr.ram_steps, lst.ram_steps)
        self.assertEqual(arr.cache_misses, 512 // 8)
        self.assertGreater(lst.cache_misses, 5 * arr.cache_misses)

    def test_bit_complexity_grows_exponentially_relative_to_word_ram_count(self) -> None:
        k4, b4 = repeated_squaring_bit_cost(4)
        k8, b8 = repeated_squaring_bit_cost(8)
        self.assertEqual(k4, 4)
        self.assertEqual(k8, 8)
        self.assertGreater(b8, 200 * b4)


if __name__ == "__main__":
    print(demo(), end="")
