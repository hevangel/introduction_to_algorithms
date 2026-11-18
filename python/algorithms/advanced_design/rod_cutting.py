"""Rod Cutting — CLRS 15.1 (3e) / 14.1 (4e); 1e–2e had assembly-line instead (3e,4e).

The introductory dynamic program: naive recursion, memoization, bottom-up and solution reconstruction.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

CLRS_PRICES = [0, 1, 5, 8, 9, 10, 17, 17, 20, 24, 30]


def cut_rod(p: list[int], n: int) -> int:
    """CUT-ROD(p, n): plain recursion, Theta(2^n). p[i] is the price of length i (p[0] = 0)."""
    if n == 0:
        return 0
    return max(p[i] + cut_rod(p, n - i) for i in range(1, n + 1))


def memoized_cut_rod(p: list[int], n: int) -> int:
    """MEMOIZED-CUT-ROD: top-down with a table, Theta(n^2)."""
    r: list[int | None] = [None] * (n + 1)

    def aux(k: int) -> int:
        if r[k] is not None:
            return r[k]  # type: ignore[return-value]
        q = 0 if k == 0 else max(p[i] + aux(k - i) for i in range(1, k + 1))
        r[k] = q
        return q

    return aux(n)


def bottom_up_cut_rod(p: list[int], n: int) -> int:
    """BOTTOM-UP-CUT-ROD: solve subproblems in order of size, Theta(n^2)."""
    r = [0] * (n + 1)
    for j in range(1, n + 1):
        r[j] = max(p[i] + r[j - i] for i in range(1, j + 1))
    return r[n]


def extended_bottom_up_cut_rod(p: list[int], n: int) -> tuple[list[int], list[int]]:
    """EXTENDED-BOTTOM-UP-CUT-ROD: returns revenues r and first-piece sizes s."""
    r = [0] * (n + 1)
    s = [0] * (n + 1)
    for j in range(1, n + 1):
        q = -(1 << 60)
        for i in range(1, j + 1):
            if q < p[i] + r[j - i]:
                q = p[i] + r[j - i]
                s[j] = i
        r[j] = q
    return r, s


def cut_rod_solution(p: list[int], n: int) -> list[int]:
    """PRINT-CUT-ROD-SOLUTION: the sizes of the pieces in an optimal cut."""
    _, s = extended_bottom_up_cut_rod(p, n)
    pieces: list[int] = []
    while n > 0:
        pieces.append(s[n])
        n -= s[n]
    return pieces


def cut_rod_with_cost(p: list[int], n: int, c: int) -> int:
    """Exercise 15.1-3: each cut costs c."""
    r = [0] * (n + 1)
    for j in range(1, n + 1):
        r[j] = max(p[i] + r[j - i] - (c if i < j else 0) for i in range(1, j + 1))
    return r[n]


def demo() -> str:
    lines: list[str] = []
    lines.append(f"prices p[1..10] = {CLRS_PRICES[1:]}")
    lines.append(" n | r_n | optimal pieces")
    for n in range(1, 11):
        lines.append(f"{n:>2} | {bottom_up_cut_rod(CLRS_PRICES, n):>3} | {cut_rod_solution(CLRS_PRICES, n)}")
    lines.append(
        f"naive == memoized == bottom-up for n=10: "
        f"{cut_rod(CLRS_PRICES, 10) == memoized_cut_rod(CLRS_PRICES, 10)}"
    )
    lines.append(f"with a cut cost of 2, n=7 yields {cut_rod_with_cost(CLRS_PRICES, 7, 2)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_values(self) -> None:
        expected = [0, 1, 5, 8, 10, 13, 17, 18, 22, 25, 30]
        for n in range(11):
            self.assertEqual(cut_rod(CLRS_PRICES, n), expected[n])
            self.assertEqual(memoized_cut_rod(CLRS_PRICES, n), expected[n])
            self.assertEqual(bottom_up_cut_rod(CLRS_PRICES, n), expected[n])

    def test_reconstruction_is_consistent(self) -> None:
        for n in range(1, 11):
            pieces = cut_rod_solution(CLRS_PRICES, n)
            self.assertEqual(sum(pieces), n)
            self.assertEqual(sum(CLRS_PRICES[i] for i in pieces), bottom_up_cut_rod(CLRS_PRICES, n))
        self.assertEqual(cut_rod_solution(CLRS_PRICES, 7), [1, 6])

    def test_cut_cost_reduces_revenue(self) -> None:
        self.assertEqual(cut_rod_with_cost(CLRS_PRICES, 10, 0), 30)
        self.assertEqual(cut_rod_with_cost(CLRS_PRICES, 4, 1), 9)
        self.assertLess(cut_rod_with_cost(CLRS_PRICES, 7, 3), 18)


if __name__ == "__main__":
    print(demo(), end="")
