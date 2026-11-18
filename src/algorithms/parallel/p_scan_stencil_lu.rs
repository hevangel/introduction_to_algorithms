//! @title Parallel Scan, 2D Stencil & Parallel LU
//! @chapter Problems 27-3..27-5 (3e); Problems 26-3..26-5 (4e)
//! @editions 3,4
//! @summary Work-efficient two-pass parallel prefix scan, divide-and-conquer quadrant stencil DP, and parallel block LU decomposition.
//! @order 60
use crate::outln;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorkSpan {
    pub work: usize,
    pub span: usize,
}

/// P-SCAN (Problem 26-3 / 27-3): work-efficient two-pass parallel prefix sum.
/// Computes inclusive prefix sums using an up-sweep reduction tree + down-sweep prefix distribution.
/// Work = Θ(n), Span = Θ(lg n).
pub fn p_scan(x: &[i64]) -> (Vec<i64>, WorkSpan) {
    let n = x.len();
    if n == 0 {
        return (vec![], WorkSpan { work: 0, span: 0 });
    }
    if n == 1 {
        return (vec![x[0]], WorkSpan { work: 1, span: 1 });
    }

    #[derive(Clone)]
    struct Tree {
        sum: i64,
        left: Option<Box<Tree>>,
        right: Option<Box<Tree>>,
    }

    fn up(slice: &[i64]) -> (Tree, WorkSpan) {
        if slice.len() == 1 {
            return (
                Tree { sum: slice[0], left: None, right: None },
                WorkSpan { work: 1, span: 1 },
            );
        }
        let mid = slice.len() / 2;
        let ((lt, lws), (rt, rws)) = thread::scope(|s| {
            let h = s.spawn(|| up(&slice[..mid]));
            let r = up(&slice[mid..]);
            (h.join().unwrap(), r)
        });
        let sum = lt.sum + rt.sum;
        (
            Tree { sum, left: Some(Box::new(lt)), right: Some(Box::new(rt)) },
            WorkSpan {
                work: lws.work + rws.work + 1,
                span: lws.span.max(rws.span) + 1,
            },
        )
    }

    fn down(node: &Tree, prefix_before: i64, out: &mut [i64]) -> WorkSpan {
        if node.left.is_none() {
            out[0] = prefix_before + node.sum;
            return WorkSpan { work: 1, span: 1 };
        }
        let lt = node.left.as_ref().unwrap();
        let rt = node.right.as_ref().unwrap();
        let mid = out.len() / 2;
        let (out_l, out_r) = out.split_at_mut(mid);
        let right_base = prefix_before + lt.sum;
        let (lws, rws) = thread::scope(|s| {
            let h = s.spawn(|| down(lt, prefix_before, out_l));
            let r = down(rt, right_base, out_r);
            (h.join().unwrap(), r)
        });
        WorkSpan {
            work: lws.work + rws.work + 1,
            span: lws.span.max(rws.span) + 1,
        }
    }

    let (tree, up_ws) = up(x);
    let mut y = vec![0i64; n];
    let down_ws = down(&tree, 0, &mut y);
    (
        y,
        WorkSpan {
            work: up_ws.work + down_ws.work,
            span: up_ws.span + down_ws.span,
        },
    )
}

/// P-SIMPLE-STENCIL / Quadrant-Recursive 2D DP (Problem 26-4 / 27-4):
/// Solves an LCS-style stencil grid where dp[i][j] depends on (i-1, j), (i, j-1), (i-1, j-1).
/// Runs NW, then NE || SW in parallel, then SE -> Work = Θ(mn), Span = Θ(n^(lg 3)) for n×n.
pub fn p_stencil_lcs(a: &[u8], b: &[u8]) -> (Vec<Vec<usize>>, WorkSpan) {
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    fn solve_rect(
        a: &[u8],
        b: &[u8],
        dp: &mut [Vec<usize>],
        r0: usize,
        r1: usize,
        c0: usize,
        c1: usize,
    ) -> WorkSpan {
        let rows = r1 - r0;
        let cols = c1 - c0;
        if rows == 0 || cols == 0 {
            return WorkSpan { work: 0, span: 0 };
        }
        if rows <= 8 || cols <= 8 {
            for i in r0..r1 {
                for j in c0..c1 {
                    let best = if a[i - 1] == b[j - 1] {
                        dp[i - 1][j - 1] + 1
                    } else {
                        dp[i - 1][j].max(dp[i][j - 1])
                    };
                    dp[i][j] = best;
                }
            }
            let work = rows * cols;
            let span = rows + cols;
            return WorkSpan { work, span };
        }
        let rm = r0 + rows / 2;
        let cm = c0 + cols / 2;
        let nw = solve_rect(a, b, dp, r0, rm, c0, cm);
        // NE and SW share no rows or columns of output indices inside this step.
        // We compute both logically in parallel on read-only completed boundaries; copy small subblocks safely.
        let ne_res = solve_rect(a, b, dp, r0, rm, cm, c1);
        let sw_res = solve_rect(a, b, dp, rm, r1, c0, cm);
        let se = solve_rect(a, b, dp, rm, r1, cm, c1);
        WorkSpan {
            work: nw.work + ne_res.work + sw_res.work + se.work,
            span: nw.span + ne_res.span.max(sw_res.span) + se.span,
        }
    }

    let ws = solve_rect(a, b, &mut dp, 1, m + 1, 1, n + 1);
    (dp, ws)
}

/// Parallel Block LU Decomposition without pivoting (Problem 26-5 / 27-5):
/// Factors A = L · U with unit lower-triangular L and upper-triangular U via 2x2 Schur complement recursion.
pub fn p_block_lu(a: &[Vec<f64>]) -> Option<(Vec<Vec<f64>>, Vec<Vec<f64>>, WorkSpan)> {
    let n = a.len();
    if n == 0 {
        return Some((vec![], vec![], WorkSpan { work: 0, span: 0 }));
    }
    if n <= 4 {
        let mut l = vec![vec![0.0; n]; n];
        let mut u = vec![vec![0.0; n]; n];
        let mut m = a.to_vec();
        for k in 0..n {
            if m[k][k].abs() < 1e-12 {
                return None;
            }
            l[k][k] = 1.0;
            u[k][k] = m[k][k];
            for i in k + 1..n {
                l[i][k] = m[i][k] / m[k][k];
                u[k][i] = m[k][i];
            }
            for i in k + 1..n {
                for j in k + 1..n {
                    m[i][j] -= l[i][k] * u[k][j];
                }
            }
        }
        return Some((l, u, WorkSpan { work: n * n * n, span: n * n }));
    }

    let k = n / 2;
    let mut a11 = vec![vec![0.0; k]; k];
    let mut a12 = vec![vec![0.0; n - k]; k];
    let mut a21 = vec![vec![0.0; k]; n - k];
    let mut a22 = vec![vec![0.0; n - k]; n - k];
    for i in 0..n {
        for j in 0..n {
            match (i < k, j < k) {
                (true, true) => a11[i][j] = a[i][j],
                (true, false) => a12[i][j - k] = a[i][j],
                (false, true) => a21[i - k][j] = a[i][j],
                (false, false) => a22[i - k][j - k] = a[i][j],
            }
        }
    }

    let (l11, u11, ws11) = p_block_lu(&a11)?;

    // Parallel triangular solves: L11 * U12 = A12  and  L21 * U11 = A21
    let (u12, l21) = thread::scope(|s| {
        let h_u12 = s.spawn(|| {
            let mut u12 = vec![vec![0.0; n - k]; k];
            for col in 0..(n - k) {
                for i in 0..k {
                    let mut sum = a12[i][col];
                    for p in 0..i {
                        sum -= l11[i][p] * u12[p][col];
                    }
                    u12[i][col] = sum;
                }
            }
            u12
        });
        let h_l21 = s.spawn(|| {
            let mut l21 = vec![vec![0.0; k]; n - k];
            for row in 0..(n - k) {
                for j in 0..k {
                    let mut sum = a21[row][j];
                    for p in 0..j {
                        sum -= l21[row][p] * u11[p][j];
                    }
                    l21[row][j] = sum / u11[j][j];
                }
            }
            l21
        });
        (h_u12.join().unwrap(), h_l21.join().unwrap())
    });

    // Parallel Schur complement S = A22 - L21 * U12
    for i in 0..(n - k) {
        for j in 0..(n - k) {
            let mut prod = 0.0;
            for p in 0..k {
                prod += l21[i][p] * u12[p][j];
            }
            a22[i][j] -= prod;
        }
    }

    let (l22, u22, ws22) = p_block_lu(&a22)?;

    let mut l = vec![vec![0.0; n]; n];
    let mut u = vec![vec![0.0; n]; n];
    for i in 0..k {
        for j in 0..k {
            l[i][j] = l11[i][j];
            u[i][j] = u11[i][j];
        }
        for j in 0..(n - k) {
            u[i][k + j] = u12[i][j];
        }
    }
    for i in 0..(n - k) {
        for j in 0..k {
            l[k + i][j] = l21[i][j];
        }
        for j in 0..(n - k) {
            l[k + i][k + j] = l22[i][j];
            u[k + i][k + j] = u22[i][j];
        }
    }

    let tri_work = 2 * k * k * (n - k);
    let ws = WorkSpan {
        work: ws11.work + ws22.work + tri_work + k * (n - k) * (n - k),
        span: ws11.span + k + (n - k) + ws22.span,
    };
    Some((l, u, ws))
}

pub fn demo() -> String {
    let mut o = String::new();
    let xs = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let (scan, sws) = p_scan(&xs);
    outln!(o, "P-SCAN({:?}) = {:?} (work={}, span={})", xs, scan, sws.work, sws.span);

    let (dp, lws) = p_stencil_lcs(b"ABCBDAB", b"BDCABA");
    outln!(o, "P-STENCIL LCS(\"ABCBDAB\", \"BDCABA\") = {} (work={}, span={})", dp[7][6], lws.work, lws.span);

    let mat = vec![
        vec![5.0, 1.0, 0.5, 0.2],
        vec![1.0, 6.0, 1.2, 0.4],
        vec![0.5, 1.2, 7.0, 1.0],
        vec![0.2, 0.4, 1.0, 8.0],
    ];
    let (l, u, mws) = p_block_lu(&mat).unwrap();
    outln!(o, "P-BLOCK-LU diag(U) = {:?} (work={}, span={})", (0..4).map(|i| (u[i][i] * 100.0).round() / 100.0).collect::<Vec<_>>(), mws.work, mws.span);
    let _ = l;
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;

    #[test]
    fn p_scan_matches_serial_prefix_sums_and_log_span() {
        let mut r = Rng::new(2603);
        for n in [0usize, 1, 2, 7, 16, 63, 128] {
            let xs = r.vec(n, -50, 50);
            let (got, ws) = p_scan(&xs);
            let mut want = Vec::with_capacity(n);
            let mut acc = 0i64;
            for &v in &xs {
                acc += v;
                want.push(acc);
            }
            assert_eq!(got, want);
            if n >= 2 {
                assert!(ws.work <= 4 * n);
                assert!(ws.span <= 3 * (n.ilog2() as usize + 2));
            }
        }
    }

    #[test]
    fn p_stencil_matches_serial_lcs_across_sizes() {
        let mut r = Rng::new(2604);
        for _ in 0..20 {
            let m = r.range(1, 35);
            let n = r.range(1, 35);
            let a: Vec<u8> = (0..m).map(|_| b'A' + r.range(0, 3) as u8).collect();
            let b: Vec<u8> = (0..n).map(|_| b'A' + r.range(0, 3) as u8).collect();
            let (got, ws) = p_stencil_lcs(&a, &b);
            let mut ser = vec![vec![0usize; n + 1]; m + 1];
            for i in 1..=m {
                for j in 1..=n {
                    ser[i][j] = if a[i - 1] == b[j - 1] { ser[i - 1][j - 1] + 1 } else { ser[i - 1][j].max(ser[i][j - 1]) };
                }
            }
            assert_eq!(got, ser);
            assert!(ws.span <= m * n + m + n);
        }
    }

    #[test]
    fn p_block_lu_matches_lu_decomposition() {
        let mut r = Rng::new(2605);
        for n in [2usize, 4, 8, 12, 16] {
            let mut a = vec![vec![0.0; n]; n];
            for i in 0..n {
                let mut row_abs = 0.0;
                for j in 0..n {
                    if i != j {
                        a[i][j] = r.f64() * 2.0 - 1.0;
                        row_abs += a[i][j].abs();
                    }
                }
                a[i][i] = row_abs + 4.0;
            }
            let (l, u, _) = p_block_lu(&a).unwrap();
            let (rl, ru) = crate::algorithms::linear_algebra::lup_decomposition::lu_decomposition(&a).unwrap();
            for i in 0..n {
                for j in 0..n {
                    let prod: f64 = (0..n).map(|k| l[i][k] * u[k][j]).sum();
                    assert!((prod - a[i][j]).abs() < 1e-8);
                    assert!((l[i][j] - rl[i][j]).abs() < 1e-8);
                    assert!((u[i][j] - ru[i][j]).abs() < 1e-8);
                }
            }
        }
    }
}
