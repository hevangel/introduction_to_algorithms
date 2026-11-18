"""Open Addressing — CLRS 11.4 (1e–4e) (1,2,3,4).

Store all keys in the table itself; linear probing, quadratic probing and double hashing.
"""

from __future__ import annotations

from enum import Enum, auto
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng

_MASK64 = (1 << 64) - 1
_EMPTY = object()
_DELETED = object()


class Probe(Enum):
    LINEAR = auto()
    QUADRATIC = auto()
    DOUBLE = auto()


class OpenAddressTable:
    """Open-addressing table; m is a power of two so probe sequences visit every slot."""

    def __init__(self, m: int, probe: Probe) -> None:
        assert m > 0 and (m & (m - 1)) == 0
        self.slots: list[object] = [_EMPTY] * m
        self.probe = probe
        self.n = 0

    def _h1(self, k: int) -> int:
        return (((k * 0x9E37_79B9_7F4A_7C15) & _MASK64) >> 29) & (len(self.slots) - 1)

    def _h2(self, k: int) -> int:
        return ((((k * 0xC2B2_AE3D_27D4_EB4F) & _MASK64) >> 31) | 1) & (len(self.slots) - 1)

    def _at(self, k: int, i: int) -> int:
        m = len(self.slots)
        if self.probe == Probe.LINEAR:
            return (self._h1(k) + i) & (m - 1)
        if self.probe == Probe.QUADRATIC:
            return (self._h1(k) + (i + i * i) // 2) & (m - 1)
        return (self._h1(k) + i * self._h2(k)) & (m - 1)

    def __len__(self) -> int:
        return self.n

    def is_empty(self) -> bool:
        return self.n == 0

    def load_factor(self) -> float:
        return self.n / len(self.slots)

    def insert(self, k: int) -> int:
        if self.search(k) is not None:
            return 0
        for i in range(len(self.slots)):
            j = self._at(k, i)
            if self.slots[j] is _EMPTY or self.slots[j] is _DELETED:
                self.slots[j] = k
                self.n += 1
                return i + 1
        raise OverflowError("hash table overflow")

    def search(self, k: int) -> tuple[int, int] | None:
        for i in range(len(self.slots)):
            j = self._at(k, i)
            val = self.slots[j]
            if val is _EMPTY:
                return None
            if val is not _DELETED and val == k:
                return (j, i + 1)
        return None

    def delete(self, k: int) -> bool:
        found = self.search(k)
        if found is None:
            return False
        j, _ = found
        self.slots[j] = _DELETED
        self.n -= 1
        return True


def average_successful_probes(probe: Probe, m: int, alpha: float, seed: int) -> float:
    r = Rng(seed)
    t = OpenAddressTable(m, probe)
    keys = [r.next_u64() for _ in range(int(alpha * m))]
    for k in keys:
        t.insert(k)
    return sum(t.search(k)[1] for k in keys) / len(keys)  # type: ignore[index]


def demo() -> str:
    lines: list[str] = []
    lines.append("average probes for a successful search (m = 4096)")
    lines.append("  α   | linear | quadratic | double | uniform-hashing theory (1/α)ln(1/(1−α))")
    for alpha in [0.5, 0.75, 0.9]:
        th = (1.0 / alpha) * math.log(1.0 / (1.0 - alpha))
        lin = average_successful_probes(Probe.LINEAR, 4096, alpha, 1)
        quad = average_successful_probes(Probe.QUADRATIC, 4096, alpha, 1)
        dbl = average_successful_probes(Probe.DOUBLE, 4096, alpha, 1)
        lines.append(f" {alpha:.2f} | {lin:>6.2f} | {quad:>9.2f} | {dbl:>6.2f} | {th:.2f}")
    t = OpenAddressTable(8, Probe.LINEAR)
    for k in [10, 18, 26]:
        t.insert(k)
    lines.append(f"deleted flag keeps later keys reachable: delete(10)={t.delete(10)}, search(26)={t.search(26) is not None}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_all_strategies_behave_like_a_set(self) -> None:
        for probe in (Probe.LINEAR, Probe.QUADRATIC, Probe.DOUBLE):
            t = OpenAddressTable(256, probe)
            s: set[int] = set()
            r = Rng(57)
            for _ in range(4000):
                k = r.range(0, 400)
                op = r.range(0, 2)
                if op == 0:
                    if len(s) < 200:
                        t.insert(k)
                        s.add(k)
                elif op == 1:
                    got = t.delete(k)
                    want = k in s
                    s.discard(k)
                    self.assertEqual(got, want)
                else:
                    self.assertEqual(t.search(k) is not None, k in s)
            self.assertEqual(len(t), len(s))

    def test_table_can_be_filled_completely_and_then_overflows(self) -> None:
        for probe in (Probe.LINEAR, Probe.QUADRATIC, Probe.DOUBLE):
            t = OpenAddressTable(64, probe)
            for k in range(64):
                t.insert(k * 7919)
            with self.assertRaises(OverflowError):
                t.insert(999_983)
            for k in range(64):
                self.assertIsNotNone(t.search(k * 7919))

    def test_double_hashing_beats_linear_at_high_load(self) -> None:
        lin = average_successful_probes(Probe.LINEAR, 4096, 0.9, 3)
        dbl = average_successful_probes(Probe.DOUBLE, 4096, 0.9, 3)
        self.assertLess(dbl, lin)


if __name__ == "__main__":
    print(demo(), end="")
