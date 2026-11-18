"""Stacks on Secondary Storage — CLRS Problem 18-1 (1e-4e) (Editions: 1,2,3,4).

Keep one page in memory and a push/pop pattern at a page boundary thrashes (Θ(1) I/Os per operation); two pages give O(1/m) amortised I/Os.
"""

from __future__ import annotations

from collections import deque
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


class PagedStack:
    """A stack of `len` items stored in pages of `m` items on disk, with an LRU buffer of `b` pages in memory.

    After every operation the page holding the top element must be resident. Counts page reads and write-backs.
    """

    def __init__(self, m: int, b: int) -> None:
        self.m = m
        self.b = b
        self._len = 0
        # (page, dirty), front (index 0) = most recently used
        self.resident: deque[tuple[int, bool]] = deque()
        self.allocated = 0  # pages 0..allocated exist on disk
        self.reads = 0
        self.writes = 0

    def accesses(self) -> int:
        return self.reads + self.writes

    def __len__(self) -> int:
        return self._len

    def is_empty(self) -> bool:
        return self._len == 0

    def _touch(self, page: int, dirty: bool) -> None:
        for i, (p, _) in enumerate(self.resident):
            if p == page:
                _, d = self.resident[i]
                del self.resident[i]
                self.resident.appendleft((p, d or dirty))
                return
        if page >= self.allocated:
            self.allocated = page + 1  # a brand-new page needs no disk read
        else:
            self.reads += 1
        if len(self.resident) == self.b:
            _, d = self.resident.pop()
            if d:
                self.writes += 1
        self.resident.appendleft((page, dirty))

    def push(self) -> None:
        self._len += 1
        self._touch((self._len - 1) // self.m, True)

    def pop(self) -> None:
        assert self._len > 0
        self._touch((self._len - 1) // self.m, True)
        self._len -= 1
        if self._len > 0:
            self._touch((self._len - 1) // self.m, False)  # the new top must be in memory


def demo() -> str:
    lines: list[str] = []
    m = 64
    for b in (1, 2, 3):
        # fill exactly to a page boundary, then alternate push/pop 10000 times
        s = PagedStack(m, b)
        for _ in range(m):
            s.push()
        before = s.accesses()
        for _ in range(10_000):
            s.push()
            s.pop()
        lines.append(
            f"page size {m}, {b} page(s) in memory: {s.accesses() - before:>6} disk accesses for 20000 operations at a page boundary"
        )
    s = PagedStack(m, 2)
    for _ in range(100_000):
        s.push()
    lines.append(
        f"100000 pushes with 2 buffered pages: {s.accesses()} accesses (n/m = {100_000 // m})"
    )
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_one_buffered_page_thrashes_at_a_boundary(self) -> None:
        m = 16
        s = PagedStack(m, 1)
        for _ in range(m):
            s.push()
        base = s.accesses()
        for _ in range(1000):
            s.push()
            s.pop()
        self.assertGreaterEqual(s.accesses() - base, 2000, s.accesses() - base)

    def test_two_pages_give_constant_amortised_cost(self) -> None:
        m = 16
        s = PagedStack(m, 2)
        for _ in range(m):
            s.push()
        base = s.accesses()
        for _ in range(1000):
            s.push()
            s.pop()
        self.assertLessEqual(s.accesses() - base, 4, s.accesses() - base)
        # n pushes cost about n/m page writes; popping everything reads each page back once
        # and writes it again: ≈ 3n/m in total
        t = PagedStack(m, 2)
        for _ in range(1600):
            t.push()
        for _ in range(1600):
            t.pop()
        self.assertLessEqual(t.accesses(), 4 * 1600 // m + 8, t.accesses())


if __name__ == "__main__":
    print(demo(), end="")
