"""RSA Public-Key Cryptosystem — CLRS 31.7 (1e–4e) (1, 2, 3, 4).

Key generation from two primes, encryption/decryption by modular exponentiation, and signatures.
"""

from __future__ import annotations

from dataclasses import dataclass
import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _extended_euclid(a: int, b: int) -> tuple[int, int, int]:
    if b == 0:
        return (a, 1, 0)
    d, x, y = _extended_euclid(b, a % b)
    return (d, y, x - (a // b) * y)


def _mod_inverse(a: int, n: int) -> int | None:
    d, x, _ = _extended_euclid(a % n, n)
    return (x % n) if d == 1 else None


def _witness(a: int, n: int) -> bool:
    t = 0
    u = n - 1
    while u % 2 == 0:
        u //= 2
        t += 1
    x = pow(a, u, n)
    for _ in range(t):
        y = (x * x) % n
        if y == 1 and x != 1 and x != n - 1:
            return True
        x = y
    return x != 1


def _is_prime(n: int) -> bool:
    if n < 2:
        return False
    bases = (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37)
    for p in bases:
        if n % p == 0:
            return n == p
    return not any(_witness(a, n) for a in bases)


@dataclass(frozen=True, slots=True)
class PublicKey:
    n: int
    e: int


@dataclass(frozen=True, slots=True)
class SecretKey:
    n: int
    d: int


def random_prime(bits: int, rng: Rng) -> int:
    """Random prime with exactly `bits` bits (bits <= 31 so that n = p*q fits in u64)."""
    while True:
        c = (rng.next_u64() >> (64 - bits)) | (1 << (bits - 1)) | 1
        if _is_prime(c):
            return c


def keygen_with_e(p: int, q: int, e: int) -> tuple[PublicKey, SecretKey] | None:
    n = p * q
    phi = (p - 1) * (q - 1)
    if math.gcd(e, phi) != 1:
        return None
    d = _mod_inverse(e, phi)
    if d is None:
        return None
    return (PublicKey(n=n, e=e), SecretKey(n=n, d=d))


def keygen_from_primes(p: int, q: int) -> tuple[PublicKey, SecretKey] | None:
    phi = (p - 1) * (q - 1)
    e = next((cand for cand in (65537, 17, 5, 3) if cand < phi and math.gcd(cand, phi) == 1), None)
    if e is None:
        return None
    return keygen_with_e(p, q, e)


def keygen(bits: int, rng: Rng) -> tuple[PublicKey, SecretKey]:
    while True:
        p = random_prime(bits, rng)
        q = random_prime(bits, rng)
        if p != q:
            res = keygen_from_primes(p, q)
            if res is not None:
                return res


def encrypt(pk: PublicKey, m: int) -> int:
    return pow(m, pk.e, pk.n)


def decrypt(sk: SecretKey, c: int) -> int:
    return pow(c, sk.d, sk.n)


def sign(sk: SecretKey, m: int) -> int:
    return pow(m, sk.d, sk.n)


def verify(pk: PublicKey, m: int, sig: int) -> bool:
    return pow(sig, pk.e, pk.n) == (m % pk.n)


def demo() -> str:
    lines: list[str] = []
    pair = keygen_with_e(11, 29, 3)
    assert pair is not None
    pk, sk = pair
    lines.append(f"p=11, q=29: public key (n={pk.n}, e={pk.e}), secret key d={sk.d}")
    c = encrypt(pk, 100)
    lines.append(f"encrypt 100 → {c}; decrypt → {decrypt(sk, c)}")
    r = Rng(99)
    pk2, sk2 = keygen(30, r)
    m = 123_456_789_012 % pk2.n
    c2 = encrypt(pk2, m)
    lines.append(f"60-bit modulus n = {pk2.n}: message {m} → ciphertext {c2} → {decrypt(sk2, c2)}")
    sig = sign(sk2, m)
    lines.append(f"signature verifies: {verify(pk2, m, sig)}   tampered message verifies: {verify(pk2, m + 1, sig)}")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_book_numbers(self) -> None:
        pair = keygen_with_e(11, 29, 3)
        assert pair is not None
        pk, sk = pair
        self.assertEqual((pk.n, pk.e, sk.d), (319, 3, 187))
        self.assertIsNone(keygen_with_e(11, 29, 2))
        pair2 = keygen_from_primes(11, 29)
        assert pair2 is not None
        pk2, sk2 = pair2
        self.assertEqual((pk2.e, sk2.d), (17, 33))
        self.assertEqual(encrypt(pk, 100), 254)
        self.assertEqual(decrypt(sk, 254), 100)

    def test_roundtrip_and_signatures_with_random_keys(self) -> None:
        r = Rng(214)
        for bits in (8, 12, 20, 31):
            pk, sk = keygen(bits, r)
            for _ in range(30):
                m = r.next_u64() % pk.n
                self.assertEqual(decrypt(sk, encrypt(pk, m)), m)
                self.assertEqual(encrypt(pk, decrypt(sk, m)), m)
                s = sign(sk, m)
                self.assertTrue(verify(pk, m, s))
                if pk.n > 2:
                    self.assertFalse(verify(pk, (m + 1) % pk.n, s))

    def test_random_primes_are_prime_with_requested_size(self) -> None:
        r = Rng(215)
        for _ in range(20):
            p = random_prime(31, r)
            self.assertTrue(_is_prime(p))
            self.assertEqual(p.bit_length(), 31)


if __name__ == "__main__":
    print(demo(), end="")
