# Machine Learning

## Introduction
Introduced brand-new in the **4th edition of CLRS (2022, Ch. 33)**, Machine Learning bridges classical algorithm analysis with modern data science. Rather than treating ML as black-box library calls, CLRS focuses on the **algorithmic heart** of three foundational paradigms: **unsupervised clustering**, **online prediction from experts**, and **continuous optimization by gradient / Newton methods**—complete with explicit convergence and regret proofs!

## What Problem Is It Solving?
- **Group unlabeled points into $k$ clusters:** Partition $n$ vectors into $k$ cohesive groups minimizing squared distance to cluster centers ([k-Means / Lloyd's Algorithm](#/k_means)).
- **Learn online from competing advisors ("experts"):** Predict outcomes sequentially while making almost as few mistakes as the best single expert in hindsight ([Weighted-Majority Algorithm](#/weighted_majority), [Hedge Multiplicative Weights](#/newton_hedge_sgd)).
- **Minimize smooth loss functions $f(w)$:** Iteratively adjust model parameters $w$ toward a minimum when solving $\nabla f(w) = 0$ directly is too huge or non-linear ([Gradient Descent](#/gradient_descent), [Newton's Method & Stochastic Gradient Descent (SGD)](#/newton_hedge_sgd)).

```mermaid
flowchart TD
  ML{"ML Problem Type"} -->|Unlabeled data grouping| KM["k-Means (Lloyd): O(t·n·k·d)\nLocal minimum, depends on init"]
  ML -->|Sequential expert advice| WM["Weighted Majority / Hedge\nMultiplicative weights -> O(√(T ln N)) regret"]
  ML -->|Smooth optimization min f(w)| OPT{"Cost per step vs Steps?"}
  OPT -->|Cheap steps, huge data n| SGD["SGD: O(d) per batch step\nNoisy but scalable"]
  OPT -->|Moderate d, fast local polish| GD["Full Gradient Descent / Newton\nLinear vs Quadratic convergence"]
```

## Algorithms & Trade-Offs

| Algorithm | Cost per Iteration | Convergence / Guarantee | Key Trade-Off |
|---|---|---|---|
| [k-Means (Lloyd's Algorithm)](#/k_means) | $O(n \cdot k \cdot d)$ | Monotonically decreases SSE; finds a **local** minimum (exact k-means is NP-hard!) | Super fast and intuitive alternating assignment/centroid updates, **but** sensitive to initial centroids (`k-means++` helps) and assumes roughly spherical clusters of similar size. |
| [Weighted-Majority](#/weighted_majority) vs. [Hedge](#/newton_hedge_sgd) | $O(N)$ over $N$ experts | Mistake bound $2.41 \cdot (m^* + \lg N)$ vs. $(1+\varepsilon)\text{OPT} + \frac{\ln N}{\varepsilon}$ | Deterministic Weighted Majority predicts deterministically and can suffer $2\times$ mistakes on adversarial ties; **Hedge** maintains a probability distribution over experts and achieves $(1+\varepsilon)$ competitive regret against the best expert! |
| **Full-Batch** [Gradient Descent](#/gradient_descent) vs. [Stochastic GD (SGD)](#/newton_hedge_sgd) | $O(n \cdot d)$ vs. **$O(b \cdot d)$** (mini-batch $b \ll n$) | Deterministic decrease vs. noisy expected descent | Full GD computes the exact gradient over all $n$ training examples—smooth steps, **but** painfully slow when $n = 10^8$. SGD updates on tiny random batches, making massive early progress at the cost of variance near the optimum (needs decaying step size $\eta_t$). |
| [Gradient Descent](#/gradient_descent) vs. [Newton's Method](#/newton_hedge_sgd) | $O(d)$ (first-order) vs. $O(d^3)$ (Hessian inverse) | Linear ($O(\lg(1/\varepsilon))$) vs. **Quadratic** ($O(\lg\lg(1/\varepsilon))$) | Newton uses curvature ($f''(x)$ or Hessian $H^{-1}$) so error digits **double** each step near a strictly convex minimum, **but** building and inverting a $d \times d$ Hessian is impossible when $d = 10^6$ neural-net weights! |

## Quick Rules of Thumb for CS Students
- **Step size $\eta$ makes or breaks GD:** Too small $\implies$ crawls forever; too large $\implies$ overshoots the valley and diverges to $\infty$!
- **Why multiplicative weights beat additive updates:** Halving an expert's weight on every mistake (or multiplying by $e^{-\varepsilon \ell}$) means bad experts vanish exponentially fast (`2^{-m}`), giving $O(\lg N)$ dependence on the number of experts even when $N = 1,000,000$!
- **Look at dimension $d$ vs. sample count $n$:** Large $n$, moderate $d \implies$ **SGD**; huge $d$ $\implies$ first-order **GD/SGD** (never $O(d^3)$ Newton); tiny $d$ needing 15 digits of precision $\implies$ **Newton's Method**.
