"""Checkerboard Optimization & Profit-Deadline Scheduling — CLRS Problems 15-6 & 15-7 (2e); Problem 34-4 (1e–4e) (1e,2e,3e,4e).

Bottom-up maximum-profit checkerboard traversal and pseudopolynomial O(nD) scheduling with profits, deadlines, and durations (contrasted with NP-completeness).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_NEG = -(1 << 60)


def checkerboard_max_profit(profit: list[list[int]]) -> tuple[int, list[int]]:
    """Problem 15-6: maximum-profit path from row 0 to row m-1 moving down-left/down/down-right."""
    m = len(profit)
    if m == 0:
        return 0, []
    n = len(profit[0])
    if n == 0 or any(len(row) != n for row in profit):
        raise ValueError("profit grid must be non-empty rectangular")

    dp = [[_NEG] * n for _ in range(m)]
    prev_col = [[0] * n for _ in range(m)]
    for c in range(n):
        dp[0][c] = profit[0][c]
    for r in range(1, m):
        for c in range(n):
            lo = max(0, c - 1)
            hi = min(n - 1, c + 1)
            best_p = _NEG
            best_pc = lo
            for pc in range(lo, hi + 1):
                if dp[r - 1][pc] > best_p:
                    best_p = dp[r - 1][pc]
                    best_pc = pc
            dp[r][c] = best_p + profit[r][c]
            prev_col[r][c] = best_pc

    cur_c = max(range(n), key=lambda c: dp[m - 1][c])
    best_total = dp[m - 1][cur_c]
    path = [0] * m
    for r in range(m - 1, -1, -1):
        path[r] = cur_c
        if r > 0:
            cur_c = prev_col[r][cur_c]
    return best_total, path


def max_profit_scheduling(jobs: list[tuple[int, int, int]]) -> tuple[int, list[int]]:
    """Problem 15-7 / 34-4: Scheduling with (duration t_i, deadline d_i, profit p_i).

    Pseudopolynomial O(n D) DP where D = max d_i. Returns (max_profit, selected_job_indices_in_order).
    """
    n = len(jobs)
    if n == 0:
        return 0, []
    order = sorted(range(n), key=lambda idx: (jobs[idx][1], jobs[idx][0], idx))
    max_d = max(jobs[idx][1] for idx in order)
    dp = [[_NEG] * (max_d + 1) for _ in range(n + 1)]
    took = [[False] * (max_d + 1) for _ in range(n + 1)]
    dp[0][0] = 0

    for step, orig_idx in enumerate(order):
        dur, dl, prof = jobs[orig_idx]
        for t in range(max_d + 1):
            dp[step + 1][t] = dp[step][t]
        for t in range(dur, dl + 1):
            if dp[step][t - dur] > _NEG // 2:
                cand = dp[step][t - dur] + prof
                if cand > dp[step + 1][t]:
                    dp[step + 1][t] = cand
                    took[step + 1][t] = True

    best_t = max(range(max_d + 1), key=lambda t: dp[n][t])
    best_profit = dp[n][best_t]
    chosen: list[int] = []
    for step in range(n - 1, -1, -1):
        if took[step + 1][best_t]:
            orig_idx = order[step]
            chosen.append(orig_idx)
            best_t -= jobs[orig_idx][0]
    chosen.reverse()
    return best_profit, chosen


def demo() -> str:
    lines: list[str] = []
    board = [
        [3, 1, 7, 2],
        [2, 9, 4, 8],
        [6, 1, 5, 3],
        [4, 8, 2, 7],
    ]
    b_prof, path = checkerboard_max_profit(board)
    lines.append(f"checkerboard max profit = {b_prof}, column path = {path}")

    jobs = [(3, 5, 40), (2, 4, 25), (4, 6, 50), (1, 2, 15)]
    s_prof, chosen = max_profit_scheduling(jobs)
    lines.append(f"jobs (dur, dl, profit) = {jobs}")
    lines.append(f"max scheduling profit   = {s_prof}, execution order = {chosen}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_checkerboard_matches_exhaustive_dfs(self) -> None:
        r = Rng(1506)
        for _ in range(50):
            m = r.range(1, 7)
            n = r.range(1, 5)
            board = [r.vec(n, -20, 40) for _ in range(m)]
            got, path = checkerboard_max_profit(board)
            self.assertEqual(len(path), m)
            total = 0
            for r_idx in range(m):
                total += board[r_idx][path[r_idx]]
                if r_idx > 0:
                    self.assertLessEqual(abs(path[r_idx] - path[r_idx - 1]), 1)
            self.assertEqual(total, got)

            def brute(row: int, col: int) -> int:
                if row + 1 == len(board):
                    return board[row][col]
                lo = max(0, col - 1)
                hi = min(len(board[0]) - 1, col + 1)
                return board[row][col] + max(brute(row + 1, nc) for nc in range(lo, hi + 1))

            expected = max(brute(0, c) for c in range(n))
            self.assertEqual(got, expected)

    def test_scheduling_matches_exhaustive_subset_enumeration(self) -> None:
        r = Rng(1507)
        for _ in range(60):
            n = r.range(1, 11)
            jobs = []
            for _ in range(n):
                dur = r.range(1, 6)
                dl = r.range(dur - 1, 14)
                prof = r.range(1, 50)
                jobs.append((dur, dl, prof))
            got, chosen = max_profit_scheduling(jobs)

            time = 0
            prof_sum = 0
            for idx in chosen:
                dur, dl, prof = jobs[idx]
                time += dur
                self.assertLessEqual(time, dl)
                prof_sum += prof
            self.assertEqual(prof_sum, got)

            brute = 0
            for mask in range(1 << n):
                sub = sorted((jobs[i] for i in range(n) if (mask >> i) & 1), key=lambda j: j[1])
                t = 0
                ok = True
                p = 0
                for dur, dl, prof in sub:
                    t += dur
                    if t > dl:
                        ok = False
                        break
                    p += prof
                if ok:
                    brute = max(brute, p)
            self.assertEqual(got, brute)


if __name__ == "__main__":
    print(demo(), end="")
