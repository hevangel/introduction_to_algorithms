"""Assembly-Line Scheduling — CLRS 15.1 (2e) (2e).

The original DP example: fastest way through a two-line factory with transfer costs.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def fastest_way(
    a: list[list[int]] | tuple[list[int], list[int]],
    t: list[list[int]] | tuple[list[int], list[int]],
    e: list[int] | tuple[int, int],
    x: list[int] | tuple[int, int],
) -> tuple[int, list[int]]:
    """FASTEST-WAY: returns (fastest total time, line used at each station)."""
    n = len(a[0])
    f = [[0] * n, [0] * n]
    l = [[0] * n, [0] * n]
    f[0][0] = e[0] + a[0][0]
    f[1][0] = e[1] + a[1][0]
    for j in range(1, n):
        for i in range(2):
            o = 1 - i
            stay = f[i][j - 1] + a[i][j]
            switch = f[o][j - 1] + t[o][j - 1] + a[i][j]
            if stay <= switch:
                f[i][j] = stay
                l[i][j] = i
            else:
                f[i][j] = switch
                l[i][j] = o
    if f[0][n - 1] + x[0] <= f[1][n - 1] + x[1]:
        best, line = f[0][n - 1] + x[0], 0
    else:
        best, line = f[1][n - 1] + x[1], 1
    path = [line] * n
    for j in range(n - 1, 0, -1):
        line = l[line][j]
        path[j - 1] = line
    return best, path


def demo() -> str:
    lines: list[str] = []
    a1 = [7, 9, 3, 4, 8, 4]
    a2 = [8, 5, 6, 4, 5, 7]
    t1 = [2, 3, 1, 3, 4]
    t2 = [2, 1, 2, 2, 1]
    best, path = fastest_way([a1, a2], [t1, t2], [2, 4], [3, 2])
    lines.append(f"fastest time through the factory = {best}")
    for j, line in enumerate(path):
        lines.append(f"  station S{line + 1},{j + 1}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(
        self,
        a: list[list[int]],
        t: list[list[int]],
        e: list[int],
        x: list[int],
    ) -> int:
        n = len(a[0])
        best = sys.maxsize
        for mask in range(1 << n):
            line0 = (mask >> 0) & 1
            total = e[line0] + a[line0][0]
            prev = line0
            for j in range(1, n):
                cur = (mask >> j) & 1
                if cur != prev:
                    total += t[prev][j - 1]
                total += a[cur][j]
                prev = cur
            total += x[prev]
            best = min(best, total)
        return best

    def test_book_example(self) -> None:
        a1 = [7, 9, 3, 4, 8, 4]
        a2 = [8, 5, 6, 4, 5, 7]
        t1 = [2, 3, 1, 3, 4]
        t2 = [2, 1, 2, 2, 1]
        best, path = fastest_way([a1, a2], [t1, t2], [2, 4], [3, 2])
        self.assertEqual(best, 38)
        self.assertEqual(path, [0, 1, 0, 1, 1, 0])

    def test_matches_brute_force(self) -> None:
        r = Rng(100)
        for n in range(1, 10):
            a1, a2 = r.vec(n, 1, 9), r.vec(n, 1, 9)
            t1, t2 = r.vec(max(n, 1) - 1, 1, 5), r.vec(max(n, 1) - 1, 1, 5)
            e = [r.vec(1, 1, 5)[0], r.vec(1, 1, 5)[0]]
            x = [r.vec(1, 1, 5)[0], r.vec(1, 1, 5)[0]]
            self.assertEqual(fastest_way([a1, a2], [t1, t2], e, x)[0], self._brute([a1, a2], [t1, t2], e, x))


if __name__ == "__main__":
    print(demo(), end="")
