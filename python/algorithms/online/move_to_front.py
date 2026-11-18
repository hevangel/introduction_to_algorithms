"""Self-Organizing Lists: Move-to-Front — CLRS 27.2 (4e); Problem 17-5 (2e–3e) (2e,3e,4e).

Access cost = position; moving each accessed item to the front is 4-competitive against any strategy.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


class MtfList:
    """A list of distinct keys; accessing the key at position i (1-based) costs i."""

    def __init__(self, n: int) -> None:
        self._items: list[int] = list(range(n))

    def order(self) -> list[int]:
        return list(self._items)

    def access(self, key: int) -> int:
        """Access key: pay its (1-based) position, then move it to the front."""
        pos = self._items.index(key)
        k = self._items.pop(pos)
        self._items.insert(0, k)
        return pos + 1


def mtf_cost(n: int, requests: list[int]) -> int:
    """Total access cost of move-to-front on a request sequence."""
    lst = MtfList(n)
    return sum(lst.access(r) for r in requests)


def static_cost(n: int, requests: list[int], order: list[int]) -> int:
    """Cost of a list that never changes."""
    _ = n
    pos = {key: idx + 1 for idx, key in enumerate(order)}
    return sum(pos[r] for r in requests)


def best_static_order(n: int, requests: list[int]) -> tuple[list[int], int]:
    """The best fixed order (sorted by decreasing access frequency) and its cost."""
    freq = [0] * n
    for r in requests:
        freq[r] += 1
    order = sorted(range(n), key=lambda k: freq[k], reverse=True)
    return order, static_cost(n, requests, order)


def demo() -> str:
    lines: list[str] = []
    r = Rng(6)
    n = 30
    weights = [1.0 / (i + 1.0) for i in range(n)]
    total = sum(weights)

    def stream() -> int:
        t = r.f64() * total
        for i, w in enumerate(weights):
            if t < w:
                return i
            t -= w
        return n - 1

    reqs = [stream() for _ in range(5000)]
    shuffled = list(range(n))
    r.shuffle(shuffled)
    best, best_cost = best_static_order(n, reqs)
    lines.append(f"5000 Zipf requests over {n} keys")
    lines.append(f"move-to-front total cost        = {mtf_cost(n, reqs)}")
    lines.append(f"fixed order, initial (0..n)     = {static_cost(n, reqs, list(range(n)))}")
    lines.append(f"fixed random order              = {static_cost(n, reqs, shuffled)}")
    lines.append(f"best fixed order (offline)      = {best_cost} (order starts {best[:5]})")
    lines.append("theorem: MTF ≤ 4 × cost of ANY algorithm (additive n² for the starting arrangement)")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_access_moves_to_front_and_costs_position(self) -> None:
        lst = MtfList(5)
        self.assertEqual(lst.access(3), 4)
        self.assertEqual(lst.order(), [3, 0, 1, 2, 4])
        self.assertEqual(lst.access(3), 1)
        self.assertEqual(lst.access(4), 5)
        self.assertEqual(lst.order(), [4, 3, 0, 1, 2])

    def test_four_competitive_against_the_best_static_order(self) -> None:
        r = Rng(270)
        for _ in range(100):
            n = r.range(2, 15)
            length = r.range(1, 400)
            reqs = [int((r.f64() ** 2) * n) % n for _ in range(length)]
            _, opt_static = best_static_order(n, reqs)
            self.assertLessEqual(mtf_cost(n, reqs), 4 * (opt_static + n * n), f"n={n} len={length}")

    def test_beats_a_bad_fixed_order_on_skewed_input(self) -> None:
        n = 20
        reqs = [19 if i % 5 == 0 else 18 for i in range(2000)]
        self.assertLess(mtf_cost(n, reqs) * 4, static_cost(n, reqs, list(range(n))))

    def test_adversarial_sequence_makes_mtf_cost_n_per_request(self) -> None:
        n = 8
        lst = MtfList(n)
        total = 0
        for _ in range(50):
            last = lst.order()[-1]
            total += lst.access(last)
        self.assertEqual(total, 50 * n)


if __name__ == "__main__":
    print(demo(), end="")
