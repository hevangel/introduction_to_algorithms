"""Combinational Circuits — CLRS 29.1 (1e) (Editions: 1).

A gate-level circuit model with size and depth: the building block for adders and multipliers.
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


@dataclass(frozen=True)
class Gate:
    kind: str
    a: int = 0
    b: int = 0
    val: bool = False


class Circuit:
    """A combinational circuit as a DAG of gates; wire `i` is the output of gate `i`.

    Size = number of logic gates; depth of a wire = longest gate path from any input.
    """

    def __init__(self) -> None:
        self.gates: list[Gate] = []
        self._depth: list[int] = []
        self.n_inputs: int = 0

    def _push(self, g: Gate, d: int) -> int:
        self.gates.append(g)
        self._depth.append(d)
        return len(self.gates) - 1

    def input(self) -> int:
        self.n_inputs += 1
        return self._push(Gate("input"), 0)

    def inputs(self, n: int) -> list[int]:
        return [self.input() for _ in range(n)]

    def constant(self, b: bool) -> int:
        return self._push(Gate("const", val=b), 0)

    def not_gate(self, a: int) -> int:
        return self._push(Gate("not", a=a), self._depth[a] + 1)

    def and_gate(self, a: int, b: int) -> int:
        return self._push(Gate("and", a=a, b=b), max(self._depth[a], self._depth[b]) + 1)

    def or_gate(self, a: int, b: int) -> int:
        return self._push(Gate("or", a=a, b=b), max(self._depth[a], self._depth[b]) + 1)

    def xor_gate(self, a: int, b: int) -> int:
        return self._push(Gate("xor", a=a, b=b), max(self._depth[a], self._depth[b]) + 1)

    def depth_of(self, w: int) -> int:
        return self._depth[w]

    def max_depth(self, outs: list[int]) -> int:
        return max((self._depth[w] for w in outs), default=0)

    def size(self) -> int:
        """Number of logic gates (inputs and constants are free)."""
        return sum(1 for g in self.gates if g.kind not in ("input", "const"))

    def eval(self, inputs: list[bool]) -> list[bool]:
        """Evaluate every wire for the given input values (in creation order)."""
        assert len(inputs) == self.n_inputs
        v: list[bool] = []
        it = iter(inputs)
        for g in self.gates:
            if g.kind == "input":
                x = next(it)
            elif g.kind == "const":
                x = g.val
            elif g.kind == "not":
                x = not v[g.a]
            elif g.kind == "and":
                x = v[g.a] and v[g.b]
            elif g.kind == "or":
                x = v[g.a] or v[g.b]
            elif g.kind == "xor":
                x = v[g.a] ^ v[g.b]
            else:
                raise ValueError(f"unknown gate kind: {g.kind}")
            v.append(x)
        return v

    def half_adder(self, a: int, b: int) -> tuple[int, int]:
        """Half adder: (sum, carry)."""
        return (self.xor_gate(a, b), self.and_gate(a, b))

    def full_adder(self, a: int, b: int, cin: int) -> tuple[int, int]:
        """Full adder: (sum, carry-out) — 5 gates, depth 2 for sum and 3 for carry."""
        x = self.xor_gate(a, b)
        s = self.xor_gate(x, cin)
        c1 = self.and_gate(a, b)
        c2 = self.and_gate(x, cin)
        return (s, self.or_gate(c1, c2))


def to_bits(x: int, n: int) -> list[bool]:
    """Little-endian bits of `x` (n bits)."""
    return [((x >> i) & 1) == 1 for i in range(n)]


def from_bits(b: list[bool]) -> int:
    return sum((1 << i) for i, x in enumerate(b) if x)


def demo() -> str:
    lines: list[str] = []
    c = Circuit()
    a, b, cin = c.input(), c.input(), c.input()
    s, cout = c.full_adder(a, b, cin)
    lines.append(
        f"full adder: size {c.size()} gates, depth(sum) {c.depth_of(s)}, depth(carry) {c.depth_of(cout)}"
    )
    lines.append(" a b cin | sum cout")
    for x in range(8):
        v = c.eval([(x & 1) == 1, ((x >> 1) & 1) == 1, ((x >> 2) & 1) == 1])
        lines.append(
            f" {x & 1} {(x >> 1) & 1}  {(x >> 2) & 1}  |  {int(v[s])}   {int(v[cout])}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_full_adder_truth_table(self) -> None:
        c = Circuit()
        a, b, cin = c.input(), c.input(), c.input()
        s, co = c.full_adder(a, b, cin)
        self.assertEqual((c.size(), c.depth_of(s), c.depth_of(co)), (5, 2, 3))
        for x in range(8):
            bits = [(x & 1) == 1, ((x >> 1) & 1) == 1, ((x >> 2) & 1) == 1]
            v = c.eval(bits)
            total = sum(1 for bit in bits if bit)
            self.assertEqual(v[s], total % 2 == 1)
            self.assertEqual(v[co], total >= 2)

    def test_bit_helpers_and_constants(self) -> None:
        self.assertEqual(from_bits(to_bits(0b1011, 6)), 11)
        c = Circuit()
        t = c.constant(True)
        x = c.input()
        n = c.not_gate(x)
        o = c.and_gate(t, n)
        self.assertTrue(c.eval([False])[o])
        self.assertFalse(c.eval([True])[o])
        self.assertEqual(c.size(), 2)


if __name__ == "__main__":
    print(demo(), end="")
