"""Sorting in Place in Linear Time — CLRS Problem 8-2 (1e–4e).

Compare the three O(n) constrained-key trade-offs: O(n) worst-case, stability, and O(1) auxiliary space (in-place counting-sort cycle chasing).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def stable_linear_sort(items: list[tuple[int, object]], k: int) -> list[tuple[int, object]]:
    """(a) O(n + k) worst-case, stable, O(n + k) auxiliary space (counting sort)."""
    c = [0] * (k + 1)
    for key, _ in items:
        c[key] += 1
    for i in range(1, k + 1):
        c[i] += c[i - 1]
    out: list = [None] * len(items)
    for item in reversed(items):
        kx = item[0]
        c[kx] -= 1
        out[c[kx]] = item
    return out


def in_place_binary_partition(items: list[tuple[int, object]]) -> None:
    """(b) O(n) worst-case and O(1) auxiliary space for binary keys in {0, 1} (unstable Hoare partition)."""
    lo = 0
    hi = len(items)
    while lo < hi:
        while lo < hi and items[lo][0] == 0:
            lo += 1
        while lo < hi and items[hi - 1][0] != 0:
            hi -= 1
        if lo < hi:
            items[lo], items[hi - 1] = items[hi - 1], items[lo]
            lo += 1
            hi -= 1


def stable_in_place_binary_sort(items: list[tuple[int, object]]) -> None:
    """(c) Stable and O(1) auxiliary space for {0, 1} keys: insertion-like block shifting takes Θ(n²)."""
    write_zero = 0
    for i in range(len(items)):
        if items[i][0] == 0:
            val = items[i]
            items[write_zero + 1 : i + 1] = items[write_zero:i]
            items[write_zero] = val
            write_zero += 1


def in_place_counting_sort(items: list[tuple[int, object]], k: int) -> None:
    """(d) Unstable O(n + k) cycle-chasing counting sort using only O(k) count/cursor space."""
    n = len(items)
    if n <= 1:
        return
    counts = [0] * (k + 1)
    for key, _ in items:
        counts[key] += 1
    pos = [0] * (k + 1)
    limit = [0] * (k + 1)
    acc = 0
    for v in range(k + 1):
        pos[v] = acc
        acc += counts[v]
        limit[v] = acc
    for v in range(k + 1):
        while pos[v] < limit[v]:
            i = pos[v]
            kv = items[i][0]
            if kv == v:
                pos[v] += 1
            else:
                dst = pos[kv]
                pos[kv] += 1
                items[i], items[dst] = items[dst], items[i]


def in_place_msd_radix_sort(items: list[tuple[int, object]], bits: int) -> None:
    """(e) In-place MSD binary radix sort over `bits`-bit keys: O(b n) time, O(b) stack, unstable."""

    def recurse(lo_idx: int, hi_idx: int, bit: int) -> None:
        if hi_idx - lo_idx <= 1:
            return
        mask = 1 << bit
        lo = lo_idx
        hi = hi_idx
        while lo < hi:
            while lo < hi and (items[lo][0] & mask) == 0:
                lo += 1
            while lo < hi and (items[hi - 1][0] & mask) != 0:
                hi -= 1
            if lo < hi:
                items[lo], items[hi - 1] = items[hi - 1], items[lo]
                lo += 1
                hi -= 1
        if bit > 0:
            recurse(lo_idx, lo, bit - 1)
            recurse(lo, hi_idx, bit - 1)

    if bits > 0:
        recurse(0, len(items), bits - 1)


def demo() -> str:
    lines: list[str] = []
    sample: list[tuple[int, object]] = [(2, "a"), (0, "b"), (1, "c"), (2, "d"), (0, "e"), (1, "f")]
    lines.append(f"input records (key, tag): {sample}")
    lines.append(f"(a) stable O(n+k)       : {stable_linear_sort(sample, 2)}")
    c_in_place = list(sample)
    in_place_counting_sort(c_in_place, 2)
    lines.append(f"(d) in-place O(k)-space : {c_in_place}")
    bin_items: list[tuple[int, object]] = [(1, "p"), (0, "q"), (1, "r"), (0, "s"), (0, "t")]
    b_part = list(bin_items)
    in_place_binary_partition(b_part)
    b_stab = list(bin_items)
    stable_in_place_binary_sort(b_stab)
    lines.append(f"(b) 0-1 partition O(n)  : {b_part}")
    lines.append(f"(c) 0-1 stable O(1) spc : {b_stab}")
    msd: list[tuple[int, object]] = [(5, "a"), (2, "b"), (7, "c"), (1, "d"), (4, "e"), (2, "f")]
    in_place_msd_radix_sort(msd, 3)
    lines.append(f"(e) in-place MSD radix  : {msd}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_small_key_variants_match_sorted_order(self) -> None:
        r = Rng(8201)
        for k in [0, 1, 5, 12]:
            n = r.range(0, 80)
            recs: list[tuple[int, object]] = [(r.range(0, k), i) for i in range(n)]
            stab = stable_linear_sort(recs, k)
            self.assertTrue(
                all(
                    stab[i][0] < stab[i + 1][0]
                    or (stab[i][0] == stab[i + 1][0] and int(stab[i][1]) < int(stab[i + 1][1]))  # type: ignore[arg-type]
                    for i in range(len(stab) - 1)
                )
            )
            cyc = list(recs)
            in_place_counting_sort(cyc, k)
            self.assertTrue(all(cyc[i][0] <= cyc[i + 1][0] for i in range(len(cyc) - 1)))
            self.assertEqual(sorted(cyc), sorted(recs))

    def test_binary_and_msd_radix_variants(self) -> None:
        r = Rng(8202)
        for _ in range(60):
            n = r.range(0, 60)
            bin_items: list[tuple[int, object]] = [(r.range(0, 1), i) for i in range(n)]
            part = list(bin_items)
            in_place_binary_partition(part)
            self.assertTrue(all(part[i][0] <= part[i + 1][0] for i in range(len(part) - 1)))

            stab = list(bin_items)
            stable_in_place_binary_sort(stab)
            expected = sorted(bin_items, key=lambda x: x[0])
            self.assertEqual(stab, expected)

            bits = 10
            words: list[tuple[int, object]] = [(r.next_u64() & ((1 << bits) - 1), i) for i in range(n)]
            msd = list(words)
            in_place_msd_radix_sort(msd, bits)
            self.assertTrue(all(msd[i][0] <= msd[i + 1][0] for i in range(len(msd) - 1)))
            self.assertEqual(sorted(msd), sorted(words))


if __name__ == "__main__":
    print(demo(), end="")
