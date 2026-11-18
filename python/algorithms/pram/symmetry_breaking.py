"""PRAM Work-Efficient Prefix & Symmetry Breaking — CLRS 30.4-30.5 (1e) (Editions: 1).

Deterministic coin-tossing 6-coloring in O(lg* n) rounds and O(1) reduction to
3-coloring plus randomized EREW list prefix.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def six_color_list(next_ptr: list[int | None]) -> tuple[list[int], int]:
    """SIX-COLOR(L): deterministic EREW PRAM 6-coloring of directed paths `next_ptr[u]`.

    Tail nodes (`next_ptr[u] is None`) serve as roots with fixed color 0; non-tails start at `u + 1`.
    Returns `(colors_in_0..6, rounds_taken)`.
    """
    n = len(next_ptr)
    if n == 0:
        return ([], 0)
    c = [0 if next_ptr[u] is None else u + 1 for u in range(n)]
    rounds = 0
    while any(col >= 6 for col in c):
        old = list(c)
        for u in range(n):
            v = next_ptr[u]
            if v is not None:
                diff = old[u] ^ old[v]
                k = (diff & -diff).bit_length() - 1
                bit = (old[u] >> k) & 1
                c[u] = 2 * k + bit
        rounds += 1
    return (c, rounds)


def three_color_from_six(next_ptr: list[int | None], c: list[int]) -> list[int]:
    """Reduce a valid 6-coloring of directed paths to a valid 3-coloring in 3 parallel rounds."""
    n = len(next_ptr)
    c = list(c)
    prev: list[int | None] = [None] * n
    for u in range(n):
        v = next_ptr[u]
        if v is not None:
            prev[v] = u
    for target in [5, 4, 3]:
        old = list(c)
        for u in range(n):
            if old[u] == target:
                p_col = old[prev[u]] if prev[u] is not None else None
                n_col = old[next_ptr[u]] if next_ptr[u] is not None else None
                c[u] = next(cand for cand in range(3) if cand != p_col and cand != n_col)
    return c


def mis_from_three_coloring(next_ptr: list[int | None], c: list[int]) -> list[bool]:
    """Greedily extract a maximal independent set (MIS) from a valid 3-coloring in 3 rounds."""
    n = len(next_ptr)
    prev: list[int | None] = [None] * n
    for u in range(n):
        v = next_ptr[u]
        if v is not None:
            prev[v] = u
    in_mis = [False] * n
    for color in range(3):
        for u in range(n):
            if c[u] == color:
                p_in = prev[u] is not None and in_mis[prev[u]]
                n_in = next_ptr[u] is not None and in_mis[next_ptr[u]]
                if not p_in and not n_in:
                    in_mis[u] = True
    return in_mis


def randomized_list_prefix(next_ptr: list[int | None], vals: list[int], seed: int) -> list[int]:
    """RANDOMIZED-LIST-PREFIX: work-efficient EREW prefix sums by random-mate contraction."""
    n = len(next_ptr)
    assert n == len(vals)
    if n == 0:
        return []
    rng = Rng(seed)
    cur_next = list(next_ptr)
    cur_val = list(vals)
    active = [True] * n
    active_count = n
    splice_log: list[list[tuple[int, int | None, int]]] = []
    lg_n = max(n.bit_length() - 1, 1)
    threshold = max(n // lg_n, 2)

    while active_count > threshold:
        prev: list[int | None] = [None] * n
        for u in range(n):
            if active[u]:
                v = cur_next[u]
                if v is not None:
                    prev[v] = u
        coin = [(rng.next_u64() & 1) == 0 for _ in range(n)]
        step_log: list[tuple[int, int | None, int]] = []
        for u in range(n):
            if not active[u] or not coin[u]:
                continue
            v = cur_next[u]
            if v is not None and active[v] and not coin[v] and cur_next[v] is not None:
                step_log.append((v, prev[u], u))
        if not step_log:
            continue
        round_records: list[tuple[int, int | None, int]] = []
        for v, _pu, u in step_log:
            after_v = cur_next[v]
            cur_val[u] += cur_val[v]
            cur_next[u] = after_v
            active[v] = False
            active_count -= 1
            round_records.append((v, u, v))
        splice_log.append(round_records)

    rev_prev: list[int | None] = [None] * n
    for u in range(n):
        if active[u]:
            v = cur_next[u]
            if v is not None:
                rev_prev[v] = u
    prefix = [0] * n
    for start in range(n):
        if active[start] and rev_prev[start] is None:
            acc = 0
            cur: int | None = start
            while cur is not None:
                acc += cur_val[cur]
                prefix[cur] = acc
                cur = cur_next[cur]

    for round_records in reversed(splice_log):
        for v, u_opt, _ in round_records:
            assert u_opt is not None
            total_through_v = prefix[u_opt]
            prefix[v] = total_through_v
            prefix[u_opt] = total_through_v - cur_val[v]
    return prefix


def demo() -> str:
    lines: list[str] = []
    n = 16
    nxt: list[int | None] = [i + 1 if i + 1 < n else None for i in range(n)]
    c6, rounds = six_color_list(nxt)
    c3 = three_color_from_six(nxt, c6)
    mis = mis_from_three_coloring(nxt, c3)
    lines.append(f"n={n} path 6-colored in {rounds} deterministic coin-tossing rounds:")
    lines.append(f"  c6  = {c6}")
    lines.append(f"  c3  = {c3}")
    lines.append(f"  MIS = {[i for i, b in enumerate(mis) if b]}")
    vals = list(range(1, n + 1))
    pref = randomized_list_prefix(nxt, vals, 304)
    lines.append(f"randomized list prefix(1..={n}): {pref}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_six_and_three_coloring_are_proper_and_lg_star_fast(self) -> None:
        r = Rng(305)
        for n in [1, 2, 7, 32, 200, 500]:
            order = list(range(n))
            r.shuffle(order)
            nxt: list[int | None] = [None] * n
            for i in range(n - 1):
                nxt[order[i]] = order[i + 1]
            c6, rounds = six_color_list(nxt)
            self.assertLessEqual(rounds, 6, f"O(lg* n) rounds took {rounds} for n={n}")
            self.assertTrue(all(col < 6 for col in c6))
            for u in range(n):
                v = nxt[u]
                if v is not None:
                    self.assertNotEqual(c6[u], c6[v])
            c3 = three_color_from_six(nxt, c6)
            self.assertTrue(all(col < 3 for col in c3))
            for u in range(n):
                v = nxt[u]
                if v is not None:
                    self.assertNotEqual(c3[u], c3[v])
            mis = mis_from_three_coloring(nxt, c3)
            for u in range(n):
                v = nxt[u]
                if v is not None:
                    self.assertFalse(mis[u] and mis[v])

    def test_randomized_list_prefix_matches_sequential_walk(self) -> None:
        r = Rng(306)
        for n in [1, 5, 19, 64, 150]:
            order = list(range(n))
            r.shuffle(order)
            nxt: list[int | None] = [None] * n
            for i in range(n - 1):
                nxt[order[i]] = order[i + 1]
            vals = r.vec(n, -20, 20)
            got = randomized_list_prefix(nxt, vals, 77 + n)
            expected = [0] * n
            acc = 0
            for u in order:
                acc += vals[u]
                expected[u] = acc
            self.assertEqual(got, expected)


if __name__ == "__main__":
    print(demo(), end="")
