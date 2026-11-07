//! @title Master Theorem
//! @chapter 4.3 (1e–3e) / 4.5 (4e)
//! @editions 1,2,3,4
//! @summary Solve T(n)=aT(n/b)+Θ(n^k·lg^p n) by comparing the watershed n^(log_b a) with the driving function.
//! @order 120
use crate::outln;

/// Which case of the master theorem applies.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Case {
    One,
    Two,
    Three,
    /// f(n) differs from the watershed function by a non-polynomial factor (e.g. n lg n vs n).
    Gap,
}

/// Solve `T(n) = a·T(n/b) + Θ(n^k · (lg n)^p)`, returning the case and the Θ-bound as text.
pub fn solve(a: f64, b: f64, k: f64, p: f64) -> (Case, String) {
    let crit = a.ln() / b.ln(); // log_b a
    let fmt = |e: f64, lg: f64| -> String {
        let pow = if e.abs() < 1e-9 { String::new() } else if (e - 1.0).abs() < 1e-9 { "n".to_string() } else if (e - e.round()).abs() < 1e-9 { format!("n^{}", e.round() as i64) } else { format!("n^{:.3}", e) };
        let log = if lg.abs() < 1e-9 { String::new() } else if (lg - 1.0).abs() < 1e-9 { "lg n".to_string() } else { format!("lg^{} n", lg) };
        match (pow.is_empty(), log.is_empty()) {
            (true, true) => "1".into(),
            (true, false) => log,
            (false, true) => pow,
            _ => format!("{pow} {log}"),
        }
    };
    if (k - crit).abs() < 1e-9 {
        if p > -1.0 {
            (Case::Two, format!("Θ({})", fmt(k, p + 1.0)))
        } else if (p + 1.0).abs() < 1e-9 {
            (Case::Two, format!("Θ({} lg lg n)", fmt(k, 0.0)))
        } else {
            (Case::Two, format!("Θ({})", fmt(k, 0.0)))
        }
    } else if k < crit {
        (Case::One, format!("Θ({})", fmt(crit, 0.0)))
    } else if p >= 0.0 {
        // regularity holds for polynomially larger f(n) = n^k lg^p n
        (Case::Three, format!("Θ({})", fmt(k, p)))
    } else {
        (Case::Gap, "master theorem does not apply".into())
    }
}

/// Numerically evaluate T(n) = a·T(⌈n/b⌉) + n^k (n ≥ 1, T(1)=1), to sanity-check the asymptotics.
pub fn evaluate(a: u32, b: u32, k: f64, n: u64) -> f64 {
    if n <= 1 {
        return 1.0;
    }
    a as f64 * evaluate(a, b, k, (n + b as u64 - 1) / b as u64) + (n as f64).powf(k)
}

pub fn demo() -> String {
    let mut o = String::new();
    let rows = [
        ("merge sort            T=2T(n/2)+n", 2.0, 2.0, 1.0, 0.0),
        ("binary search         T=T(n/2)+1", 1.0, 2.0, 0.0, 0.0),
        ("Strassen              T=7T(n/2)+n²", 7.0, 2.0, 2.0, 0.0),
        ("recursive matrix mult T=8T(n/2)+n²", 8.0, 2.0, 2.0, 0.0),
        ("T=3T(n/4)+n lg n", 3.0, 4.0, 1.0, 1.0),
        ("T=2T(n/2)+n²", 2.0, 2.0, 2.0, 0.0),
        ("T=2T(n/2)+n/lg n", 2.0, 2.0, 1.0, -1.0),
    ];
    for (name, a, b, k, p) in rows {
        let (c, s) = solve(a, b, k, p);
        outln!(o, "{:<38} case {:?}: {}", name, c, s);
    }
    outln!(o, "\nnumeric check, T(n)=2T(n/2)+n: T(2^20)/(n lg n) = {:.3}", evaluate(2, 2, 1.0, 1 << 20) / ((1u64 << 20) as f64 * 20.0));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classic_recurrences() {
        assert_eq!(solve(2.0, 2.0, 1.0, 0.0), (Case::Two, "Θ(n lg n)".to_string()));
        assert_eq!(solve(1.0, 2.0, 0.0, 0.0), (Case::Two, "Θ(lg n)".to_string()));
        assert_eq!(solve(8.0, 2.0, 2.0, 0.0), (Case::One, "Θ(n^3)".to_string()));
        assert_eq!(solve(2.0, 2.0, 2.0, 0.0), (Case::Three, "Θ(n^2)".to_string()));
        assert_eq!(solve(7.0, 2.0, 2.0, 0.0).0, Case::One);
        assert_eq!(solve(2.0, 2.0, 1.0, -1.0), (Case::Two, "Θ(n lg lg n)".to_string()));
        assert_eq!(solve(2.0, 2.0, 1.0, -2.0).1, "Θ(n)");
    }
    #[test]
    fn numeric_agrees_with_theory() {
        let n = 1u64 << 16;
        let ratio = evaluate(2, 2, 1.0, n) / (n as f64 * 16.0);
        assert!((0.9..1.2).contains(&ratio));
        let r2 = evaluate(8, 2, 2.0, n) / (n as f64).powi(3);
        assert!((1.0..2.1).contains(&r2));
    }
}
