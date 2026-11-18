"""Huffman Codes — CLRS 16.3 (1e–3e) / 15.3 (4e) (1e,2e,3e,4e).

Optimal prefix-free codes by greedily merging the two least frequent symbols.
"""

from __future__ import annotations

from dataclasses import dataclass
import heapq
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


@dataclass
class _Node:
    freq: int
    sym: str | None
    left: int = -1
    right: int = -1


class HuffmanTree:
    """A Huffman tree stored in an arena; root is the index of the root node."""

    def __init__(self, nodes: list[_Node], root: int) -> None:
        self._nodes = nodes
        self._root = root

    def codes(self) -> dict[str, str]:
        """Codeword for each symbol ('0' = left, '1' = right). A single-symbol alphabet gets '0'."""
        m: dict[str, str] = {}
        st: list[tuple[int, str]] = [(self._root, "")]
        while st:
            v, code = st.pop()
            n = self._nodes[v]
            if n.sym is not None:
                m[n.sym] = code if code else "0"
            else:
                st.append((n.left, code + "0"))
                st.append((n.right, code + "1"))
        return dict(sorted(m.items()))

    def cost(self) -> int:
        """B(T) = sum f(c)*d_T(c): total encoded length in bits."""
        internal = sum(n.freq for n in self._nodes if n.sym is None)
        return self._nodes[self._root].freq if internal == 0 else internal

    def encode(self, text: str) -> str:
        c = self.codes()
        return "".join(c[ch] for ch in text)

    def decode(self, bits: str) -> str:
        out: list[str] = []
        v = self._root
        root_sym = self._nodes[self._root].sym
        for b in bits:
            if root_sym is None:
                v = self._nodes[v].left if b == "0" else self._nodes[v].right
            sym = self._nodes[v].sym
            if sym is not None:
                out.append(sym)
                v = self._root
            elif root_sym is not None:
                out.append(root_sym)
        return "".join(out)


def huffman(freqs: list[tuple[str, int]]) -> HuffmanTree:
    """HUFFMAN(C): repeatedly extract the two minimum-frequency nodes and merge them."""
    if not freqs:
        raise ValueError("freqs must not be empty")
    nodes = [_Node(freq=f, sym=c) for c, f in freqs]
    heap: list[tuple[int, int]] = [(nodes[i].freq, i) for i in range(len(nodes))]
    heapq.heapify(heap)
    while len(heap) > 1:
        f1, a = heapq.heappop(heap)
        f2, b = heapq.heappop(heap)
        idx = len(nodes)
        nodes.append(_Node(freq=f1 + f2, sym=None, left=a, right=b))
        heapq.heappush(heap, (f1 + f2, idx))
    _, root = heapq.heappop(heap)
    return HuffmanTree(nodes, root)


def demo() -> str:
    lines: list[str] = []
    f = [("a", 45), ("b", 13), ("c", 12), ("d", 16), ("e", 9), ("f", 5)]
    t = huffman(f)
    lines.append(f"frequencies (thousands): {f}")
    for c, code in t.codes().items():
        lines.append(f"  {c} → {code}")
    lines.append(f"Huffman cost = {t.cost()} bits per 100 symbols (fixed 3-bit code would use 300)")
    msg = "abacabadabacaba"
    t2 = huffman([("a", 8), ("b", 4), ("c", 2), ("d", 1)])
    enc = t2.encode(msg)
    lines.append(f"encode {msg!r} → {enc} ({len(enc)} bits vs {len(msg) * 2} fixed)")
    lines.append(f"decode → {t2.decode(enc)!r}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_example_cost(self) -> None:
        t = huffman([("a", 45), ("b", 13), ("c", 12), ("d", 16), ("e", 9), ("f", 5)])
        self.assertEqual(t.cost(), 224)
        codes = t.codes()
        self.assertEqual(len(codes["a"]), 1)
        self.assertEqual(len(codes["e"]), 4)
        self.assertEqual(len(codes["f"]), 4)

    def test_prefix_free_roundtrip_and_cost_formula(self) -> None:
        r = Rng(114)
        for _ in range(40):
            n = r.range(2, 12)
            freqs = [(chr(ord("a") + i), r.range(1, 100)) for i in range(n)]
            t = huffman(freqs)
            codes = t.codes()
            for a, ca in codes.items():
                for b, cb in codes.items():
                    if a != b:
                        self.assertFalse(cb.startswith(ca))
            direct = sum(f * len(codes[c]) for c, f in freqs)
            self.assertEqual(direct, t.cost())
            text = "".join(c * (f % 5 + 1) for c, f in freqs)
            self.assertEqual(t.decode(t.encode(text)), text)

    def test_optimality_on_two_symbol_and_single_symbol_alphabets(self) -> None:
        self.assertEqual(huffman([("x", 3), ("y", 7)]).cost(), 10)
        single = huffman([("z", 5)])
        self.assertEqual(single.codes()["z"], "0")
        self.assertEqual(single.decode(single.encode("zzz")), "zzz")


if __name__ == "__main__":
    print(demo(), end="")
