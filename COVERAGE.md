# Coverage checklist (CLRS 3rd edition end-of-chapter Problems)

The table lists every numbered **Problem** of the 3rd edition (titles taken from the public solutions index at <https://walkccc.me/CLRS/>) and where it lives in this repository. Section-level material (the algorithms in the main text of all four editions) is covered by the other families listed in the web app; this file is only about the Problems.

* ✅ — implemented, with tests, in the named algorithm family (the name is the file/route id: `src/algorithms/<category>/<id>.rs`, `#/<id>` in the web app).
* ➖ — the Problem is a pure analysis/proof exercise (asymptotics, recurrences, a table of running times) and has no algorithm to implement; the closest implemented material is named where it exists.
* ◐ — partly implemented; the note says what is missing.

Edition-specific problems of the 1st, 2nd and 4th editions (circuits, PRAM, sorting networks, machine learning, online algorithms, matching) are covered by their own families and are not part of this table; the 4th edition renumbers many problems.

| Ch | Problem | Title | Status | Implemented in |
|---|---|---|---|---|
| 1 | 1-1 | Comparison of running times | ➖ | analysis only (table) |
| 2 | 2-1 | Insertion sort on small arrays in merge sort | ✅ | `insertion_merge_hybrids` |
| 2 | 2-2 | Correctness of bubblesort | ✅ | `bubble_sort` |
| 2 | 2-3 | Correctness of Horner's rule | ✅ | `horner` |
| 2 | 2-4 | Inversions | ✅ | `inversions` |
| 3 | 3-1…3-6 | Asymptotic notation problems | ➖ | analysis only (no algorithm) |
| 4 | 4-1, 4-3 | Recurrence examples | ✅ | `master_theorem`, `akra_bazzi` |
| 4 | 4-2 | Parameter-passing costs | ➖ | analysis only |
| 4 | 4-4 | Fibonacci numbers | ◐ | `fibonacci_algorithms` — generating-function derivation not coded |
| 4 | 4-5 | Chip testing | ✅ | `chip_testing` |
| 4 | 4-6 | Monge arrays | ✅ | `monge_arrays` |
| 5 | 5-1 | Probabilistic counting | ✅ | `probabilistic_counting` |
| 5 | 5-2 | Searching an unsorted array | ✅ | `random_search` |
| 6 | 6-1 | Building a heap using insertion | ✅ | `heap_build_by_insertion` |
| 6 | 6-2 | Analysis of d-ary heaps | ✅ | `d_ary_heap` |
| 6 | 6-3 | Young tableaus | ✅ | `young_tableau` |
| 7 | 7-1 | Hoare partition correctness | ✅ | `hoare_quicksort` |
| 7 | 7-2 | Quicksort with equal element values | ✅ | `quicksort_variants` |
| 7 | 7-3 | Alternative quicksort analysis | ➖ | analysis only — see `randomized_quicksort` |
| 7 | 7-4 | Stack depth for quicksort | ✅ | `quicksort_variants` |
| 7 | 7-5 | Median-of-3 partition | ✅ | `quicksort_variants` |
| 7 | 7-6 | Fuzzy sorting of intervals | ✅ | `fuzzy_sort` |
| 8 | 8-1 | Probabilistic lower bounds on comparison sorting | ➖ | analysis only — see `decision_tree_lower_bound` |
| 8 | 8-2 | Sorting in place in linear time | ✅ | `sort_in_place_linear` |
| 8 | 8-3 | Sorting variable-length items | ✅ | `variable_length_sort` |
| 8 | 8-4 | Water jugs | ✅ | `water_jugs` |
| 8 | 8-5 | Average sorting | ✅ | `average_sorting` |
| 8 | 8-6 | Lower bound on merging sorted lists | ✅ | `merging_lower_bound` |
| 8 | 8-7 | The 0-1 sorting lemma and columnsort | ✅ | `columnsort`, `comparison_network` |
| 9 | 9-1 | Largest i numbers in sorted order | ✅ | `largest_i_numbers` |
| 9 | 9-2 | Weighted median | ✅ | `weighted_median` |
| 9 | 9-3 | Small order statistics | ✅ | `small_order_statistics` |
| 9 | 9-4 | Alternative analysis of randomized selection | ➖ | analysis only — see `randomized_select` |
| 10 | 10-1 | Comparisons among lists | ➖ | analysis only (table) |
| 10 | 10-2 | Mergeable heaps using linked lists | ✅ | `mergeable_heaps_lists` |
| 10 | 10-3 | Searching a sorted compact list | ✅ | `compact_list_search` |
| 11 | 11-1, 11-2 | Longest-probe bound; slot-size bound for chaining | ✅ | `hashing_bounds` |
| 11 | 11-3 | Quadratic probing | ✅ | `open_addressing` |
| 11 | 11-4 | Hashing and authentication | ✅ | `hashing_authentication` |
| 12 | 12-1 | Binary search trees with equal keys | ✅ | `equal_keys_bst` |
| 12 | 12-2 | Radix trees | ✅ | `radix_tree` |
| 12 | 12-3 | Average node depth in a randomly built BST | ✅ | `randomly_built_bst` |
| 12 | 12-4 | Number of different binary trees | ✅ | `binary_tree_counting` |
| 13 | 13-1, 13-2 | Persistent dynamic sets; join on red-black trees | ✅ | `rb_join_split` |
| 13 | 13-3 | AVL trees | ✅ | `avl_tree` |
| 13 | 13-4 | Treaps | ✅ | `treap` |
| 14 | 14-1, 14-2 | Point of maximum overlap; Josephus permutation | ✅ | `augmentation_problems` |
| 15 | 15-1 | Longest simple path in a DAG | ✅ | `longest_simple_path_dag` |
| 15 | 15-2 | Longest palindrome subsequence | ✅ | `longest_palindromic_subsequence` |
| 15 | 15-3 | Bitonic Euclidean TSP | ✅ | `bitonic_tsp` |
| 15 | 15-4 | Printing neatly | ✅ | `printing_neatly` |
| 15 | 15-5 | Edit distance | ✅ | `edit_distance` |
| 15 | 15-6, 15-7 | Company party; Viterbi algorithm | ✅ | `dp_problems` |
| 15 | 15-8 | Image compression by seam carving | ✅ | `seam_carving` |
| 15 | 15-9 | Breaking a string | ✅ | `breaking_a_string` |
| 15 | 15-10 | Planning an investment strategy | ✅ | `investment_strategy` |
| 15 | 15-11 | Inventory planning | ✅ | `inventory_planning` |
| 15 | 15-12 | Signing free-agent baseball players | ✅ | `free_agents` |
| 16 | 16-1 | Coin changing | ✅ | `coin_change` |
| 16 | 16-2 | Scheduling to minimize average completion time | ✅ | `min_avg_completion_time` |
| 16 | 16-3 | Acyclic subgraphs | ✅ | `acyclic_subgraphs` |
| 16 | 16-4 | Scheduling variations | ✅ | `unit_task_scheduling_union_find` |
| 16 | 16-5 | Off-line caching | ✅ | `online_caching` |
| 17 | 17-1 | Bit-reversed binary counter | ✅ | `bit_reversal_counter` |
| 17 | 17-2 | Making binary search dynamic | ✅ | `dynamic_binary_search` |
| 17 | 17-3 | Amortized weight-balanced trees | ✅ | `scapegoat_tree` |
| 17 | 17-4 | The cost of restructuring red-black trees | ✅ | `red_black_tree` |
| 17 | 17-5 | Competitive analysis of move-to-front | ✅ | `move_to_front` |
| 18 | 18-1 | Stacks on secondary storage | ✅ | `stack_secondary_storage` |
| 18 | 18-2 | Joining and splitting 2-3-4 trees | ✅ | `two_three_four_tree` |
| 19 | 19-1 | Alternative implementation of deletion | ✅ | `fibonacci_heap` |
| 19 | 19-2 | Binomial trees and binomial heaps | ◐ | `binomial_heap` — chapter material; individual parts not separately exercised |
| 19 | 19-3 | More Fibonacci-heap operations | ◐ | `fibonacci_heap` — change-key done; FIB-HEAP-PRUNE not implemented |
| 19 | 19-4 | 2-3-4 heaps | ✅ | `two_three_four_heap` |
| 20 | 20-1 | Space requirements for van Emde Boas trees | ✅ | `rs_veb_tree` |
| 20 | 20-2 | y-fast tries | ✅ | `y_fast_trie` |
| 21 | 21-1…21-3 | Off-line minimum; depth determination; Tarjan's LCA | ✅ | `offline_disjoint_set_apps` |
| 22 | 22-1 | Classifying edges by BFS | ✅ | `bfs_edge_classification` |
| 22 | 22-2 | Articulation points, bridges, biconnected components | ✅ | `biconnected_components` |
| 22 | 22-3 | Euler tour | ✅ | `euler_tour` |
| 22 | 22-4 | Reachability | ✅ | `min_reachable_label` |
| 23 | 23-1 | Second-best minimum spanning tree | ✅ | `mst_kruskal` |
| 23 | 23-2 | MST in sparse graphs | ✅ | `boruvka` |
| 23 | 23-3 | Bottleneck spanning tree | ✅ | `bottleneck_spanning_tree` |
| 23 | 23-4 | Alternative MST algorithms | ✅ | `alternative_mst` |
| 24 | 24-1 | Yen's improvement to Bellman-Ford | ✅ | `yen_bellman_ford` |
| 24 | 24-2 | Nesting boxes | ✅ | `nesting_boxes` |
| 24 | 24-3 | Arbitrage | ✅ | `arbitrage` |
| 24 | 24-4 | Gabow's scaling algorithm | ✅ | `gabow_scaling` |
| 24 | 24-5 | Karp's minimum mean-weight cycle | ✅ | `minimum_mean_cycle` |
| 24 | 24-6 | Bitonic shortest paths | ✅ | `bitonic_shortest_paths` |
| 25 | 25-1 | Transitive closure of a dynamic graph | ✅ | `dynamic_transitive_closure` |
| 25 | 25-2 | Shortest paths in ε-dense graphs | ✅ | `epsilon_dense_shortest_paths` |
| 26 | 26-1 | Escape problem | ✅ | `escape_problem` |
| 26 | 26-2 | Minimum path cover | ✅ | `min_path_cover` |
| 26 | 26-3 | Algorithmic consulting | ✅ | `algorithmic_consulting` |
| 26 | 26-4 | Updating maximum flow | ✅ | `updating_max_flow` |
| 26 | 26-5 | Maximum flow by scaling | ✅ | `scaling_max_flow` |
| 26 | 26-6 | Hopcroft-Karp bipartite matching | ✅ | `hopcroft_karp` |
| 27 | 27-1 | Implementing parallel loops using nested parallelism | ✅ | `p_parallel_loops` |
| 27 | 27-2 | Saving temporary space in matrix multiplication | ✅ | `p_matmul_no_temp` |
| 27 | 27-3 | Multithreaded matrix algorithms | ◐ | `p_lu_decomposition` — transpose and LU; LUP and Cholesky variants not implemented |
| 27 | 27-4 | Multithreading reductions and prefix computations | ✅ | `p_reduce_scan` |
| 27 | 27-5 | Multithreading a simple stencil calculation | ✅ | `p_stencil` |
| 27 | 27-6 | Randomized multithreaded algorithms | ✅ | `p_randomized_select`, `p_quicksort` |
| 28 | 28-1 | Tridiagonal systems of linear equations | ✅ | `tridiagonal_solver` |
| 28 | 28-2 | Splines | ✅ | `cubic_spline` |
| 29 | 29-1, 29-2, 29-4 | Linear-inequality feasibility; complementary slackness; Farkas' lemma | ✅ | `lp_feasibility_farkas` |
| 29 | 29-3 | Integer linear programming | ✅ | `integer_linear_programming` |
| 29 | 29-5 | Minimum-cost circulation | ✅ | `min_cost_circulation` |
| 30 | 30-1 | Divide-and-conquer multiplication | ✅ | `karatsuba` |
| 30 | 30-2 | Toeplitz matrices | ✅ | `toeplitz_multiply` |
| 30 | 30-3 | Multidimensional FFT | ✅ | `multidimensional_fft` |
| 30 | 30-4 | Evaluating all derivatives at a point | ✅ | `all_derivatives` |
| 30 | 30-5 | Polynomial evaluation at multiple points | ✅ | `chirp_transform` |
| 30 | 30-6 | FFT using modular arithmetic | ✅ | `number_theoretic_transform` |
| 31 | 31-1 | Binary gcd algorithm | ✅ | `euclid_gcd` |
| 31 | 31-2 | Analysis of bit operations in Euclid's algorithm | ✅ | `euclid_bit_operations` |
| 31 | 31-3 | Three algorithms for Fibonacci numbers | ✅ | `fibonacci_algorithms` |
| 31 | 31-4 | Quadratic residues | ✅ | `quadratic_residues` |
| 32 | 32-1 | String matching based on repetition factors | ✅ | `repetition_factor` |
| 33 | 33-1 | Convex layers | ✅ | `convex_layers` |
| 33 | 33-2 | Maximal layers | ✅ | `maximal_layers` |
| 33 | 33-3 | Ghostbusters and ghosts | ✅ | `ghostbusters` |
| 33 | 33-4 | Picking up sticks | ✅ | `picking_up_sticks` |
| 33 | 33-5 | Sparse-hulled distributions | ✅ | `sparse_hulled_distributions` |
| 34 | 34-1 | Independent set | ✅ | `independent_set` |
| 34 | 34-2 | Bonnie and Clyde | ✅ | `bonnie_and_clyde` |
| 34 | 34-3 | Graph coloring | ✅ | `graph_coloring` |
| 34 | 34-4 | Scheduling with profits and deadlines | ✅ | `scheduling_profits_deadlines` |
| 35 | 35-1 | Bin packing | ✅ | `bin_packing` |
| 35 | 35-2 | Approximating the size of a maximum clique | ✅ | `clique_graph_product` |
| 35 | 35-3 | Weighted set-covering problem | ✅ | `weighted_set_cover` |
| 35 | 35-4 | Maximum matching | ✅ | `matching_approximation` |
| 35 | 35-5 | Parallel machine scheduling | ✅ | `list_scheduling` |
| 35 | 35-6 | Approximating a maximum spanning tree | ✅ | `max_spanning_tree_approx` |
| 35 | 35-7 | Approximation algorithm for 0-1 knapsack | ✅ | `knapsack_approximation` |

**Summary:** 126 entries fully implemented, 4 partly implemented, 7 analysis-only entries without an algorithm.

## Known gaps

* **19-3** `FIB-HEAP-PRUNE` (delete `min(r, n)` nodes in amortised `O(r)`) is not implemented; change-key is.
* **19-2** is covered by the binomial-heap chapter material, but its individual parts (a)–(e) are not separately exercised.
* **27-3** only the transpose and LU parts; LUP and Cholesky variants are missing.
* Many exercises (as opposed to Problems) are one-line variations or proofs and are not individually listed; the 1st/2nd/4th edition lists were not audited line by line against the books.
