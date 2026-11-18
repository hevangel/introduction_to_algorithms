"""Parallel Matrix-Vector Product (P-MAT-VEC) — CLRS 27.1 (3e) / 26.1 (4e) (Editions: 3,4).

A parallel for over rows gives work Theta(n^2) and span Theta(lg n + n); making the inner
sum a parallel reduction brings span to Theta(lg n). Includes a determinacy-race demo.
"""

from __future__ import annotations

from pathlib import Path
import sys
import threading
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def p_mat_vec(a: list[list[float]], x: list[float], workers: int) -> list[float]:
    """P-MAT-VEC: every row is an independent strand, executed by `workers` threads over row blocks."""
    n = len(a)
    y = [0.0] * n
    w = max(workers, 1)
    chunk = max((n + w - 1) // w, 1)

    def run_chunk(start: int, end: int) -> None:
        for i in range(start, end):
            y[i] = sum(p * q for p, q in zip(a[i], x))

    threads: list[threading.Thread] = []
    for start in range(0, n, chunk):
        end = min(start + chunk, n)
        t = threading.Thread(target=run_chunk, args=(start, end))
        t.start()
        threads.append(t)
    for t in threads:
        t.join()
    return y


def _next_pow2_log2(n: int) -> int:
    p = 1
    k = 0
    while p < n:
        p *= 2
        k += 1
    return k


def work_span_serial_rows(n: int) -> tuple[int, int]:
    """Work and span of P-MAT-VEC with serial row dot products."""
    return (n * n, n + _next_pow2_log2(n))


def work_span_parallel_rows(n: int) -> tuple[int, int]:
    """Work and span with parallel reduction tree per row."""
    return (n * n, 2 * _next_pow2_log2(n) + 1)


def racy_accumulate(contrib: list[float]) -> float:
    """Replay one interleaving of P-MAT-VEC-RACE showing lost updates."""
    reads = [0.0] * len(contrib)
    writes = [r + c for r, c in zip(reads, contrib)]
    return writes[-1] if writes else 0.0


def demo() -> str:
    lines: list[str] = []
    a = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 10.0]]
    lines.append(f"A*(1,1,1) with 2 workers = {p_mat_vec(a, [1.0, 1.0, 1.0], 2)}")
    for n in [8, 64, 1024]:
        w, s1 = work_span_serial_rows(n)
        _, s2 = work_span_parallel_rows(n)
        lines.append(
            f"n = {n:>5}: work {w:>8}; span {s1:>5} (serial dot products, parallelism {w / s1:.1f}) / "
            f"{s2} (tree reduction, parallelism {w / s2:.1f})"
        )
    lines.append(
        f"racy version: contributions [1,2,3,4] accumulate to {racy_accumulate([1.0, 2.0, 3.0, 4.0])} instead of 10"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_matches_serial_product_for_any_worker_count(self) -> None:
        r = Rng(430)
        for _ in range(50):
            n = r.range(1, 30)
            a = [[r.f64() for _ in range(n)] for _ in range(n)]
            x = [r.f64() for _ in range(n)]
            want = [sum(p * q for p, q in zip(row, x)) for row in a]
            for w in [1, 2, 3, 8, 100]:
                self.assertEqual(p_mat_vec(a, x, w), want)

    def test_span_formulas_and_race(self) -> None:
        self.assertEqual(work_span_serial_rows(8), (64, 11))
        self.assertLess(work_span_parallel_rows(1024)[1], work_span_serial_rows(1024)[1])
        self.assertEqual(racy_accumulate([1.0, 2.0, 3.0, 4.0]), 4.0)


if __name__ == "__main__":
    print(demo(), end="")
