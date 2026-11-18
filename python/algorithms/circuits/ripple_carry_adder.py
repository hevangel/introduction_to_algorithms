"""Ripple-Carry Adder — CLRS 29.2 (1e) (Editions: 1).

n cascaded full adders: simple, Theta(n) size, but Theta(n) depth because the carry ripples.
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


class Circuit:
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

    def depth_of(self, w: int) -> int:
        return self._depth[w]

    def size(self) -> int:
        return sum(1 for g in self.gates if g.kind not in ("input", "const"))

    def full_adder(self, a: int, b: int, cin: int) -> tuple[int, int]:
        x = self.xor_gate(a, b)
        s = self.xor_gate(x, cin)
        c1 = self.and_gate(a, b)
        c2 = self.and_gate(x, cin)
        return (s, self.or_gate(c1, c2))

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


def build_ripple(c: Circuit, a: list[int], b: list[int], cin: int) -> tuple[list[int], int]:
    """Append an n-bit ripple-carry adder to `c`. Returns (sum wires (n bits), carry-out wire)."""
    carry = cin
    total: list[int] = []
    for i in range(len(a)):
        s, co = c.full_adder(a[i], b[i], carry)
        total.append(s)
        carry = co
    return total, carry


def ripple_carry_adder(n: int) -> tuple[Circuit, list[int]]:
    """Standalone n-bit adder circuit plus its output wires (inputs: a_0..a_{n-1}, b_0..b_{n-1})."""
    c = Circuit()
    a = c.inputs(n)
    b = c.inputs(n)
    zero = c.constant(False)
    total, cout = build_ripple(c, a, b, zero)
    total.append(cout)
    return c, total


def add(n: int, x: int, y: int) -> int:
    """Add two numbers with the circuit."""
    c, outs = ripple_carry_adder(n)
    inp = _to_bits(x, n) + _to_bits(y, n)
    v = c.eval(inp)
    return _from_bits([v[w] for w in outs])


def demo() -> str:
    lines: list[str] = []
    lines.append(f"12 + 7 with a 4-bit ripple-carry adder = {add(4, 12, 7)}")
    lines.append(f"15 + 15 = {add(4, 15, 15)} (carry-out becomes bit 4)")
    lines.append("  n | size | depth of carry-out")
    for n in [4, 8, 16, 32, 64]:
        c, outs = ripple_carry_adder(n)
        lines.append(f"{n:>3} | {c.size():>4} | {c.depth_of(outs[-1]):>3}   (= 2n + 1)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_adds_correctly(self) -> None:
        r = Rng(170)
        for n in range(1, 13):
            mask = (1 << n) - 1
            for _ in range(40):
                x = r.next_u64() & mask
                y = r.next_u64() & mask
                self.assertEqual(add(n, x, y), x + y, f"n={n}")

    def test_size_and_depth_are_linear(self) -> None:
        for n in [1, 2, 8, 20]:
            c, outs = ripple_carry_adder(n)
            self.assertEqual(c.size(), 5 * n)
            self.assertEqual(c.depth_of(outs[-1]), 2 * n + 1)


if __name__ == "__main__":
    print(demo(), end="")
