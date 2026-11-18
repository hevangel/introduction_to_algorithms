"""Radix Trees — CLRS Problem 12-2 (1e-4e) (Editions: 1,2,3,4).

A binary trie over bit strings: insert, search and sort a set of binary strings in Θ(total length).
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class _Node:
    __slots__ = ("child", "terminal")

    def __init__(self) -> None:
        self.child: list[int | None] = [None, None]
        self.terminal = False


class RadixTree:
    """Radix tree (binary trie) storing strings of '0'/'1' characters."""

    def __init__(self) -> None:
        self.nodes: list[_Node] = [_Node()]
        self.len = 0

    def insert(self, s: str) -> bool:
        """Insert in O(|s|). Returns False if already present."""
        x = 0
        for c in s:
            b = ord(c) - ord("0")
            n = self.nodes[x].child[b]
            if n is None:
                self.nodes.append(_Node())
                n = len(self.nodes) - 1
                self.nodes[x].child[b] = n
            x = n
        fresh = not self.nodes[x].terminal
        self.nodes[x].terminal = True
        if fresh:
            self.len += 1
        return fresh

    def contains(self, s: str) -> bool:
        """Search in O(|s|)."""
        x = 0
        for c in s:
            n = self.nodes[x].child[ord(c) - ord("0")]
            if n is None:
                return False
            x = n
        return self.nodes[x].terminal

    def sorted(self) -> list[str]:
        """All stored strings in lexicographic order: a preorder walk (a string comes before its
        extensions, the 0-branch before the 1-branch)."""
        out: list[str] = []
        stack: list[tuple[int, str]] = [(0, "")]
        while stack:
            x, p = stack.pop()
            if self.nodes[x].terminal:
                out.append(p)
            n1 = self.nodes[x].child[1]
            if n1 is not None:
                stack.append((n1, p + "1"))
            n0 = self.nodes[x].child[0]
            if n0 is not None:
                stack.append((n0, p + "0"))
        return out

    def node_count(self) -> int:
        return len(self.nodes)


def radix_sort_strings(strings: list[str]) -> list[str]:
    """Problem 12-2: sort distinct binary strings in Θ(Σ|s|) time by inserting them into a radix
    tree and walking it in preorder."""
    t = RadixTree()
    for s in strings:
        t.insert(s)
    return t.sorted()


def _py_list(xs: list[str]) -> str:
    return "[" + ", ".join(f'"{x}"' for x in xs) + "]"


def demo() -> str:
    lines: list[str] = []
    words = ["1011", "10", "011", "100", "0", "1011", "101", "1"]
    t = RadixTree()
    for w in words:
        t.insert(w)
    lines.append(f"inserted {_py_list(words)}")
    lines.append(f"{t.len} distinct strings in {t.node_count()} trie nodes")
    lines.append(f"sorted: {_py_list(t.sorted())}")
    lines.append(
        f"contains '100'? {str(t.contains('100')).lower()}   contains '1010'? {str(t.contains('1010')).lower()}"
        f"   contains '01'? {str(t.contains('01')).lower()}"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_sorts_like_the_standard_library_and_membership(self) -> None:
        r = Rng(320)
        for _ in range(100):
            count = r.range(0, 40)
            strs: list[str] = []
            for _ in range(count):
                length = r.range(0, 8)
                strs.append("".join("1" if r.range(0, 1) == 1 else "0" for _ in range(length)))
            want = sorted(set(strs))
            self.assertEqual(radix_sort_strings(strs), want)
            t = RadixTree()
            for s in strs:
                t.insert(s)
            self.assertTrue(all(t.contains(s) for s in strs))
            self.assertEqual(t.len, len(want))
            self.assertFalse(t.contains("1111111111"))

    def test_prefixes_are_not_members_unless_inserted(self) -> None:
        t = RadixTree()
        self.assertTrue(t.insert("1010"))
        self.assertFalse(t.insert("1010"))
        self.assertTrue(not t.contains("101") and not t.contains("10101"))
        self.assertTrue(t.insert("") and t.contains(""))
        self.assertEqual(t.sorted(), ["", "1010"])


if __name__ == "__main__":
    print(demo(), end="")
