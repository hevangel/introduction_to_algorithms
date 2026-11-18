"""Parallel Scan, 2D Stencil & Parallel LU — CLRS Problems 27-3..27-5 (3e) / 26-3..26-5 (4e) (Editions: 3,4).

Work-efficient two-pass parallel prefix scan, divide-and-conquer quadrant stencil DP,
and parallel block LU decomposition.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import threading
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass(frozen=True)
class WorkSpan:
    work: int
    span: int


@dataclass
class _Tree:
    sum: int
    left: _Tree | None = None
    right: _Tree | None = None


def p_scan(x: list[int]) -> tuple[list[int], WorkSpan]:
    """P-SCAN (Problem 26-3 / 27-3): work-efficient two-pass parallel prefix sum.

    Work = Theta(n), Span = Theta(lg n).
    """
    n = len(x)
    if n == 0:
        return [], WorkSpan(work=0, span=0)
    if n == 1:
        return [x[0]], WorkSpan(work=1, span=1)

    def up(lo: int, hi: int) -> tuple[_Tree, WorkSpan]:
        if hi - lo == 1:
            return _Tree(sum=x[lo]), WorkSpan(work=1, span=1)
        mid = lo + (hi - lo) // 2
        if hi - lo >= 64:
            left_box: list[tuple[_Tree, WorkSpan] | None] = [None]

            def run_left() -> None:
                left_box[0] = up(lo, mid)

            t = threading.Thread(target=run_left)
            t.start()
            rt, rws = up(mid, hi)
            t.join()
            assert left_box[0] is not None
            lt, lws = left_box[0]
        else:
            lt, lws = up(lo, mid)
            rt, rws = up(mid, hi)
        return (
            _Tree(sum=lt.sum + rt.sum, left=lt, right=rt),
            WorkSpan(work=lws.work + rws.work + 1, span=max(lws.span, rws.span) + 1),
        )

    y = [0] * n

    def down(node: _Tree, prefix_before: int, lo: int, hi: int) -> WorkSpan:
        if node.left is None:
            y[lo] = prefix_before + node.sum
            return WorkSpan(work=1, span=1)
        assert node.right is not None
        mid = lo + (hi - lo) // 2
        right_base = prefix_before + node.left.sum
        if hi - lo >= 64:
            lws_box: list[WorkSpan | None] = [None]

            def run_left() -> None:
                assert node.left is not None
                lws_box[0] = down(node.left, prefix_before, lo, mid)

            t = threading.Thread(target=run_left)
            t.start()
            rws = down(node.right, right_base, mid, hi)
            t.join()
            assert lws_box[0] is not None
            lws = lws_box[0]
        else:
            lws = down(node.left, prefix_before, lo, mid)
            rws = down(node.right, right_base, mid, hi)
        return WorkSpan(work=lws.work + rws.work + 1, span=max(lws.span, rws.span) + 1)

    tree, up_ws = up(0, n)
    down_ws = down(tree, 0, 0, n)
    return y, WorkSpan(work=up_ws.work + down_ws.work, span=up_ws.span + down_ws.span)


def p_stencil_lcs(a: bytes | str, b: bytes | str) -> tuple[list[list[int]], WorkSpan]:
    """P-SIMPLE-STENCIL / Quadrant-Recursive 2D DP (Problem 26-4 / 27-4)."""
    m, n = len(a), len(b)
    dp = [[0] * (n + 1) for _ in range(m + 1)]

    def solve_rect(r0: int, r1: int, c0: int, c1: int) -> WorkSpan:
        rows = r1 - r0
        cols = c1 - c0
        if rows == 0 or cols == 0:
            return WorkSpan(work=0, span=0)
        if rows <= 8 or cols <= 8:
            for i in range(r0, r1):
                for j in range(c0, c1):
                    if a[i - 1] == b[j - 1]:
                        dp[i][j] = dp[i - 1][j - 1] + 1
                    else:
                        dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])
            return WorkSpan(work=rows * cols, span=rows + cols)
        rm = r0 + rows // 2
        cm = c0 + cols // 2
        nw = solve_rect(r0, rm, c0, cm)
        ne_res = solve_rect(r0, rm, cm, c1)
        sw_res = solve_rect(rm, r1, c0, cm)
        se = solve_rect(rm, r1, cm, c1)
        return WorkSpan(
            work=nw.work + ne_res.work + sw_res.work + se.work,
            span=nw.span + max(ne_res.span, sw_res.span) + se.span,
        )

    ws = solve_rect(1, m + 1, 1, n + 1)
    return dp, ws


def p_block_lu(a: list[list[float]]) -> tuple[list[list[float]], list[list[float]], WorkSpan] | None:
    """Parallel Block LU Decomposition without pivoting (Problem 26-5 / 27-5)."""
    n = len(a)
    if n == 0:
        return [], [], WorkSpan(work=0, span=0)
    if n <= 4:
        l = [[0.0] * n for _ in range(n)]
        u = [[0.0] * n for _ in range(n)]
        m = [list(row) for row in a]
        for k in range(n):
            if abs(m[k][k]) < 1e-12:
                return None
            l[k][k] = 1.0
            u[k][k] = m[k][k]
            for i in range(k + 1, n):
                l[i][k] = m[i][k] / m[k][k]
                u[k][i] = m[k][i]
            for i in range(k + 1, n):
                for j in range(k + 1, n):
                    m[i][j] -= l[i][k] * u[k][j]
        return l, u, WorkSpan(work=n * n * n, span=n * n)

    k = n // 2
    a11 = [[a[i][j] for j in range(k)] for i in range(k)]
    a12 = [[a[i][j] for j in range(k, n)] for i in range(k)]
    a21 = [[a[i][j] for j in range(k)] for i in range(k, n)]
    a22 = [[a[i][j] for j in range(k, n)] for i in range(k, n)]

    res11 = p_block_lu(a11)
    if res11 is None:
        return None
    l11, u11, ws11 = res11

    u12_box: list[list[list[float]] | None] = [None]

    def solve_u12() -> None:
        u12 = [[0.0] * (n - k) for _ in range(k)]
        for col in range(n - k):
            for i in range(k):
                total = a12[i][col]
                for p in range(i):
                    total -= l11[i][p] * u12[p][col]
                u12[i][col] = total
        u12_box[0] = u12

    t = threading.Thread(target=solve_u12)
    t.start()

    l21 = [[0.0] * k for _ in range(n - k)]
    for row in range(n - k):
        for j in range(k):
            total = a21[row][j]
            for p in range(j):
                total -= l21[row][p] * u11[p][j]
            l21[row][j] = total / u11[j][j]

    t.join()
    assert u12_box[0] is not None
    u12 = u12_box[0]

    for i in range(n - k):
        for j in range(n - k):
            prod = sum(l21[i][p] * u12[p][j] for p in range(k))
            a22[i][j] -= prod

    res22 = p_block_lu(a22)
    if res22 is None:
        return None
    l22, u22, ws22 = res22

    l = [[0.0] * n for _ in range(n)]
    u = [[0.0] * n for _ in range(n)]
    for i in range(k):
        for j in range(k):
            l[i][j] = l11[i][j]
            u[i][j] = u11[i][j]
        for j in range(n - k):
            u[i][k + j] = u12[i][j]
    for i in range(n - k):
        for j in range(k):
            l[k + i][j] = l21[i][j]
        for j in range(n - k):
            l[k + i][k + j] = l22[i][j]
            u[k + i][k + j] = u22[i][j]

    tri_work = 2 * k * k * (n - k)
    ws = WorkSpan(
        work=ws11.work + ws22.work + tri_work + k * (n - k) * (n - k),
        span=ws11.span + k + (n - k) + ws22.span,
    )
    return l, u, ws


def _lu_decomposition_ref(a: list[list[float]]) -> tuple[list[list[float]], list[list[float]]]:
    n = len(a)
    m = [list(row) for row in a]
    l = [[1.0 if i == j else 0.0 for j in range(n)] for i in range(n)]
    u = [[0.0] * n for _ in range(n)]
    for k in range(n):
        u[k][k] = m[k][k]
        for i in range(k + 1, n):
            l[i][k] = m[i][k] / u[k][k]
            u[k][i] = m[k][i]
        for i in range(k + 1, n):
            for j in range(k + 1, n):
                m[i][j] -= l[i][k] * u[k][j]
    return l, u


def demo() -> str:
    lines: list[str] = []
    xs = [3, 1, 4, 1, 5, 9, 2, 6]
    scan, sws = p_scan(xs)
    lines.append(f"P-SCAN({xs}) = {scan} (work={sws.work}, span={sws.span})")

    dp, lws = p_stencil_lcs(b"ABCBDAB", b"BDCABA")
    lines.append(
        f'P-STENCIL LCS("ABCBDAB", "BDCABA") = {dp[7][6]} (work={lws.work}, span={lws.span})'
    )

    mat = [
        [5.0, 1.0, 0.5, 0.2],
        [1.0, 6.0, 1.2, 0.4],
        [0.5, 1.2, 7.0, 1.0],
        [0.2, 0.4, 1.0, 8.0],
    ]
    res = p_block_lu(mat)
    assert res is not None
    _l, u, mws = res
    diag = [round(u[i][i] * 100.0) / 100.0 for i in range(4)]
    lines.append(f"P-BLOCK-LU diag(U) = {diag} (work={mws.work}, span={mws.span})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_p_scan_matches_serial_prefix_sums_and_log_span(self) -> None:
        r = Rng(2603)
        for n in [0, 1, 2, 7, 16, 63, 128]:
            xs = r.vec(n, -50, 50)
            got, ws = p_scan(xs)
            want: list[int] = []
            acc = 0
            for v in xs:
                acc += v
                want.append(acc)
            self.assertEqual(got, want)
            if n >= 2:
                self.assertLessEqual(ws.work, 4 * n)
                self.assertLessEqual(ws.span, 3 * (n.bit_length() - 1 + 2))

    def test_p_stencil_matches_serial_lcs_across_sizes(self) -> None:
        r = Rng(2604)
        for _ in range(20):
            m = r.range(1, 35)
            n = r.range(1, 35)
            a = bytes(ord("A") + r.range(0, 3) for _ in range(m))
            b = bytes(ord("A") + r.range(0, 3) for _ in range(n))
            got, ws = p_stencil_lcs(a, b)
            ser = [[0] * (n + 1) for _ in range(m + 1)]
            for i in range(1, m + 1):
                for j in range(1, n + 1):
                    if a[i - 1] == b[j - 1]:
                        ser[i][j] = ser[i - 1][j - 1] + 1
                    else:
                        ser[i][j] = max(ser[i - 1][j], ser[i][j - 1])
            self.assertEqual(got, ser)
            self.assertLessEqual(ws.span, m * n + m + n)

    def test_p_block_lu_matches_lu_decomposition(self) -> None:
        r = Rng(2605)
        for n in [2, 4, 8, 12, 16]:
            a = [[0.0] * n for _ in range(n)]
            for i in range(n):
                row_abs = 0.0
                for j in range(n):
                    if i != j:
                        a[i][j] = r.f64() * 2.0 - 1.0
                        row_abs += abs(a[i][j])
                a[i][i] = row_abs + 4.0
            res = p_block_lu(a)
            self.assertIsNotNone(res)
            assert res is not None
            l, u, _ = res
            rl, ru = _lu_decomposition_ref(a)
            for i in range(n):
                for j in range(n):
                    prod = sum(l[i][k] * u[k][j] for k in range(n))
                    self.assertAlmostEqual(prod, a[i][j], delta=1e-8)
                    self.assertAlmostEqual(l[i][j], rl[i][j], delta=1e-8)
                    self.assertAlmostEqual(u[i][j], ru[i][j], delta=1e-8)


if __name__ == "__main__":
    print(demo(), end="")
