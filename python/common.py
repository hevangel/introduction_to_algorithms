"""Tiny shared helpers: a deterministic RNG (stdlib only) and sorted check."""

from __future__ import annotations

_MASK64 = (1 << 64) - 1


class Rng:
    """xorshift64* pseudo-random generator — deterministic, matching src/common.rs."""

    def __init__(self, seed: int) -> None:
        self.state = (max(seed, 1) ^ 0x9E37_79B9_7F4A_7C15) & _MASK64

    def next_u64(self) -> int:
        x = self.state
        x ^= x >> 12
        x ^= (x << 25) & _MASK64
        x ^= x >> 27
        self.state = x
        return (x * 0x2545_F491_4F6C_DD1D) & _MASK64

    def range(self, lo: int, hi: int) -> int:
        """Uniform integer in lo..=hi (inclusive)."""
        return lo + (self.next_u64() % (hi - lo + 1))

    def f64(self) -> float:
        """Uniform float in [0, 1)."""
        return (self.next_u64() >> 11) / float(1 << 53)

    def shuffle(self, a: list) -> None:
        for i in range(len(a) - 1, 0, -1):
            j = self.range(0, i)
            a[i], a[j] = a[j], a[i]

    def vec(self, n: int, lo: int, hi: int) -> list[int]:
        return [lo + (self.next_u64() % (hi - lo + 1)) for _ in range(n)]


def is_sorted(a: list) -> bool:
    return all(a[i] <= a[i + 1] for i in range(len(a) - 1))
