//! @title Assembly-Line Scheduling
//! @chapter 15.1 (2e)
//! @editions 2
//! @summary The original DP example: fastest way through a two-line factory with transfer costs.
//! @order 20
use crate::outln;

/// FASTEST-WAY: `a[line][station]`, `t[line][station]` = time to move to the *other* line after `station`,
/// `e` entry times, `x` exit times. Returns (fastest total time, line used at each station).
pub fn fastest_way(a: [&[i64]; 2], t: [&[i64]; 2], e: [i64; 2], x: [i64; 2]) -> (i64, Vec<usize>) {
    let n = a[0].len();
    let mut f = [vec![0i64; n], vec![0i64; n]];
    let mut l = [vec![0usize; n], vec![0usize; n]]; // l[i][j]: line used at station j-1 on the fastest way to (i, j)
    f[0][0] = e[0] + a[0][0];
    f[1][0] = e[1] + a[1][0];
    for j in 1..n {
        for i in 0..2 {
            let o = 1 - i;
            let stay = f[i][j - 1] + a[i][j];
            let switch = f[o][j - 1] + t[o][j - 1] + a[i][j];
            if stay <= switch { f[i][j] = stay; l[i][j] = i; } else { f[i][j] = switch; l[i][j] = o; }
        }
    }
    let (best, mut line) = if f[0][n - 1] + x[0] <= f[1][n - 1] + x[1] { (f[0][n - 1] + x[0], 0) } else { (f[1][n - 1] + x[1], 1) };
    let mut path = vec![line; n];
    for j in (1..n).rev() {
        line = l[line][j];
        path[j - 1] = line;
    }
    (best, path)
}

pub fn demo() -> String {
    let mut o = String::new();
    let a1 = [7, 9, 3, 4, 8, 4];
    let a2 = [8, 5, 6, 4, 5, 7];
    let t1 = [2, 3, 1, 3, 4];
    let t2 = [2, 1, 2, 2, 1];
    let (best, path) = fastest_way([&a1, &a2], [&t1, &t2], [2, 4], [3, 2]);
    outln!(o, "fastest time through the factory = {}", best);
    for (j, line) in path.iter().enumerate() { outln!(o, "  station S{},{}", line + 1, j + 1); }
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn brute(a: [&[i64]; 2], t: [&[i64]; 2], e: [i64; 2], x: [i64; 2]) -> i64 {
        let n = a[0].len();
        (0..1u32 << n).map(|mask| {
            let line = |j: usize| ((mask >> j) & 1) as usize;
            let mut total = e[line(0)] + a[line(0)][0];
            for j in 1..n {
                if line(j) != line(j - 1) { total += t[line(j - 1)][j - 1]; }
                total += a[line(j)][j];
            }
            total + x[line(n - 1)]
        }).min().unwrap()
    }
    #[test]
    fn book_example() {
        let a1 = [7, 9, 3, 4, 8, 4];
        let a2 = [8, 5, 6, 4, 5, 7];
        let t1 = [2, 3, 1, 3, 4];
        let t2 = [2, 1, 2, 2, 1];
        let (best, path) = fastest_way([&a1, &a2], [&t1, &t2], [2, 4], [3, 2]);
        assert_eq!(best, 38);
        assert_eq!(path, vec![0, 1, 0, 1, 1, 0]);
    }
    #[test]
    fn matches_brute_force() {
        let mut r = Rng::new(100);
        for n in 1..=9 {
            let (a1, a2) = (r.vec(n, 1, 9), r.vec(n, 1, 9));
            let (t1, t2) = (r.vec(n.max(1) - 1, 1, 5), r.vec(n.max(1) - 1, 1, 5));
            let (e, x) = ([r.vec(1, 1, 5)[0], r.vec(1, 1, 5)[0]], [r.vec(1, 1, 5)[0], r.vec(1, 1, 5)[0]]);
            assert_eq!(fastest_way([&a1, &a2], [&t1, &t2], e, x).0, brute([&a1, &a2], [&t1, &t2], e, x));
        }
    }
}
