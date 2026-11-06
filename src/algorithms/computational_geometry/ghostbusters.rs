//! @title Ghostbusters and Ghosts
//! @chapter Problem 33-4 (1e–4e)
//! @editions 1,2,3,4
//! @summary Match n ghostbusters to n ghosts by non-crossing segments (no three points collinear): split by the lowest point's angular sweep, O(n² lg n) worst case.
//! @order 62
use super::segment_intersection::{cross, Pt};
use crate::outln;
use std::cmp::Ordering;

/// A point with a flag: true = ghostbuster, false = ghost.
pub type Person = (Pt, bool);

/// Perfect matching between busters and ghosts (index pairs) whose segments do not cross. Requires equal counts and no three collinear points.
pub fn match_no_crossing(people: &[Person]) -> Vec<(usize, usize)> {
    let mut out = vec![];
    solve(people, (0..people.len()).collect(), &mut out);
    out
}

fn solve(people: &[Person], idx: Vec<usize>, out: &mut Vec<(usize, usize)>) {
    if idx.is_empty() { return; }
    // p = lowest point (leftmost on ties); all others are in the upper half-plane around it
    let &p = idx.iter().min_by_key(|&&i| (people[i].0 .1, people[i].0 .0)).unwrap();
    let mut rest: Vec<usize> = idx.into_iter().filter(|&i| i != p).collect();
    let o = people[p].0;
    rest.sort_by(|&a, &b| { let c = cross(o, people[a].0, people[b].0); if c > 0 { Ordering::Less } else if c < 0 { Ordering::Greater } else { Ordering::Equal } });
    // scan counter-clockwise; match p with the first opposite-type point that has a balanced set before it
    let mut balance = 0i32; // (# same type as p) − (# opposite type) among points seen so far
    for (k, &q) in rest.iter().enumerate() {
        let same = people[q].1 == people[p].1;
        if !same && balance == 0 {
            out.push((p, q));
            solve(people, rest[..k].to_vec(), out);
            solve(people, rest[k + 1..].to_vec(), out);
            return;
        }
        balance += if same { 1 } else { -1 };
    }
    unreachable!("a balanced split always exists when #busters = #ghosts");
}

/// Do two segments (a,b) and (c,d) cross properly? (Valid when no three of the points are collinear.)
pub fn crosses(a: Pt, b: Pt, c: Pt, d: Pt) -> bool {
    (cross(a, b, c) > 0) != (cross(a, b, d) > 0) && (cross(c, d, a) > 0) != (cross(c, d, b) > 0)
}

pub fn demo() -> String {
    let mut o = String::new();
    let people: Vec<Person> = vec![((0, 0), true), ((10, 1), true), ((4, 9), true), ((9, 8), false), ((2, 5), false), ((6, 3), false)];
    let m = match_no_crossing(&people);
    for &(a, b) in &m { outln!(o, "{:?} ({}) — {:?} ({})", people[a].0, if people[a].1 { "buster" } else { "ghost" }, people[b].0, if people[b].1 { "buster" } else { "ghost" }); }
    let crossing = (0..m.len()).any(|i| (i + 1..m.len()).any(|j| crosses(people[m[i].0].0, people[m[i].1].0, people[m[j].0].0, people[m[j].1].0)));
    outln!(o, "any two segments cross? {crossing}");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    fn no_three_collinear(p: &[Pt]) -> bool {
        (0..p.len()).all(|i| (i + 1..p.len()).all(|j| (j + 1..p.len()).all(|k| cross(p[i], p[j], p[k]) != 0)))
    }
    #[test]
    fn matchings_are_perfect_bichromatic_and_planar() {
        let mut r = Rng::new(408);
        let mut checked = 0;
        while checked < 200 {
            let n = r.range(1, 6);
            let pts: Vec<Pt> = (0..2 * n).map(|_| (r.range(0, 40) as i64, r.range(0, 40) as i64)).collect();
            let mut sorted = pts.clone();
            sorted.sort();
            sorted.dedup();
            if sorted.len() < 2 * n || !no_three_collinear(&pts) { continue; }
            let people: Vec<Person> = pts.iter().enumerate().map(|(i, &p)| (p, i < n)).collect();
            let m = match_no_crossing(&people);
            assert_eq!(m.len(), n);
            let mut used: Vec<usize> = m.iter().flat_map(|&(a, b)| [a, b]).collect();
            used.sort();
            assert_eq!(used, (0..2 * n).collect::<Vec<_>>());
            assert!(m.iter().all(|&(a, b)| people[a].1 != people[b].1));
            for i in 0..m.len() { for j in i + 1..m.len() { assert!(!crosses(pts[m[i].0], pts[m[i].1], pts[m[j].0], pts[m[j].1])); } }
            checked += 1;
        }
    }
    #[test]
    fn crossing_predicate() {
        assert!(crosses((0, 0), (4, 4), (0, 4), (4, 0)));
        assert!(!crosses((0, 0), (1, 1), (3, 0), (4, 5)));
    }
}
