"""Parallel Fibonacci & Work-Span Analysis — CLRS 27.1 (3e) / 26.1 (4e) (Editions: 3,4).

spawn/sync Fibonacci, with work T_1, span T_inf and parallelism T_1/T_inf computed
from the computation DAG.
"""

from __future__ import annotations

from pathlib import Path
import sys
import threading
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def fib(n: int) -> int:
    """Serial Fibonacci (the baseline)."""
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)


def p_fib(n: int, cutoff: int) -> int:
    """P-FIB(n): spawn the first recursive call, compute the second in parent, then sync."""
    if n < 2:
        return n
    if n <= cutoff:
        return fib(n)
    res = [0]

    def run() -> None:
        res[0] = p_fib(n - 1, cutoff)

    t = threading.Thread(target=run)
    t.start()
    y = p_fib(n - 2, cutoff)
    t.join()
    return res[0] + y


def work(n: int) -> int:
    """Work T_1(n): total strands = number of calls."""
    if n < 2:
        return 1
    return work(n - 1) + work(n - 2) + 1


def span(n: int) -> int:
    """Span T_inf(n): longest dependency chain."""
    if n < 2:
        return 1
    return max(span(n - 1), span(n - 2)) + 1


def demo() -> str:
    lines: list[str] = []
    lines.append(f"P-FIB(30) = {p_fib(30, 20)} (serial {fib(30)})")
    lines.append("   n |      work T1 |  span T_inf | parallelism T1/T_inf")
    for n in [10, 20, 30, 40]:
        # Use DP for work/span at n=40 so demo runs instantaneously
        w_a, w_b = 1, 1
        s_a, s_b = 1, 1
        for _ in range(2, n + 1):
            w_a, w_b = w_b, w_a + w_b + 1
            s_a, s_b = s_b, max(s_a, s_b) + 1
        w_n = 1 if n < 2 else w_b
        s_n = 1 if n < 2 else s_b
        lines.append(f"{n:>4} | {w_n:>12} | {s_n:>11} | {w_n / s_n:>12.1f}")
    lines.append(
        "speedup bound on P processors: T_P >= max(T1/P, T_inf) -- plenty of parallelism since T1/T_inf grows like phi^n/n"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_parallel_matches_serial(self) -> None:
        for n in range(28):
            self.assertEqual(p_fib(n, 12), fib(n))
        self.assertEqual(p_fib(32, 20), 2178309)

    def test_work_and_span_recurrences(self) -> None:
        for n in range(2, 20):
            self.assertEqual(span(n), n)
        for n in range(20):
            self.assertEqual(work(n), 2 * fib(n + 1) - 1)
        # Verify work(30) / span(30) > 10_000 via closed form
        a, b = 0, 1
        for _ in range(31):
            a, b = b, a + b
        w30 = 2 * a - 1
        self.assertGreater(w30 // 30, 10_000)


if __name__ == "__main__":
    print(demo(), end="")
