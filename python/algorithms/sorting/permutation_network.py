"""Transposition & Permutation Networks — CLRS Problems 28-1 & 28-3 (1e); Problems 27-1 & 27-3 (2e).

Odd-even transposition sorting in Θ(n) depth and recursive 2-switch Beneš permutation routing in O(lg n) depth and O(n lg n) switches.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def odd_even_transposition_sort(a: list[int]) -> int:
    """Odd-even transposition (brick) sort: alternates even-pair and odd-pair compare-exchange layers."""
    n = len(a)
    if n <= 1:
        return 0
    layers = 0
    for phase in range(n):
        swapped = False
        i = phase & 1
        while i + 1 < n:
            if a[i] > a[i + 1]:
                a[i], a[i + 1] = a[i + 1], a[i]
                swapped = True
            i += 2
        layers += 1
        if not swapped and phase > 0:
            break
    return layers


class BenesNetwork:
    """Recursive Beneš / Waksman permutation network on n = 2^k wires (`2*k - 1` stages of 2-switches)."""

    def __init__(
        self,
        n: int,
        input_cross: list[bool],
        output_cross: list[bool],
        sub: tuple[BenesNetwork, BenesNetwork] | None,
    ) -> None:
        self.n = n
        self.input_cross = input_cross
        self.output_cross = output_cross
        self.sub = sub

    @classmethod
    def from_permutation(cls, perm: list[int]) -> BenesNetwork:
        """Construct switch settings routing input wire `i` to output wire `perm[i]` for n = 2^k."""
        n = len(perm)
        assert n >= 2 and (n & (n - 1)) == 0, "n must be a power of 2 >= 2"
        if n == 2:
            assert (perm[0] == 0 and perm[1] == 1) or (perm[0] == 1 and perm[1] == 0)
            return cls(2, [perm[0] == 1], [], None)
        m = n // 2
        inv = [0] * n
        for i, d in enumerate(perm):
            inv[d] = i

        color: list[int | None] = [None] * n
        for start in range(n):
            if color[start] is not None:
                continue
            stack = [(start, 0)]
            while stack:
                u, c = stack.pop()
                prev = color[u]
                if prev is not None:
                    assert prev == c, "conflict in Beneš 2-coloring"
                    continue
                color[u] = c
                stack.append((u ^ 1, c ^ 1))
                out_mate = inv[perm[u] ^ 1]
                stack.append((out_mate, c ^ 1))

        input_cross = [False] * m
        upper_perm = [0] * m
        lower_perm = [0] * m
        output_cross = [False] * m
        for p in range(m):
            c0 = color[2 * p]
            if c0 == 0:
                u_wire, l_wire = 2 * p, 2 * p + 1
            else:
                input_cross[p] = True
                u_wire, l_wire = 2 * p + 1, 2 * p
            upper_perm[p] = perm[u_wire] // 2
            lower_perm[p] = perm[l_wire] // 2
            q = perm[u_wire] // 2
            output_cross[q] = (perm[u_wire] & 1) == 1

        upper = cls.from_permutation(upper_perm)
        lower = cls.from_permutation(lower_perm)
        return cls(n, input_cross, output_cross, (upper, lower))

    def stage_count(self) -> int:
        if self.sub is None:
            return 1
        return 2 + self.sub[0].stage_count()

    def route(self, inputs: list) -> list:
        assert len(inputs) == self.n
        if self.n == 2:
            return [inputs[1], inputs[0]] if self.input_cross[0] else list(inputs)
        m = self.n // 2
        upper_in = []
        lower_in = []
        for p in range(m):
            if self.input_cross[p]:
                upper_in.append(inputs[2 * p + 1])
                lower_in.append(inputs[2 * p])
            else:
                upper_in.append(inputs[2 * p])
                lower_in.append(inputs[2 * p + 1])
        assert self.sub is not None
        u_net, l_net = self.sub
        upper_out = u_net.route(upper_in)
        lower_out = l_net.route(lower_in)
        out = []
        for q in range(m):
            if self.output_cross[q]:
                out.append(lower_out[q])
                out.append(upper_out[q])
            else:
                out.append(upper_out[q])
                out.append(lower_out[q])
        return out


def demo() -> str:
    lines: list[str] = []
    a = [8, 3, 7, 1, 6, 2, 5, 4]
    layers = odd_even_transposition_sort(a)
    lines.append(f"odd-even transposition sort in {layers} layers -> {a}")
    perm = [5, 2, 7, 0, 4, 1, 6, 3]
    net = BenesNetwork.from_permutation(perm)
    routed = net.route(list(range(8)))
    expected = [0] * 8
    for i, dest in enumerate(perm):
        expected[dest] = i
    lines.append(f"Beneš n=8 stages = {net.stage_count()}, perm = {perm}")
    lines.append(f"routed wires = {routed} (exact = {routed == expected})")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_odd_even_transposition_sorts_all_inputs(self) -> None:
        r = Rng(2701)
        for n in range(30):
            a = r.vec(n, -100, 100)
            expected = sorted(a)
            layers = odd_even_transposition_sort(a)
            self.assertEqual(a, expected)
            self.assertLessEqual(layers, n)

    def test_benes_routes_arbitrary_power_of_two_permutations(self) -> None:
        r = Rng(2703)
        for k in range(1, 7):
            n = 1 << k
            for _ in range(40):
                perm = list(range(n))
                for i in range(n - 1, 0, -1):
                    j = r.range(0, i)
                    perm[i], perm[j] = perm[j], perm[i]
                net = BenesNetwork.from_permutation(perm)
                self.assertEqual(net.stage_count(), 2 * k - 1)
                out = net.route(list(range(n)))
                for i, dest in enumerate(perm):
                    self.assertEqual(out[dest], i, f"failed for n={n}, perm={perm}")


if __name__ == "__main__":
    print(demo(), end="")
