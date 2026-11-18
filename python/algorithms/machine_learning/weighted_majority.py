"""Weighted-Majority Algorithm — CLRS 33.2 (4e) (4e).

Learning from expert advice: shrink the weight of every wrong expert by (1-eps); mistakes <= 2(1+eps)m + (2 ln n)/eps.
"""

from __future__ import annotations

import math
from pathlib import Path
import sys
import unittest

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[3]))

from python.common import Rng


def weighted_majority(
    predictions: list[list[bool]],
    outcomes: list[bool],
    eps: float,
) -> tuple[int, int, list[float]]:
    """WEIGHTED-MAJORITY(predictions, outcomes, eps).

    predictions[t][i] is expert i's prediction in round t.
    Returns (learner's mistakes, mistakes of the best expert, final weights).
    """
    n = len(predictions[0])
    w = [1.0] * n
    mistakes = 0
    expert_mistakes = [0] * n
    for t, preds in enumerate(predictions):
        yes = sum(wi for p, wi in zip(preds, w) if p)
        no = sum(wi for p, wi in zip(preds, w) if not p)
        guess = yes >= no
        if guess != outcomes[t]:
            mistakes += 1
        for i in range(n):
            if preds[i] != outcomes[t]:
                w[i] *= 1.0 - eps
                expert_mistakes[i] += 1
    return mistakes, min(expert_mistakes), w


def mistake_bound(best_expert_mistakes: int, n: int, eps: float) -> float:
    """Theorem bound on the learner's mistakes: 2(1+eps)*m* + 2*ln(n)/eps."""
    return 2.0 * (1.0 + eps) * best_expert_mistakes + 2.0 * math.log(n) / eps


def demo() -> str:
    lines: list[str] = []
    r = Rng(5)
    n, rounds = 10, 400
    outcomes = [r.range(0, 1) == 1 for _ in range(rounds)]
    preds = [
        [
            (outcomes[t] if r.f64() < 0.9 else not outcomes[t]) if i == 0 else (r.range(0, 1) == 1)
            for i in range(n)
        ]
        for t in range(rounds)
    ]
    for eps in (0.5, 0.2, 0.1):
        m, best, w = weighted_majority(preds, outcomes, eps)
        heaviest = max(range(len(w)), key=lambda i: w[i])
        lines.append(
            f"ε = {eps:<3}: learner mistakes {m:>3}, best expert {best:>3}, "
            f"bound {mistake_bound(best, n, eps):>6.1f}, heaviest expert = {heaviest}"
        )
    lines.append("the learner quickly learns to trust expert 0; the regret against it stays within the bound")
    return "\n".join(lines) + "\n"


class Tests(unittest.TestCase):
    def test_mistake_bound_holds_on_random_instances(self) -> None:
        r = Rng(262)
        for _ in range(100):
            n, rounds = r.range(2, 12), r.range(10, 300)
            outcomes = [r.range(0, 1) == 1 for _ in range(rounds)]
            good = r.f64() * 0.5
            preds = [
                [
                    ((not outcomes[t]) if r.f64() < good else outcomes[t])
                    if i == 0
                    else (r.range(0, 1) == 1)
                    for i in range(n)
                ]
                for t in range(rounds)
            ]
            for eps in (0.5, 0.25, 0.1):
                m, best, _ = weighted_majority(preds, outcomes, eps)
                self.assertLessEqual(
                    m,
                    mistake_bound(best, n, eps) + 1e-9,
                    f"m={m} best={best} n={n} eps={eps}",
                )

    def test_perfect_expert_makes_learner_converge(self) -> None:
        rounds = 200
        outcomes = [t % 3 == 0 for t in range(rounds)]
        preds = [[outcomes[t], not outcomes[t], not outcomes[t], not outcomes[t], not outcomes[t]] for t in range(rounds)]
        m, best, w = weighted_majority(preds, outcomes, 0.5)
        self.assertEqual(best, 0)
        self.assertLessEqual(m, 3)
        self.assertEqual(w[0], 1.0)
        self.assertLess(w[1], 1e-30)


if __name__ == "__main__":
    print(demo(), end="")
