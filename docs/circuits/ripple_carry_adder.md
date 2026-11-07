# Ripple-Carry Adder

## What it is
To add two `n`-bit binary numbers `a` and `b` chain `n` [full adders](#/combinational_circuits): stage `i` adds bits `aᵢ`, `bᵢ` and the carry `cᵢ` from the previous stage, producing sum bit `sᵢ` and carry `cᵢ₊₁`. This is the schoolbook algorithm cast in hardware.

## How it works
```mermaid
flowchart LR
  c0((c0 = 0)) --> FA0["FA₀<br/>a₀ b₀"] -- c1 --> FA1["FA₁<br/>a₁ b₁"] -- c2 --> FA2["FA₂<br/>a₂ b₂"] -- c3 --> FA3["FA₃<br/>a₃ b₃"] --> c4((c4 = carry out))
  FA0 --> s0((s0))
  FA1 --> s1((s1))
  FA2 --> s2((s2))
  FA3 --> s3((s3))
```
Correctness follows by induction on `i` using the invariant `Σ_{j<i} 2ʲ(aⱼ + bⱼ) = Σ_{j<i} 2ʲ sⱼ + 2ⁱ cᵢ`.

The **critical path** is the carry chain: `c_{i+1}` is available only after `c_i`, so depth grows by 2 per stage (AND then OR in each full adder).

## Complexity
Size `5n` gates; depth `2n + 1`: both `Θ(n)`.

## When to use it
Small word sizes, area-constrained designs, FPGAs (which have dedicated fast carry chains), and as the final stage inside larger multipliers when the operand is short. For speed use a [carry-lookahead adder](#/carry_lookahead_adder).

## In this implementation
`build_ripple` appends the adder to any `Circuit` so other designs ([array multiplier](#/multiplier_circuits)) can reuse it; tests assert exact gate count (`5n`), depth (`2n+1`), and correctness on random operands.
