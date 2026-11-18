"""Two-Dimensional Pattern Matching (Rabin-Karp) — CLRS Exercise 32.2-3 (1e–4e) (1,2,3,4).

Find an m×m pattern in an n×n text array with a rolling hash along rows and then along columns: O(n² + m²) expected plus verification.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_MOD = 1_000_000_007
_B = 257


def match_2d(text: list[bytes], pattern: list[bytes]) -> list[tuple[int, int]]:
    """All (row, col) positions where the m×m `pattern` occurs in `text`."""
    n_rows, m = len(text), len(pattern)
    if m == 0 or n_rows < m or len(text[0]) < m:
        return []
    n_cols = len(text[0])
    bm = pow(_B, m, _MOD)

    def row_hash(row: bytes) -> list[int]:
        h = 0
        for c in row[:m]:
            h = (h * _B + c) % _MOD
        out = [h]
        for c in range(1, len(row) - m + 1):
            h = (h * _B - row[c - 1] * bm + row[c + m - 1]) % _MOD
            out.append(h)
        return out

    th = [row_hash(r) for r in text]
    ph = 0
    for r in pattern:
        ph = (ph * _B + row_hash(r)[0]) % _MOD

    out: list[tuple[int, int]] = []
    for c in range(n_cols - m + 1):
        h = 0
        for r in range(m):
            h = (h * _B + th[r][c]) % _MOD
        for r in range(n_rows - m + 1):
            if r > 0:
                h = (h * _B - th[r - 1][c] * bm + th[r + m - 1][c]) % _MOD
            if h == ph and all(text[r + i][c : c + m] == pattern[i] for i in range(m)):
                out.append((r, c))
    out.sort()
    return out


def demo() -> str:
    lines: list[str] = []
    text = [s.encode() for s in ["abcabc", "bcabca", "abcabc", "cabcab", "abcabc", "bcabca"]]
    pat = [s.encode() for s in ["ca", "ab"]]
    lines.append("text:")
    for r in text:
        lines.append(f"  {r.decode()}")
    lines.append(f"2×2 pattern ca/ab occurs at (row, col): {match_2d(text, pat)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _naive(self, t: list[bytes], p: list[bytes]) -> list[tuple[int, int]]:
        n, m = len(t), len(p)
        out: list[tuple[int, int]] = []
        if n >= m and t and len(t[0]) >= m:
            for r in range(n - m + 1):
                for c in range(len(t[0]) - m + 1):
                    if all(t[r + i][c : c + m] == p[i] for i in range(m)):
                        out.append((r, c))
        return out

    def test_agrees_with_naive_on_small_alphabets(self) -> None:
        r = Rng(403)
        for _ in range(300):
            rows, cols, m = r.range(1, 9), r.range(1, 9), r.range(1, 3)
            alpha = r.range(1, 2)
            text = [bytes(ord("a") + r.range(0, alpha) for _ in range(cols)) for _ in range(rows)]
            pat = [bytes(ord("a") + r.range(0, alpha) for _ in range(m)) for _ in range(m)]
            self.assertEqual(match_2d(text, pat), self._naive(text, pat))

    def test_planted_pattern_is_found(self) -> None:
        r = Rng(404)
        text = [bytearray(ord("a") + r.range(0, 3) for _ in range(40)) for _ in range(40)]
        pat = [bytes(ord("a") + r.range(0, 25) for _ in range(4)) for _ in range(4)]
        for i in range(4):
            text[17 + i][9:13] = pat[i]
        self.assertIn((17, 9), match_2d([bytes(row) for row in text], pat))


if __name__ == "__main__":
    print(demo(), end="")
