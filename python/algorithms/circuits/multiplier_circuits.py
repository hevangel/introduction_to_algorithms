"""Array & Wallace-Tree Multipliers — CLRS 29.3 (1e) (Editions: 1).

Multiply n-bit numbers: array multiplier with ripple adders (Theta(n) depth) vs
carry-save Wallace tree (Theta(lg n) depth).
"""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
import sys
import unittest
from typing import Callable

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

    def half_adder(self, a: int, b: int) -> tuple[int, int]:
        return (self.xor_gate(a, b), self.and_gate(a, b))

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


def _build_ripple(c: _Circuit, a: list[int], b: list[int], cin: int) -> tuple[list[int], int]:
    carry = cin
    total: list[int] = []
    for ai, bi in zip(a, b):
        s, co = c.full_adder(ai, bi, carry)
        total.append(s)
        carry = co
    return total, carry


def _build_cla(c: _Circuit, a: list[int], b: list[int]) -> list[int]:
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


def array_multiplier(n: int) -> tuple[_Circuit, list[int]]:
    """Array multiplier: add the n shifted partial products one after another with ripple-carry adders.

    Returns (circuit, 2n product wires). Inputs: a (n bits), then b (n bits).
    """
    c = _Circuit()
    a, b = c.inputs(n), c.inputs(n)
    zero = c.constant(False)
    acc = [zero] * (2 * n)
    for i in range(n):
        row = [c.and_gate(a[j], b[i]) for j in range(n)]
        total, cout = _build_ripple(c, acc[i : i + n], row, zero)
        for k in range(n):
            acc[i + k] = total[k]
        if i + n < 2 * n:
            acc[i + n] = cout
    return c, acc


def wallace_multiplier(n: int) -> tuple[_Circuit, list[int]]:
    """Wallace-tree multiplier: column-wise 3:2/2:2 reduction + carry-lookahead adder. Depth Theta(lg n)."""
    c = _Circuit()
    a, b = c.inputs(n), c.inputs(n)
    zero = c.constant(False)
    cols: list[list[int]] = [[] for _ in range(2 * n + 1)]
    for i in range(n):
        for j in range(n):
            cols[i + j].append(c.and_gate(a[j], b[i]))
    max_h = max((len(col) for col in cols), default=0)
    targets = [2]
    while targets[-1] < max_h:
        targets.append(targets[-1] * 3 // 2)
    targets.pop()
    for d in reversed(targets):
        next_cols: list[list[int]] = [[] for _ in range(len(cols) + 1)]
        carries_in: list[int] = []
        for k in range(len(cols)):
            bits = list(cols[k])
            out: list[int] = []
            carry_out: list[int] = []
            h = len(bits) + len(carries_in)
            while h > d and len(bits) >= 2:
                if h - d >= 2 and len(bits) >= 3:
                    x, y, z = bits.pop(), bits.pop(), bits.pop()
                    s, co = c.full_adder(x, y, z)
                    out.append(s)
                    carry_out.append(co)
                    h -= 2
                else:
                    x, y = bits.pop(), bits.pop()
                    s, co = c.half_adder(x, y)
                    out.append(s)
                    carry_out.append(co)
                    h -= 1
            next_cols[k].extend(bits)
            next_cols[k].extend(out)
            next_cols[k].extend(carries_in)
            carries_in = carry_out
        next_cols[len(cols)].extend(carries_in)
        cols = next_cols[: len(cols)]

    def row(idx: int) -> list[int]:
        return [col[idx] if idx < len(col) else zero for col in cols[: 2 * n]]

    r0, r1 = row(0), row(1)
    total = _build_cla(c, r0, r1)[: 2 * n]
    return c, total


def multiply(build: Callable[[int], tuple[_Circuit, list[int]]], n: int, x: int, y: int) -> int:
    c, outs = build(n)
    inp = _to_bits(x, n) + _to_bits(y, n)
    v = c.eval(inp)
    return _from_bits([v[w] for w in outs])


def demo() -> str:
    lines: list[str] = []
    lines.append(
        f"13 x 11: array = {multiply(array_multiplier, 4, 13, 11)}, "
        f"Wallace = {multiply(wallace_multiplier, 4, 13, 11)}"
    )
    lines.append("  n | array size/depth | wallace size/depth")
    for n in [4, 8, 16, 32]:
        a, ao = array_multiplier(n)
        w, wo = wallace_multiplier(n)
        lines.append(
            f"{n:>3} | {a.size():>6} / {a.max_depth(ao):>4}    | {w.size():>6} / {w.max_depth(wo):>4}"
        )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_both_multiply_correctly(self) -> None:
        r = Rng(173)
        for n in range(1, 9):
            m = (1 << n) - 1
            for _ in range(30):
                x = r.next_u64() & m
                y = r.next_u64() & m
                self.assertEqual(multiply(array_multiplier, n, x, y), x * y, f"array n={n} {x}*{y}")
                self.assertEqual(multiply(wallace_multiplier, n, x, y), x * y, f"wallace n={n} {x}*{y}")
        self.assertEqual(multiply(wallace_multiplier, 8, 255, 255), 65025)

    def test_wallace_is_shallower_for_wide_operands(self) -> None:
        a, ao = array_multiplier(32)
        w, wo = wallace_multiplier(32)
        self.assertLess(w.max_depth(wo) * 2, a.max_depth(ao))


if __name__ == "__main__":
    print(demo(), end="")
