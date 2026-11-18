"""NP Verification Algorithms — CLRS 36.2 (1e) / 34.2 (2e–4e) (1,2,3,4).

Polynomial-time verifiers for Hamiltonian cycle, clique, vertex cover, CNF-SAT and subset sum — the definition of NP.
"""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

Cnf = list[list[int]]


def verify_hamiltonian_cycle(adj: list[list[bool]], cert: list[int]) -> bool:
    """Verify a Hamiltonian-cycle certificate in O(n)."""
    n = len(adj)
    if len(cert) != n:
        return False
    seen = [False] * n
    for v in cert:
        if not (0 <= v < n) or seen[v]:
            return False
        seen[v] = True
    return all(adj[cert[i]][cert[(i + 1) % n]] for i in range(n))


def verify_clique(adj: list[list[bool]], vertices: list[int], k: int) -> bool:
    """Verify a k-clique certificate."""
    return len(vertices) == k and all(
        vertices[i] != vertices[j] and adj[vertices[i]][vertices[j]]
        for i in range(k)
        for j in range(i + 1, k)
    )


def verify_vertex_cover(edges: list[tuple[int, int]], vertices: list[int], k: int) -> bool:
    """Verify a vertex cover of size <= k."""
    s = set(vertices)
    return len(vertices) <= k and all(u in s or v in s for u, v in edges)


def verify_cnf(f: Cnf, a: list[bool]) -> bool:
    """Verify a SAT truth assignment."""
    return all(any(a[abs(l) - 1] == (l > 0) for l in c) for c in f)


def verify_subset_sum(nums: list[int], target: int, idx: list[int]) -> bool:
    """Verify a SUBSET-SUM certificate."""
    seen: set[int] = set()
    for i in idx:
        if not (0 <= i < len(nums)) or i in seen:
            return False
        seen.add(i)
    return sum(nums[i] for i in idx) == target


def decide_by_exhaustive_certificates(bits: int, verifier: Callable[[list[bool]], bool]) -> list[bool] | None:
    """NP ⊆ EXP: decide by trying every certificate of `bits` bits with the verifier."""
    for m in range(1 << bits):
        c = [bool((m >> i) & 1) for i in range(bits)]
        if verifier(c):
            return c
    return None


def demo() -> str:
    lines: list[str] = []
    adj = [[False] * 5 for _ in range(5)]
    for a, b in [(0, 1), (1, 2), (2, 3), (3, 4), (4, 0), (0, 2)]:
        adj[a][b] = adj[b][a] = True
    lines.append(f"Hamiltonian-cycle certificate [0,1,2,3,4]: {verify_hamiltonian_cycle(adj, [0, 1, 2, 3, 4])}")
    lines.append(f"bad certificate [0,2,1,3,4]: {verify_hamiltonian_cycle(adj, [0, 2, 1, 3, 4])}")
    lines.append(f"3-clique {{0,1,2}}: {verify_clique(adj, [0, 1, 2], 3)}")
    edges = [(0, 1), (1, 2), (2, 3)]
    lines.append(f"vertex cover {{1,2}} of the path 0-1-2-3: {verify_vertex_cover(edges, [1, 2], 2)}")
    f: Cnf = [[1, 2], [-1, 3], [-2, -3]]
    lines.append(f"(x1∨x2)∧(¬x1∨x3)∧(¬x2∨¬x3) with x=(T,F,T): {verify_cnf(f, [True, False, True])}")
    lines.append(f"found by exhaustive certificate search: {decide_by_exhaustive_certificates(3, lambda a: verify_cnf(f, a))}")
    lines.append(f"subset sum {{3,34,4,12,5,2}} target 9 with items [2,4]→4+5: {verify_subset_sum([3, 34, 4, 12, 5, 2], 9, [2, 4])}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_hamiltonian_certificates(self) -> None:
        adj = [[False] * 4 for _ in range(4)]
        for a, b in [(0, 1), (1, 2), (2, 3), (3, 0)]:
            adj[a][b] = adj[b][a] = True
        self.assertTrue(verify_hamiltonian_cycle(adj, [0, 1, 2, 3]))
        self.assertTrue(verify_hamiltonian_cycle(adj, [2, 1, 0, 3]))
        self.assertFalse(verify_hamiltonian_cycle(adj, [0, 2, 1, 3]))
        self.assertFalse(verify_hamiltonian_cycle(adj, [0, 1, 2]))
        self.assertFalse(verify_hamiltonian_cycle(adj, [0, 1, 1, 3]))

    def test_clique_cover_sat_and_subset_sum(self) -> None:
        adj = [[False] * 4 for _ in range(4)]
        for a, b in [(0, 1), (1, 2), (0, 2), (2, 3)]:
            adj[a][b] = adj[b][a] = True
        self.assertTrue(verify_clique(adj, [0, 1, 2], 3))
        self.assertFalse(verify_clique(adj, [0, 1, 3], 3))
        self.assertFalse(verify_clique(adj, [0, 0, 1], 3))
        self.assertTrue(verify_vertex_cover([(0, 1), (1, 2), (2, 3)], [1, 2], 2))
        self.assertFalse(verify_vertex_cover([(0, 1), (1, 2), (2, 3)], [1], 2))
        f: Cnf = [[1, -2], [2, 3], [-1, -3]]
        self.assertTrue(verify_cnf(f, [True, True, False]))
        self.assertFalse(verify_cnf(f, [True, False, True]))
        self.assertTrue(verify_subset_sum([3, 34, 4, 12, 5, 2], 9, [2, 4]))
        self.assertFalse(verify_subset_sum([3, 34, 4, 12, 5, 2], 9, [2, 2, 3]))
        unsat: Cnf = [[1], [-1]]
        self.assertIsNone(decide_by_exhaustive_certificates(1, lambda a: verify_cnf(unsat, a)))


if __name__ == "__main__":
    print(demo(), end="")
