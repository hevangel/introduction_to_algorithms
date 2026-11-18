"""Edit Distance — CLRS Problem 15-5 (2e–3e), 14-5 (4e) (2e,3e,4e).

Transform x into y with copy / replace / delete / insert / twiddle / kill at minimum cost.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass(frozen=True)
class Costs:
    copy: int
    replace: int
    delete: int
    insert: int
    twiddle: int
    kill: int

    @classmethod
    def levenshtein(cls) -> Costs:
        big = 1 << 58
        return cls(copy=0, replace=1, delete=1, insert=1, twiddle=big, kill=big)

    @classmethod
    def with_twiddle_and_kill(cls) -> Costs:
        return cls(copy=0, replace=2, delete=2, insert=2, twiddle=1, kill=3)


@dataclass(frozen=True)
class Copy:
    c: str


@dataclass(frozen=True)
class Replace:
    a: str
    b: str


@dataclass(frozen=True)
class Delete:
    a: str


@dataclass(frozen=True)
class Insert:
    b: str


@dataclass(frozen=True)
class Twiddle:
    a: str
    b: str


@dataclass(frozen=True)
class Kill:
    k: int


Op = Copy | Replace | Delete | Insert | Twiddle | Kill


def edit_distance(x: str, y: str, c: Costs) -> tuple[int, list[Op]]:
    """Minimum cost to transform x into y, together with an optimal operation sequence."""
    m, n = len(x), len(y)
    inf = 1 << 60
    dp = [[inf] * (n + 1) for _ in range(m + 1)]
    choice = [[0] * (n + 1) for _ in range(m + 1)]
    dp[0][0] = 0

    for i in range(m + 1):
        for j in range(n + 1):
            if i == 0 and j == 0:
                continue
            best = inf
            ch = 0
            if i > 0 and j > 0:
                if x[i - 1] == y[j - 1]:
                    v = dp[i - 1][j - 1] + c.copy
                    if v < best:
                        best, ch = v, 1
                v = dp[i - 1][j - 1] + c.replace
                if v < best:
                    best, ch = v, 2
            if i > 0:
                v = dp[i - 1][j] + c.delete
                if v < best:
                    best, ch = v, 3
            if j > 0:
                v = dp[i][j - 1] + c.insert
                if v < best:
                    best, ch = v, 4
            if i > 1 and j > 1 and x[i - 1] == y[j - 2] and x[i - 2] == y[j - 1]:
                v = dp[i - 2][j - 2] + c.twiddle
                if v < best:
                    best, ch = v, 5
            dp[i][j] = best
            choice[i][j] = ch

    best_total = dp[m][n]
    kill_at = m
    for i in range(m):
        if dp[i][n] + c.kill < best_total:
            best_total = dp[i][n] + c.kill
            kill_at = i

    ops: list[Op] = []
    if kill_at < m:
        ops.append(Kill(m - kill_at))
    i, j = kill_at, n
    while i > 0 or j > 0:
        code = choice[i][j]
        if code == 1:
            ops.append(Copy(x[i - 1]))
            i -= 1
            j -= 1
        elif code == 2:
            ops.append(Replace(x[i - 1], y[j - 1]))
            i -= 1
            j -= 1
        elif code == 3:
            ops.append(Delete(x[i - 1]))
            i -= 1
        elif code == 4:
            ops.append(Insert(y[j - 1]))
            j -= 1
        elif code == 5:
            ops.append(Twiddle(x[i - 2], x[i - 1]))
            i -= 2
            j -= 2
        else:
            raise RuntimeError("unreachable choice code")
    ops.reverse()
    return best_total, ops


def apply(x: str, ops: list[Op]) -> str:
    """Apply an operation sequence to x and return the resulting string."""
    i = 0
    out: list[str] = []
    for op in ops:
        if isinstance(op, Copy):
            assert x[i] == op.c
            out.append(op.c)
            i += 1
        elif isinstance(op, Replace):
            assert x[i] == op.a
            out.append(op.b)
            i += 1
        elif isinstance(op, Delete):
            assert x[i] == op.a
            i += 1
        elif isinstance(op, Insert):
            out.append(op.b)
        elif isinstance(op, Twiddle):
            assert (x[i], x[i + 1]) == (op.a, op.b)
            out.append(op.b)
            out.append(op.a)
            i += 2
        elif isinstance(op, Kill):
            i += op.k
    assert i == len(x)
    return "".join(out)


def demo() -> str:
    lines: list[str] = []
    d, ops = edit_distance("kitten", "sitting", Costs.levenshtein())
    lines.append(f"Levenshtein(kitten → sitting) = {d}")
    lines.append(f"  ops: {ops}")
    c = Costs.with_twiddle_and_kill()
    d, ops = edit_distance("algorithm", "altruistic", c)
    lines.append(f"problem 15-5 costs (algorithm → altruistic) = {d}")
    lines.append(f"  ops: {ops}")
    d, ops = edit_distance("recieve", "receive", c)
    lines.append(f"twiddle fixes a typo: recieve → receive costs {d}: {ops}")
    d, ops = edit_distance("abcdefgh", "abc", c)
    lines.append(f"kill the tail: abcdefgh → abc costs {d}: {ops}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def _lev(self, a: str, b: str) -> int:
        d = [[0] * (len(b) + 1) for _ in range(len(a) + 1)]
        for i in range(len(a) + 1):
            d[i][0] = i
        for j in range(len(b) + 1):
            d[0][j] = j
        for i in range(1, len(a) + 1):
            for j in range(1, len(b) + 1):
                d[i][j] = min(
                    d[i - 1][j] + 1,
                    d[i][j - 1] + 1,
                    d[i - 1][j - 1] + (1 if a[i - 1] != b[j - 1] else 0),
                )
        return d[len(a)][len(b)]

    def test_classic_values(self) -> None:
        self.assertEqual(edit_distance("kitten", "sitting", Costs.levenshtein())[0], 3)
        self.assertEqual(edit_distance("", "abc", Costs.levenshtein())[0], 3)
        self.assertEqual(edit_distance("same", "same", Costs.levenshtein())[0], 0)

    def test_matches_reference_levenshtein_and_scripts_replay(self) -> None:
        r = Rng(105)
        for _ in range(100):
            x = "".join(chr(ord("a") + r.range(0, 3)) for _ in range(r.range(0, 8)))
            y = "".join(chr(ord("a") + r.range(0, 3)) for _ in range(r.range(0, 8)))
            d, ops = edit_distance(x, y, Costs.levenshtein())
            self.assertEqual(d, self._lev(x, y))
            self.assertEqual(apply(x, ops), y)
            c = Costs.with_twiddle_and_kill()
            d2, ops2 = edit_distance(x, y, c)
            self.assertEqual(apply(x, ops2), y)
            self.assertLessEqual(d2, 2 * self._lev(x, y))

    def test_twiddle_and_kill_are_used(self) -> None:
        c = Costs.with_twiddle_and_kill()
        d, ops = edit_distance("ab", "ba", c)
        self.assertEqual(d, 1)
        self.assertEqual(ops, [Twiddle("a", "b")])
        _, ops2 = edit_distance("abcdefgh", "abc", c)
        self.assertEqual(ops2[-1], Kill(5))


if __name__ == "__main__":
    print(demo(), end="")
