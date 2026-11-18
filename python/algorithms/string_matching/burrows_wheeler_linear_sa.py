"""Linear-Time Suffix Array & Burrows-Wheeler Transform — CLRS Problems 32-2 & 32-3 (4e) (4).

O(n) skew (DC3) or SA-IS suffix array construction plus invertible Burrows-Wheeler Transform and O(n) LF-mapping reconstruction.
"""

from __future__ import annotations

from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def _radix_pass(indices: list[int], keys: list[int], offset: int, alphabet_max: int) -> list[int]:
    counts = [0] * (alphabet_max + 2)
    for idx in indices:
        counts[keys[idx + offset] + 1] += 1
    for i in range(1, len(counts)):
        counts[i] += counts[i - 1]
    out = [0] * len(indices)
    for idx in indices:
        key = keys[idx + offset]
        out[counts[key]] = idx
        counts[key] += 1
    return out


def _dc3_rec(s: list[int], n: int, k: int) -> list[int]:
    if n == 0:
        return []
    if n == 1:
        return [0]
    n0 = (n + 2) // 3
    n1 = (n + 1) // 3
    n2 = n // 3
    n02 = n0 + n2

    s12 = [i for i in range(n + n0 - n1) if i % 3 != 0]

    s12 = _radix_pass(s12, s, 2, k)
    s12 = _radix_pass(s12, s, 1, k)
    sa12_raw = _radix_pass(s12, s, 0, k)

    name = 0
    prev_triple: tuple[int, int, int] | None = None
    s12_rec = [0] * (n02 + 3)
    for idx in sa12_raw:
        cur = (s[idx], s[idx + 1], s[idx + 2])
        if cur != prev_triple:
            name += 1
            prev_triple = cur
        if idx % 3 == 1:
            s12_rec[idx // 3] = name
        else:
            s12_rec[idx // 3 + n0] = name

    if name < n02:
        rec_sa = _dc3_rec(s12_rec, n02, name)
        sa12 = [3 * r + 1 if r < n0 else 3 * (r - n0) + 2 for r in rec_sa]
    else:
        sa12 = sa12_raw

    rank = [0] * (n + 3)
    for r, idx in enumerate(sa12):
        rank[idx] = r + 1

    s0_pre = [idx - 1 for idx in sa12 if idx % 3 == 1]
    sa0 = _radix_pass(s0_pre, s, 0, k)

    sa: list[int] = []
    t = 0
    p = n0 - n1
    while t < n0 and p < n02:
        i = sa12[p]
        j = sa0[t]
        if i % 3 == 1:
            i_smaller = (s[i], rank[i + 1]) < (s[j], rank[j + 1])
        else:
            i_smaller = (s[i], s[i + 1], rank[i + 2]) < (s[j], s[j + 1], rank[j + 2])
        if i_smaller:
            sa.append(i)
            p += 1
        else:
            sa.append(j)
            t += 1
    while p < n02:
        sa.append(sa12[p])
        p += 1
    while t < n0:
        sa.append(sa0[t])
        t += 1
    return sa


def dc3_suffix_array(text: bytes) -> list[int]:
    """Problem 32-2: Linear-time O(n) suffix array construction using Karkkainen-Sanders Skew (DC3)."""
    n = len(text)
    if n <= 1:
        return list(range(n))
    s = [0] * (n + 3)
    for i, b in enumerate(text):
        s[i] = b + 1
    return _dc3_rec(s, n, 256)


def bwt_encode(text: bytes) -> tuple[bytes, int]:
    """Problem 32-3: Burrows-Wheeler Transform in O(n) time via DC3 suffix array."""
    n = len(text)
    if n == 0:
        return (b"", 0)
    m = n + 1
    s = [0] * (m + 3)
    for i, b in enumerate(text):
        s[i] = b + 2
    s[n] = 1
    sa = _dc3_rec(s, m, 257)

    bwt = bytearray()
    sentinel_row = 0
    for row, pos in enumerate(sa):
        if pos == 0:
            sentinel_row = row
        else:
            bwt.append(text[pos - 1])
    return (bytes(bwt), sentinel_row)


def bwt_decode(bwt: bytes, sentinel_row: int) -> bytes:
    """Problem 32-3 inverse: O(n) Burrows-Wheeler inversion via C-table and LF-mapping."""
    n = len(bwt)
    if n == 0:
        return b""
    counts = [0] * 256
    ranks = [0] * n
    for i, b in enumerate(bwt):
        ranks[i] = counts[b]
        counts[b] += 1
    c_start = [0] * 256
    acc = 1
    for b in range(256):
        c_start[b] = acc
        acc += counts[b]

    out = bytearray()
    cur = 0
    for _ in range(n):
        if cur == sentinel_row:
            break
        adj = cur - 1 if cur > sentinel_row else cur
        ch = bwt[adj]
        out.append(ch)
        cur = c_start[ch] + ranks[adj]
    out.reverse()
    return bytes(out)


def demo() -> str:
    lines: list[str] = []
    text = b"mississippi"
    sa = dc3_suffix_array(text)
    lines.append(f"DC3 suffix array of {text.decode()!r}: {sa}")
    bwt, srow = bwt_encode(text)
    back = bwt_decode(bwt, srow)
    lines.append(f"BWT(mississippi) = {bwt.decode()!r} (sentinel row {srow}), decoded = {back.decode()!r}")
    return "\n".join(lines) + "\n"


def _suffix_array_prefix_doubling(s: bytes) -> list[int]:
    n = len(s)
    sa = list(range(n))
    rank = list(s)
    k = 1
    while k < max(n, 1):
        keys = [(rank[i], rank[i + k] + 1 if i + k < n else 0) for i in range(n)]
        sa.sort(key=lambda i: keys[i])
        new_rank = [0] * n
        for w in range(1, n):
            new_rank[sa[w]] = new_rank[sa[w - 1]] + int(keys[sa[w]] != keys[sa[w - 1]])
        rank = new_rank
        if rank[sa[n - 1]] == n - 1:
            break
        k *= 2
    return sa


class Tests(unittest.TestCase):
    def test_dc3_matches_suffix_array_across_random_and_periodic_strings(self) -> None:
        r = Rng(3202)
        cases: list[bytes] = [
            b"",
            b"a",
            b"aa",
            b"ab",
            b"ba",
            b"aaaaaa",
            b"banana",
            b"mississippi",
            b"abracadabra",
        ]
        f0, f1 = b"a", b"b"
        for _ in range(9):
            nxt = f1 + f0
            cases.append(nxt)
            f0, f1 = f1, nxt
        for _ in range(80):
            n = r.range(1, 90)
            alpha = r.range(2, 6)
            s = bytes(ord("a") + r.range(0, alpha - 1) for _ in range(n))
            cases.append(s)
        for s in cases:
            got = dc3_suffix_array(s)
            want = _suffix_array_prefix_doubling(s)
            self.assertEqual(got, want, f"DC3 mismatch for {s!r}")

    def test_bwt_encode_decode_roundtrips_for_all_lengths_and_arbitrary_bytes(self) -> None:
        r = Rng(3203)
        for length in range(121):
            s = bytes(r.range(0, 255) for _ in range(length))
            bwt, srow = bwt_encode(s)
            self.assertEqual(len(bwt), len(s))
            decoded = bwt_decode(bwt, srow)
            self.assertEqual(decoded, s, f"roundtrip failed for len={length}")
        bwt, srow = bwt_encode(b"banana")
        self.assertEqual(bwt, b"annbaa")
        self.assertEqual(srow, 4)
        self.assertEqual(bwt_decode(bwt, srow), b"banana")


if __name__ == "__main__":
    print(demo(), end="")
