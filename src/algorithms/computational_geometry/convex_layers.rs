//! @title Convex Layers
//! @chapter Problem 33-1 (1e–4e)
//! @editions 1,2,3,4
//! @summary Peel off the convex hull again and again ("onion peeling"): the layers of a point set, computed in O(n²) simply or O(n lg n) with more care.
//! @order 60
use super::convex_hull_graham::{graham_scan, hull_area2};
use super::segment_intersection::Pt;
use crate::outln;

/// The convex layers of a point set: layer 0 is the convex hull, layer 1 the hull of the remaining points, and so on.
/// Each layer lists its vertices counter-clockwise. O(n² lg n) with Graham's scan; points on hull edges stay for the next layer.
pub fn convex_layers(points: &[Pt]) -> Vec<Vec<Pt>> {
    let mut rest: Vec<Pt> = points.to_vec();
    rest.sort();
    rest.dedup();
    let mut layers = vec![];
    while !rest.is_empty() {
        let mut layer = graham_scan(&rest);
        if layer.is_empty() { layer = rest.clone(); }
        rest.retain(|p| !layer.contains(p));
        layers.push(layer);
    }
    layers
}

/// Depth of each point (index of its layer), in input order.
pub fn depths(points: &[Pt]) -> Vec<usize> {
    let layers = convex_layers(points);
    points.iter().map(|p| layers.iter().position(|l| l.contains(p)).unwrap()).collect()
}

pub fn demo() -> String {
    let mut o = String::new();
    let pts: Vec<Pt> = vec![(0, 0), (10, 0), (10, 10), (0, 10), (2, 2), (8, 2), (8, 8), (2, 8), (5, 5), (4, 6), (6, 4)];
    for (i, l) in convex_layers(&pts).iter().enumerate() { outln!(o, "layer {i}: {:?} (2×area = {})", l, hull_area2(l)); }
    outln!(o, "depth of each input point: {:?}", depths(&pts));
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::segment_intersection::cross;
    use crate::common::Rng;
    fn random_points(r: &mut Rng, n: usize) -> Vec<Pt> {
        let mut v: Vec<Pt> = (0..n).map(|_| (r.range(0, 30) as i64, r.range(0, 30) as i64)).collect();
        v.sort();
        v.dedup();
        v
    }
    #[test]
    fn layers_partition_the_points_and_nest() {
        let mut r = Rng::new(405);
        for _ in 0..100 {
            let n = r.range(1, 40); let pts = random_points(&mut r, n);
            let layers = convex_layers(&pts);
            let mut all: Vec<Pt> = layers.iter().flatten().copied().collect();
            all.sort();
            assert_eq!(all, pts);
            for w in layers.windows(2) {
                if w[0].len() >= 3 { // every deeper point lies inside or on the previous hull
                    for &p in &w[1] { for i in 0..w[0].len() { assert!(cross(w[0][i], w[0][(i + 1) % w[0].len()], p) >= 0); } }
                }
                if w[0].len() >= 3 && w[1].len() >= 3 { assert!(hull_area2(&w[1]) < hull_area2(&w[0])); }
            }
        }
    }
    #[test]
    fn first_layer_is_the_set_of_extreme_points() {
        let mut r = Rng::new(406);
        for _ in 0..100 {
            let n = r.range(3, 14); let pts = random_points(&mut r, n);
            let full = hull_area2(&graham_scan(&pts));
            let extreme: Vec<Pt> = pts.iter().copied().filter(|&p| { let rest: Vec<Pt> = pts.iter().copied().filter(|&q| q != p).collect(); hull_area2(&graham_scan(&rest)) < full }).collect();
            let mut layer0 = convex_layers(&pts).remove(0);
            layer0.sort();
            // when the points are not all collinear the area criterion finds exactly the hull vertices
            if full > 0 { assert_eq!(layer0, extreme); }
        }
    }
    #[test]
    fn nested_squares() {
        let pts: Vec<Pt> = vec![(0, 0), (9, 0), (9, 9), (0, 9), (2, 2), (7, 2), (7, 7), (2, 7), (4, 4), (5, 4), (5, 5), (4, 5)];
        assert_eq!(convex_layers(&pts).len(), 3);
    }
}
