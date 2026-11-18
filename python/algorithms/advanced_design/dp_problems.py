"""More DP: Company Party & Viterbi — CLRS Problems 15-6, 15-7 (3e), 14-6, 14-7 (4e) (3e,4e).

Planning a company party (tree DP, maximum independent set) and Viterbi decoding of a speech-recognition graph.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def company_party(
    children: list[list[int]],
    rating: list[int],
    root: int,
) -> tuple[int, list[int]]:
    """Problem 15-6: maximum total rating of an independent set in the employee tree."""
    n = len(children)
    with_v = [0] * n
    without_v = [0] * n

    def go(v: int) -> None:
        with_v[v] = rating[v]
        without_v[v] = 0
        for c in children[v]:
            go(c)
            with_v[v] += without_v[c]
            without_v[v] += max(with_v[c], without_v[c])

    go(root)
    guests: list[int] = []
    stack: list[tuple[int, bool]] = [(root, True)]
    while stack:
        v, free = stack.pop()
        invite = free and with_v[v] >= without_v[v]
        if invite:
            guests.append(v)
        for c in children[v]:
            stack.append((c, not invite))
    guests.sort()
    return max(with_v[root], without_v[root]), guests


def viterbi(
    edges: list[tuple[int, int, str, float]],
    n: int,
    start: int,
    sigma: str,
) -> tuple[float, list[int]] | None:
    """Problem 15-7: most probable path from start whose edge labels spell sigma."""
    k = len(sigma)
    best = [[0.0] * n for _ in range(k + 1)]
    back = [[-1] * n for _ in range(k + 1)]
    best[0][start] = 1.0
    for i in range(1, k + 1):
        ch = sigma[i - 1]
        for u, v, c, p in edges:
            if c == ch and best[i - 1][u] * p > best[i][v]:
                best[i][v] = best[i - 1][u] * p
                back[i][v] = u
    end = max(range(n), key=lambda a: best[k][a])
    if best[k][end] == 0.0:
        return None
    path = [end]
    v = end
    for i in range(k, 0, -1):
        v = back[i][v]
        path.append(v)
    path.reverse()
    return best[k][end], path


def demo() -> str:
    lines: list[str] = []
    children = [[1, 2, 3], [4, 5], [], [6], [], [], []]
    rating = [5, 6, 2, 3, 4, 1, 9]
    best, guests = company_party(children, rating, 0)
    lines.append(f"company party: best total conviviality {best} inviting employees {guests}")
    edges = [
        (0, 1, "a", 0.5),
        (0, 2, "a", 0.3),
        (1, 3, "b", 0.9),
        (2, 3, "b", 1.0),
        (3, 0, "a", 0.6),
    ]
    lines.append(f"viterbi 'ab' from state 0: {viterbi(edges, 4, 0, 'ab')}")
    lines.append(f"viterbi 'abaab' from state 0: {viterbi(edges, 4, 0, 'abaab')}")
    lines.append(f"viterbi 'bb' from state 0: {viterbi(edges, 4, 0, 'bb')}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_party_matches_brute_force(self) -> None:
        r = Rng(111)
        for _ in range(50):
            n = r.range(1, 10)
            children: list[list[int]] = [[] for _ in range(n)]
            parent = [-1] * n
            for v in range(1, n):
                p = r.range(0, v - 1)
                children[p].append(v)
                parent[v] = p
            rating = r.vec(n, -3, 9)
            best, guests = company_party(children, rating, 0)
            brute = max(
                sum(rating[v] for v in range(n) if (m >> v) & 1)
                for m in range(1 << n)
                if all(not (((m >> v) & 1) and ((m >> parent[v]) & 1)) for v in range(1, n))
            )
            self.assertEqual(best, brute)
            self.assertEqual(sum(rating[v] for v in guests), best)
            guest_set = set(guests)
            self.assertTrue(all(v == 0 or parent[v] not in guest_set for v in guests))

    def test_viterbi_picks_most_probable_path(self) -> None:
        edges = [
            (0, 1, "a", 0.5),
            (0, 2, "a", 0.3),
            (1, 3, "b", 0.9),
            (2, 3, "b", 1.0),
            (3, 0, "a", 0.6),
        ]
        res = viterbi(edges, 4, 0, "ab")
        assert res is not None
        p, path = res
        self.assertLess(abs(p - 0.45), 1e-12)
        self.assertEqual(path, [0, 1, 3])
        self.assertIsNone(viterbi(edges, 4, 0, "bb"))
        res2 = viterbi(edges, 4, 0, "abaab")
        assert res2 is not None
        p2, path2 = res2
        self.assertEqual(len(path2), 6)
        self.assertGreater(p2, 0.0)


if __name__ == "__main__":
    print(demo(), end="")
