# String Matching with Gap Characters

## What it is
Exercise 32.1-4 considers a pattern `P` that contains a special **gap character** `◊` (written `*` here) that matches an *arbitrary string of zero or more characters* — for example `ab*ba*c` occurs in `cabccbacbac` as `ab·cc·ba·cb·a… c`. Gap characters may appear anywhere, any number of times, but not in the text. The task is to decide whether `P` occurs in the text in `O(n + m)`-ish time.

## How it works
Cut the pattern at the gaps into **gap-free pieces** `P₁, P₂, …, P_k`. The pattern occurs iff the pieces occur in the text **in this order without overlapping**. A *greedy* argument shows that placing each piece at its **leftmost** possible position after the previous one never hurts: any valid placement can be shifted left. So

1. search for `P₁` from position 0 with a linear-time matcher ([KMP](#/kmp));
2. continue searching for `P₂` after the end of the found `P₁`, and so on;
3. the answer is "yes" iff every piece is found.

Because each search resumes where the previous one stopped, the total scanned text is at most `n` — linear overall (more precisely `O(n + m)` including the pattern pieces).

```mermaid
flowchart LR
  P["ab*ba*c"] --> S["pieces: ab | ba | c"]
  S --> F1["find ab"] --> F2["find ba after it"] --> F3["find c after it"] --> Y["occurs"]
```

## Complexity
`O(n + m)` with KMP for the pieces.

## When to use it
Glob-style searches (`foo*bar*baz`) in logs and file names, simple wildcard queries, and as a stepping stone to full regular-expression matching with the [finite automaton](#/finite_automaton_matcher) approach.

## In this implementation
Tests compare with exhaustive glob matching over all substrings on 500 random text/pattern pairs, plus edge cases (empty text, all gaps, wrong order).
