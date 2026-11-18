# String Matching

## Introduction
Whenever you hit `Ctrl+F` in a browser, run `grep`, or align DNA sequences against a 3-billion-letter genome, you are solving **String Matching** (CLRS Ch. 32; `Ch. 34` in 1e). The naive loop works fine on small English text, **but** gets crushed into $O(nm)$ time by repetitive inputs like `"aaaaaa...ab"` and completely stalls if you have to search the *same* giant genome thousands of times.

## What Problem Is It Solving?
- **Exact online pattern search (preprocess the pattern $P$ of length $m$, stream text $T$ of length $n$):** Find every shift where $P$ occurs inside $T$ without backing up or paying $O(nm)$ ([Naive](#/naive_string_matcher), [Rabin-Karp](#/rabin_karp), [Finite Automaton](#/finite_automaton_matcher), [Knuth-Morris-Pratt (KMP)](#/kmp), [Boyer-Moore](#/boyer_moore), [2D Pattern Matching](#/two_d_string_matching), [Gap Matching](#/gap_string_matching)).
- **Index the *text* $T$ once so queries take $O(m \lg n)$ (4th edition §32.5 + Problems 32-2/32-3):** Build a compact sorted suffix index and invertibly compress text ([Suffix Arrays & LCP](#/suffix_array), [Linear-Time DC3 Suffix Array & Burrows-Wheeler Transform](#/burrows_wheeler_linear_sa)).

```mermaid
flowchart TD
  Q{"Are you preprocessing the\npattern P or the giant text T?"} -->|Pattern P (stream text T once)| PAT["Online Matchers\nKMP: O(n+m) always · Rabin-Karp: rolling hash · Boyer-Moore: sublinear average"]
  Q -->|Giant Text T (many queries / compression)| TXT["Text Index\nSuffix Array + LCP · DC3 O(n) build · BWT"]
```

## Algorithms & Trade-Offs

| Algorithm | Preprocessing | Search Time | Space | Key Trade-Off |
|---|---|---|---|---|
| [Naive Matcher](#/naive_string_matcher) | $0$ | $\Theta((n-m+1)m)$ worst; $O(n)$ on random English | $O(1)$ | Zero setup and great on short random text, **but** quadratic on repetitive strings (`P = "aa...ab"` in `T = "aa...a"`). |
| [Rabin-Karp](#/rabin_karp) | $\Theta(m)$ | $\Theta(n+m)$ expected / $O(nm)$ worst | $O(1)$ | Uses a rolling polynomial hash so it easily generalizes to searching for **multiple patterns at once** or [2D images](#/two_d_string_matching), **but** needs character-by-character verification on hash hits. |
| [Finite Automaton](#/finite_automaton_matcher) | $O(m \cdot |\Sigma|)$ | **$\Theta(n)$** (strictly 1 step per text char!) | $O(m \cdot |\Sigma|)$ | Fastest possible scan loop once built, **but** transition table explodes if alphabet $|\Sigma|$ is huge (Unicode!). |
| [Knuth-Morris-Pratt (KMP)](#/kmp) | $\Theta(m)$ | **$\Theta(n)$** worst case | $\Theta(m)$ | Compresses the automaton into a single `π` failure-function array independent of alphabet size $|\Sigma|$—rock-solid linear worst-case guarantees. |
| [Boyer-Moore](#/boyer_moore) | $\Theta(m + |\Sigma|)$ | **$O(n/m)$ best/typical**; $O(nm)$ worst (Galil variant $O(n)$) | $\Theta(m + |\Sigma|)$ | Matches *right-to-left* so mismatches near the end jump forward up to $m$ characters at a time—fastest in practice for long patterns in `grep` (`1e §34.5`), though slightly trickier to implement. |
| [Suffix Array & LCP](#/suffix_array) / [DC3 + BWT](#/burrows_wheeler_linear_sa) | $O(n \lg n)$ prefix-doubling or **$O(n)$** DC3 | $O(m \lg n)$ or $O(m)$ | $\Theta(n)$ | Builds an index over the *entire* text $T$, paying off massively when querying thousands of patterns against one fixed corpus or compressing via [Burrows-Wheeler Transform](#/burrows_wheeler_linear_sa) (`bzip2`). |

## Quick Rules of Thumb for CS Students
- **Pattern changes often vs. Text is static:** Use KMP / Boyer-Moore when the *text* changes each time (`grep` on a stream); build a [Suffix Array](#/suffix_array) when the *text* is fixed (DNA, code search) and queries arrive repeatedly.
- **Multiple patterns or 2D blocks?** [Rabin-Karp](#/rabin_karp) wins effortlessly because a rolling hash lets you check hundreds of patterns in a single `HashSet` lookup per position.
- **Why BWT compresses so well:** Sorting cyclic shifts groups identical contexts together, turning scattered repeats into long identical character runs (`"oooo"`), while [LF-mapping](#/burrows_wheeler_linear_sa) reverses it losslessly with no dictionary table!
