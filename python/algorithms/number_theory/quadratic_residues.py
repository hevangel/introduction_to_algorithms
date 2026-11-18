"""Quadratic Residues & Tonelli-Shanks — CLRS Problem 31-4 (2e–4e); Problem 33-4 (1e) (1, 2, 3, 4).

Test quadratic residuosity modulo odd prime p via Euler's criterion and extract modular square roots in O(lg² p) via Tonelli-Shanks.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))


def legendre_symbol(a: int, p: int) -> int:
    """Legendre symbol (a / p) for odd prime p via Euler's criterion."""
    assert p >= 3 and p % 2 == 1
    a %= p
    if a == 0:
        return 0
    e = pow(a, (p - 1) // 2, p)
    return 1 if e == 1 else -1


def mod_sqrt(a: int, p: int) -> int | None:
    """Tonelli-Shanks modular square root: r in [0, p) with r^2 ≡ a (mod p) for prime p."""
    assert p >= 2
    a %= p
    if a == 0:
        return 0
    if p == 2:
        return a
    if legendre_symbol(a, p) != 1:
        return None
    if p % 4 == 3:
        return pow(a, (p + 1) // 4, p)

    q = p - 1
    s = 0
    while q % 2 == 0:
        q //= 2
        s += 1
    z = 2
    while legendre_symbol(z, p) != -1:
        z += 1
    m = s
    c = pow(z, q, p)
    t = pow(a, q, p)
    r = pow(a, (q + 1) // 2, p)

    while t != 1:
        i = 1
        tt = (t * t) % p
        while tt != 1:
            tt = (tt * tt) % p
            i += 1
        b = pow(c, 1 << (m - i - 1), p)
        m = i
        c = (b * b) % p
        t = (t * c) % p
        r = (r * b) % p
    return r


def demo() -> str:
    lines: list[str] = []
    p = 41
    qrs = [a for a in range(1, p) if legendre_symbol(a, p) == 1]
    lines.append(f"quadratic residues mod {p} (count {len(qrs)} = (p-1)/2): {qrs}")
    for a in (2, 5, 10, 13):
        r = mod_sqrt(a, p)
        if r is not None:
            lines.append(f"  sqrt({a}) mod {p} = {r} (and {p - r}), check: {r}^2 mod {p} = {(r * r) % p}")
        else:
            lines.append(f"  sqrt({a}) mod {p} = None (non-residue)")
    return "\n".join(lines) + "\n"


def _primes_up_to(limit: int) -> list[int]:
    is_p = [True] * (limit + 1)
    is_p[0] = False
    is_p[1] = False
    for i in range(2, limit + 1):
        if is_p[i]:
            for m in range(i * i, limit + 1, i):
                is_p[m] = False
    return [p for p in range(2, limit + 1) if is_p[p]]


class Tests(unittest.TestCase):
    def test_legendre_and_tonelli_shanks_exact_for_primes_up_to_1000(self) -> None:
        for p in _primes_up_to(1000):
            if p == 2:
                self.assertEqual(mod_sqrt(1, 2), 1)
                continue
            if p <= 200:
                residues = [False] * p
                for x in range(1, p):
                    residues[(x * x) % p] = True
                for a in range(1, p):
                    sym = legendre_symbol(a, p)
                    self.assertEqual(sym == 1, residues[a])
                    r = mod_sqrt(a, p)
                    if r is not None:
                        self.assertEqual(sym, 1)
                        self.assertEqual((r * r) % p, a)
                        self.assertEqual(((p - r) * (p - r)) % p, a)
                    else:
                        self.assertEqual(sym, -1)
            else:
                for a in range(1, 40):
                    a_mod = a % p
                    if a_mod == 0:
                        continue
                    sym = legendre_symbol(a_mod, p)
                    r = mod_sqrt(a_mod, p)
                    if r is not None:
                        self.assertEqual(sym, 1)
                        self.assertEqual((r * r) % p, a_mod)
                    else:
                        self.assertEqual(sym, -1)


if __name__ == "__main__":
    print(demo(), end="")
