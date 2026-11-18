"""Printing Neatly — CLRS Problem 15-4 (2e–3e), 14-4 (4e) (2e,3e,4e).

Break a paragraph into lines minimising the sum of cubes of trailing blanks: DP over line breaks.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def print_neatly(lens: list[int], m: int) -> tuple[int, list[int]] | None:
    """Minimum cost and number of words on each line. The last line costs nothing."""
    n = len(lens)
    if any(l > m for l in lens):
        return None
    inf = sys.maxsize
    c = [inf] * (n + 1)
    brk = [0] * (n + 1)
    c[0] = 0
    for j in range(1, n + 1):
        width = 0
        for i in range(j - 1, -1, -1):
            width += lens[i] + (1 if i + 1 < j else 0)
            if width > m:
                break
            extra = m - width
            cost = 0 if j == n else extra**3
            if c[i] != inf and c[i] + cost < c[j]:
                c[j] = c[i] + cost
                brk[j] = i
    lines: list[int] = []
    j = n
    while j > 0:
        lines.append(j - brk[j])
        j = brk[j]
    lines.reverse()
    return c[n], lines


def layout(words: list[str], m: int) -> list[str] | None:
    """Typeset words with the computed breaks."""
    lens = [len(w) for w in words]
    res = print_neatly(lens, m)
    if res is None:
        return None
    _, line_counts = res
    out: list[str] = []
    i = 0
    for k in line_counts:
        out.append(" ".join(words[i : i + k]))
        i += k
    return out


def demo() -> str:
    lines: list[str] = []
    words = "Dynamic programming solves problems by combining the solutions to overlapping subproblems".split(" ")
    for m in (20, 30):
        lens = [len(w) for w in words]
        res = print_neatly(lens, m)
        assert res is not None
        cost, _ = res
        lines.append(f"line width {m}: total cost {cost}")
        laid = layout(words, m)
        assert laid is not None
        for l in laid:
            lines.append(f"  |{l:<{m}}|")
    lines.append(f"a word longer than the line cannot be placed: {print_neatly([10], 5)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _brute(self, lens: list[int], m: int) -> int | None:
        n = len(lens)
        best: int | None = None
        for mask in range(1 << max(n - 1, 0)):
            cost = 0
            ok = True
            width = 0
            count = 0
            for i in range(n):
                width += lens[i] + (1 if count > 0 else 0)
                count += 1
                if width > m:
                    ok = False
                    break
                end = i == n - 1 or bool((mask >> i) & 1)
                if end:
                    if i != n - 1:
                        cost += (m - width) ** 3
                    width = 0
                    count = 0
            if ok and (best is None or cost < best):
                best = cost
        return best

    def test_matches_exhaustive_search(self) -> None:
        r = Rng(109)
        for _ in range(120):
            n = r.range(1, 9)
            lens = [r.range(1, 6) for _ in range(n)]
            m = r.range(6, 14)
            got = print_neatly(lens, m)
            self.assertEqual(got[0] if got is not None else None, self._brute(lens, m), f"{lens} m={m}")

    def test_layout_respects_width(self) -> None:
        words = ["a", "bb", "ccc", "dddd", "eeeee", "ff", "g"]
        laid = layout(words, 8)
        assert laid is not None
        for l in laid:
            self.assertLessEqual(len(l), 8)
        res = print_neatly([1, 1, 1], 10)
        assert res is not None
        self.assertEqual(res[0], 0)


if __name__ == "__main__":
    print(demo(), end="")
