"""Carry-Lookahead Adder — CLRS 29.2 (1e) (Editions: 1).

Compute all carries with a parallel-prefix network over (generate, propagate) pairs: Theta(lg n) depth.
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
class _Gate:
    kind: str
    a: int = 0
    b: int = 0
    val: bool = False


class _Circuit:
    def __init__(self) -> None:
        self.gates: list[_Gate] = []
        self._depth: list[int] = []
        self.n_inputs: int = 0

    def _push(self, g: _Gate, d: int) -> int:
        self.gates.append(g)
        self._depth.append(d)
        return len(self.gates) - 1

    def input(self) -> int:
        self.n_inputs += 1
        return self._push(_Gate("input"), 0)

    def inputs(self, n: int) -> list[int]:
        return [self.input() for _ in range(n)]

    def constant(self, b: bool) -> int:
        return self._push(_Gate("const", val=b), 0)

    def and_gate(self, a: int, b: int) -> int:
        return self._push(_Gate("and", a=a, b=b), max(self._depth[a], self._depth[b]) + 1)

    def or_gate(self, a: int, b: int) -> int:
        return self._push(_Gate("or", a=a, b=b), max(self._depth[a], self._depth[b]) + 1)

    def xor_gate(self, a: int, b: int) -> int:
        return self._push(_Gate("xor", a=a, b=b), max(self._depth[a], self._depth[b]) + 1)

    def full_adder(self, a: int, b: int, cin: int) -> tuple[int, int]:
        x = self.xor_gate(a, b)
        s = self.xor_gate(x, cin)
        c1 = self.and_gate(a, b)
        c2 = self.and_gate(x, cin)
        return (s, self.or_gate(c1, c2))

    def max_depth(self, outs: list[int]) -> int:
        return max((self._depth[w] for w in outs), default=0)

    def size(self) -> int:
        return sum(1 for g in self.gates if g.kind not in ("input", "const"))

    def eval(self, inputs: list[bool]) -> list[bool]:
        assert len(inputs) == self.n_inputs
        v: list[bool] = []
        it = iter(inputs)
        for g in self.gates:
            if g.kind == "input":
                x = next(it)
            elif g.kind == "const":
                x = g.val
            elif g.kind == "and":
                x = v[g.a] and v[g.b]
            elif g.kind == "or":
                x = v[g.a] or v[g.b]
            elif g.kind == "xor":
                x = v[g.a] ^ v[g.b]
            else:
                raise ValueError(f"unknown gate: {g.kind}")
            v.append(x)
        return v


def _to_bits(x: int, n: int) -> list[bool]:
    return [((x >> i) & 1) == 1 for i in range(n)]


def _from_bits(b: list[bool]) -> int:
    return sum((1 << i) for i, x in enumerate(b) if x)


def _ripple_adder(n: int) -> tuple[_Circuit, list[int]]:
    c = _Circuit()
    a, b = c.inputs(n), c.inputs(n)
    carry = c.constant(False)
    outs: list[int] = []
    for i in range(n):
        s, carry = c.full_adder(a[i], b[i], carry)
        outs.append(s)
    outs.append(carry)
    return c, outs


def build_cla(c: _Circuit, a: list[int], b: list[int]) -> list[int]:
    """Append an n-bit carry-lookahead adder (carry-in = 0) to `c`.

    Returns n+1 sum wires (the last is carry out), using a Kogge-Stone prefix network.
    """
    n = len(a)
    p0 = [c.xor_gate(a[i], b[i]) for i in range(n)]
    g = [c.and_gate(a[i], b[i]) for i in range(n)]
    p = list(p0)
    step = 1
    while step < n:
        old_g, old_p = list(g), list(p)
        for i in range(step, n):
            t = c.and_gate(old_p[i], old_g[i - step])
            g[i] = c.or_gate(old_g[i], t)
            p[i] = c.and_gate(old_p[i], old_p[i - step])
        step *= 2
    total = [p0[0]]
    for i in range(1, n):
        total.append(c.xor_gate(p0[i], g[i - 1]))
    total.append(g[n - 1])
    return total


def carry_lookahead_adder(n: int) -> tuple[_Circuit, list[int]]:
    c = _Circuit()
    a = c.inputs(n)
    b = c.inputs(n)
    total = build_cla(c, a, b)
    return c, total


def add(n: int, x: int, y: int) -> int:
    c, outs = carry_lookahead_adder(n)
    inp = _to_bits(x, n) + _to_bits(y, n)
    v = c.eval(inp)
    return _from_bits([v[w] for w in outs])


def demo() -> str:
    lines: list[str] = []
    lines.append(f"100 + 155 with an 8-bit carry-lookahead adder = {add(8, 100, 155)}")
    lines.append("  n | ripple depth | lookahead depth | lookahead size")
    for n in [4, 8, 16, 32, 64, 128]:
        r, ro = _ripple_adder(n)
        c, co = carry_lookahead_adder(n)
        lines.append(
            f"{n:>3} | {r.max_depth(ro):>12} | {c.max_depth(co):>15} | {c.size():>8}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_adds_correctly(self) -> None:
        r = Rng(171)
        for n in range(1, 17):
            mask = (1 << n) - 1
            for _ in range(40):
                x = r.next_u64() & mask
                y = r.next_u64() & mask
                self.assertEqual(add(n, x, y), x + y, f"n={n}")
            self.assertEqual(add(n, (1 << n) - 1, (1 << n) - 1), 2 * ((1 << n) - 1))

    def test_depth_is_logarithmic(self) -> None:
        for k in range(2, 8):
            n = 1 << k
            c, outs = carry_lookahead_adder(n)
            self.assertLessEqual(c.max_depth(outs), 2 * k + 3, f"n={n} depth={c.max_depth(outs)}")
            r, ro = _ripple_adder(n)
            if n >= 16:
                self.assertLess(c.max_depth(outs), r.max_depth(ro) // 2)


if __name__ == "__main__":
    print(demo(), end="")
