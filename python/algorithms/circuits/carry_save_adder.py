"""Carry-Save Adder — CLRS 29.3 (1e) (Editions: 1).

Reduce three n-bit numbers to two in constant depth (a row of independent full adders).
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


def build_csa(
    c: _Circuit, x: list[int], y: list[int], z: list[int]
) -> tuple[list[int], list[int]]:
    """Append a carry-save adder to `c`: x + y + z = s + t.

    Returns (s: n bits, t: n+1 bits where t_0 = 0).
    """
    n = len(x)
    zero = c.constant(False)
    s: list[int] = []
    t: list[int] = [zero]
    for i in range(n):
        sm, carry = c.full_adder(x[i], y[i], z[i])
        s.append(sm)
        t.append(carry)
    return s, t


def carry_save_adder(n: int) -> tuple[_Circuit, list[int], list[int]]:
    """Standalone circuit; inputs are x, y, z (n bits each); outputs are s and t."""
    c = _Circuit()
    x, y, z = c.inputs(n), c.inputs(n), c.inputs(n)
    s, t = build_csa(c, x, y, z)
    return c, s, t


def add3(n: int, x: int, y: int, z: int) -> tuple[int, int]:
    """Evaluate: returns (s, t) as integers with s + t = x + y + z."""
    c, s, t = carry_save_adder(n)
    inp = _to_bits(x, n) + _to_bits(y, n) + _to_bits(z, n)
    v = c.eval(inp)
    return _from_bits([v[w] for w in s]), _from_bits([v[w] for w in t])


def demo() -> str:
    lines: list[str] = []
    s, t = add3(6, 37, 21, 50)
    lines.append(
        f"37 + 21 + 50 = {37 + 21 + 50} ; carry-save form: s = {s}, t = {t}, s + t = {s + t}"
    )
    for n in [4, 16, 64]:
        c, s_wires, t_wires = carry_save_adder(n)
        outs = s_wires + t_wires
        lines.append(
            f"n = {n:>2}: size {c.size():>3} gates, depth {c.max_depth(outs)} (independent of n!)"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sum_of_outputs_equals_sum_of_inputs(self) -> None:
        r = Rng(172)
        for n in range(1, 17):
            m = (1 << n) - 1
            for _ in range(30):
                x = r.next_u64() & m
                y = r.next_u64() & m
                z = r.next_u64() & m
                s, t = add3(n, x, y, z)
                self.assertEqual(s + t, x + y + z)

    def test_constant_depth(self) -> None:
        for n in [1, 8, 100]:
            c, s, t = carry_save_adder(n)
            outs = s + t
            self.assertEqual(c.max_depth(outs), 3)
            self.assertEqual(c.size(), 5 * n)


if __name__ == "__main__":
    print(demo(), end="")
