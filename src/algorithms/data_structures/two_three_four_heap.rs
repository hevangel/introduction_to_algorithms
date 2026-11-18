//! @title 2-3-4 Heaps
//! @chapter Problem 20-1 (1e); Problem 19-1 (2e–3e)
//! @editions 1,2,3
//! @summary Store keys only in leaves with subtree minima cached in internal 2-3-4 nodes for O(lg n) minimum, insert, decrease-key, extract-min, delete, and meld.
//! @order 192
use crate::outln;

#[derive(Debug, Clone)]
enum Node {
    Leaf {
        key: i64,
        parent: Option<usize>,
        alive: bool,
    },
    Internal {
        small: i64,
        height: usize,
        parent: Option<usize>,
        children: Vec<usize>,
    },
}

/// Arena-backed 2-3-4 heap supporting O(lg n) minimum, insert, decrease_key, extract_min, delete, and union.
#[derive(Debug, Clone, Default)]
pub struct TwoThreeFourHeap {
    nodes: Vec<Option<Node>>,
    root: Option<usize>,
    len: usize,
}

impl TwoThreeFourHeap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn height_of(&self, id: usize) -> usize {
        match self.nodes[id].as_ref().unwrap() {
            Node::Leaf { .. } => 0,
            Node::Internal { height, .. } => *height,
        }
    }

    fn small_of(&self, id: usize) -> i64 {
        match self.nodes[id].as_ref().unwrap() {
            Node::Leaf { key, .. } => *key,
            Node::Internal { small, .. } => *small,
        }
    }

    fn set_parent(&mut self, id: usize, parent: Option<usize>) {
        match self.nodes[id].as_mut().unwrap() {
            Node::Leaf { parent: p, .. } | Node::Internal { parent: p, .. } => *p = parent,
        }
    }

    fn parent_of(&self, id: usize) -> Option<usize> {
        match self.nodes[id].as_ref().unwrap() {
            Node::Leaf { parent, .. } | Node::Internal { parent, .. } => *parent,
        }
    }

    fn recompute_small(&mut self, id: usize) {
        let new_small = match self.nodes[id].as_ref().unwrap() {
            Node::Leaf { key, .. } => *key,
            Node::Internal { children, .. } => {
                children.iter().map(|&c| self.small_of(c)).min().unwrap()
            }
        };
        if let Node::Internal { small, .. } = self.nodes[id].as_mut().unwrap() {
            *small = new_small;
        }
    }

    fn bubble_small_up(&mut self, mut cur: Option<usize>) {
        while let Some(id) = cur {
            self.recompute_small(id);
            cur = self.parent_of(id);
        }
    }

    fn alloc_internal(&mut self, height: usize, parent: Option<usize>, children: Vec<usize>) -> usize {
        let small = children.iter().map(|&c| self.small_of(c)).min().unwrap();
        let id = self.nodes.len();
        for &c in &children {
            self.set_parent(c, Some(id));
        }
        self.nodes.push(Some(Node::Internal { small, height, parent, children }));
        id
    }

    /// Split a 4-child internal node `id` into `(id_left, id_right)` each with 2 children.
    /// If `id` was root, creates a new root one level higher; otherwise attaches `id_right` to `id`'s parent.
    fn split_four_node(&mut self, id: usize) {
        let (height, parent, right_children) = match self.nodes[id].as_mut().unwrap() {
            Node::Internal { height, parent, children, .. } => {
                assert_eq!(children.len(), 4);
                let right = children.split_off(2);
                (*height, *parent, right)
            }
            Node::Leaf { .. } => unreachable!(),
        };
        self.recompute_small(id);
        let right_id = self.alloc_internal(height, parent, right_children);
        match parent {
            None => {
                let new_root = self.alloc_internal(height + 1, None, vec![id, right_id]);
                self.root = Some(new_root);
            }
            Some(p) => {
                if let Node::Internal { children, .. } = self.nodes[p].as_mut().unwrap() {
                    let idx = children.iter().position(|&c| c == id).unwrap();
                    children.insert(idx + 1, right_id);
                }
                self.recompute_small(p);
            }
        }
    }

    fn ensure_root_not_four(&mut self) {
        if let Some(r) = self.root {
            if let Some(Node::Internal { children, .. }) = self.nodes[r].as_ref() {
                if children.len() == 4 {
                    self.split_four_node(r);
                }
            }
        }
    }

    pub fn minimum(&self) -> Option<i64> {
        self.root.map(|r| self.small_of(r))
    }

    fn insert_leaf_id(&mut self, leaf_id: usize) {
        let Some(r) = self.root else {
            self.set_parent(leaf_id, None);
            self.root = Some(leaf_id);
            return;
        };
        if self.height_of(r) == 0 {
            let new_root = self.alloc_internal(1, None, vec![r, leaf_id]);
            self.root = Some(new_root);
            return;
        }
        self.ensure_root_not_four();
        let mut cur = self.root.unwrap();
        while self.height_of(cur) > 1 {
            // Pick the child with fewest children to keep trees well balanced, splitting if 4-node.
            let next = match self.nodes[cur].as_ref().unwrap() {
                Node::Internal { children, .. } => *children
                    .iter()
                    .min_by_key(|&&c| match self.nodes[c].as_ref().unwrap() {
                        Node::Internal { children, .. } => children.len(),
                        Node::Leaf { .. } => 0,
                    })
                    .unwrap(),
                Node::Leaf { .. } => unreachable!(),
            };
            let next_is_four = matches!(
                self.nodes[next].as_ref().unwrap(),
                Node::Internal { children, .. } if children.len() == 4
            );
            if next_is_four {
                self.split_four_node(next);
            }
            cur = next;
        }
        if let Node::Internal { children, .. } = self.nodes[cur].as_mut().unwrap() {
            children.push(leaf_id);
        }
        self.set_parent(leaf_id, Some(cur));
        self.bubble_small_up(Some(cur));
    }

    /// Insert `key` and return an opaque handle usable for `decrease_key` and `delete`.
    pub fn insert(&mut self, key: i64) -> usize {
        let leaf_id = self.nodes.len();
        self.nodes.push(Some(Node::Leaf { key, parent: None, alive: true }));
        self.len += 1;
        self.insert_leaf_id(leaf_id);
        leaf_id
    }

    /// Decrease the key of `handle` in O(lg n) time.
    pub fn decrease_key(&mut self, handle: usize, new_key: i64) {
        let parent = match self.nodes[handle].as_mut().unwrap() {
            Node::Leaf { key, parent, alive } => {
                assert!(*alive, "handle was already deleted");
                assert!(new_key <= *key, "new_key {new_key} > current key {}", *key);
                *key = new_key;
                *parent
            }
            Node::Internal { .. } => panic!("handle is not a leaf"),
        };
        self.bubble_small_up(parent);
    }

    /// Delete `handle` in O(lg n) time and return its key.
    pub fn delete(&mut self, handle: usize) -> i64 {
        let (key, parent) = match self.nodes[handle].as_mut().unwrap() {
            Node::Leaf { key, parent, alive } => {
                assert!(*alive, "handle already deleted");
                *alive = false;
                (*key, *parent)
            }
            Node::Internal { .. } => panic!("handle is not a leaf"),
        };
        self.len -= 1;
        let Some(mut cur) = parent else {
            self.root = None;
            return key;
        };
        if let Node::Internal { children, .. } = self.nodes[cur].as_mut().unwrap() {
            children.retain(|&c| c != handle);
        }
        loop {
            let deg = match self.nodes[cur].as_ref().unwrap() {
                Node::Internal { children, .. } => children.len(),
                Node::Leaf { .. } => unreachable!(),
            };
            if Some(cur) == self.root {
                if deg == 1 {
                    let only = match self.nodes[cur].as_ref().unwrap() {
                        Node::Internal { children, .. } => children[0],
                        _ => unreachable!(),
                    };
                    self.set_parent(only, None);
                    self.nodes[cur] = None;
                    self.root = Some(only);
                } else {
                    self.recompute_small(cur);
                }
                return key;
            }
            if deg >= 2 {
                self.bubble_small_up(Some(cur));
                return key;
            }
            // Underflow at non-root `cur` (deg == 1): borrow from adjacent sibling or merge.
            let gp = self.parent_of(cur).unwrap();
            let (cur_pos, sib) = match self.nodes[gp].as_ref().unwrap() {
                Node::Internal { children, .. } => {
                    let pos = children.iter().position(|&c| c == cur).unwrap();
                    let sib = if pos > 0 { children[pos - 1] } else { children[1] };
                    (pos, sib)
                }
                _ => unreachable!(),
            };
            let sib_deg = match self.nodes[sib].as_ref().unwrap() {
                Node::Internal { children, .. } => children.len(),
                _ => unreachable!(),
            };
            if sib_deg >= 3 {
                // Borrow one child from sibling `sib`.
                let borrowed = match self.nodes[sib].as_mut().unwrap() {
                    Node::Internal { children, .. } => {
                        if cur_pos > 0 { children.pop().unwrap() } else { children.remove(0) }
                    }
                    _ => unreachable!(),
                };
                self.set_parent(borrowed, Some(cur));
                match self.nodes[cur].as_mut().unwrap() {
                    Node::Internal { children, .. } => {
                        if cur_pos > 0 { children.insert(0, borrowed); } else { children.push(borrowed); }
                    }
                    _ => unreachable!(),
                }
                self.recompute_small(sib);
                self.bubble_small_up(Some(cur));
                return key;
            } else {
                // Merge `cur`'s single child into `sib` and remove `cur` from `gp`.
                let only = match self.nodes[cur].as_ref().unwrap() {
                    Node::Internal { children, .. } => children[0],
                    _ => unreachable!(),
                };
                self.set_parent(only, Some(sib));
                match self.nodes[sib].as_mut().unwrap() {
                    Node::Internal { children, .. } => {
                        if cur_pos > 0 { children.push(only); } else { children.insert(0, only); }
                    }
                    _ => unreachable!(),
                }
                if let Node::Internal { children, .. } = self.nodes[gp].as_mut().unwrap() {
                    children.remove(cur_pos);
                }
                self.nodes[cur] = None;
                self.recompute_small(sib);
                cur = gp;
            }
        }
    }

    pub fn extract_min_with_handle(&mut self) -> Option<(usize, i64)> {
        let mut cur = self.root?;
        loop {
            match self.nodes[cur].as_ref().unwrap() {
                Node::Leaf { .. } => break,
                Node::Internal { small, children, .. } => {
                    let target = *small;
                    cur = *children.iter().find(|&&c| self.small_of(c) == target).unwrap();
                }
            }
        }
        Some((cur, self.delete(cur)))
    }

    pub fn extract_min(&mut self) -> Option<i64> {
        self.extract_min_with_handle().map(|(_, k)| k)
    }

    /// Meld `other` into `self` in O(lg n) time by splicing at matching depth.
    /// Returns a vector mapping `(old_handle_in_other -> new_handle_in_self)`.
    pub fn union(&mut self, mut other: TwoThreeFourHeap) -> Vec<Option<usize>> {
        let mut map = vec![None; other.nodes.len()];
        if other.root.is_none() {
            return map;
        }
        // Remap `other` nodes into `self.nodes`.
        let base = self.nodes.len();
        for (old_id, slot) in other.nodes.iter_mut().enumerate() {
            if let Some(mut node) = slot.take() {
                match &mut node {
                    Node::Leaf { parent, alive, .. } => {
                        if *alive {
                            map[old_id] = Some(base + old_id);
                        }
                        *parent = parent.map(|p| base + p);
                    }
                    Node::Internal { parent, children, .. } => {
                        *parent = parent.map(|p| base + p);
                        for c in children.iter_mut() {
                            *c += base;
                        }
                    }
                }
                self.nodes.push(Some(node));
            } else {
                self.nodes.push(None);
            }
        }
        let other_root = other.root.unwrap() + base;
        self.len += other.len;

        let Some(r1) = self.root else {
            self.root = Some(other_root);
            return map;
        };
        let h1 = self.height_of(r1);
        let h2 = self.height_of(other_root);
        if h2 == 0 {
            self.insert_leaf_id(other_root);
            return map;
        }
        if h1 == 0 {
            self.root = Some(other_root);
            self.insert_leaf_id(r1);
            return map;
        }
        if h1 == h2 {
            let new_root = self.alloc_internal(h1 + 1, None, vec![r1, other_root]);
            self.root = Some(new_root);
            return map;
        }

        // Put the taller tree at `tall_root`, shorter at `short_root` (`short_h < tall_h`).
        let (tall_root, short_root, short_h) = if h1 > h2 {
            (r1, other_root, h2)
        } else {
            self.root = Some(other_root);
            (other_root, r1, h1)
        };
        let _ = tall_root;
        self.ensure_root_not_four();
        let mut cur = self.root.unwrap();
        while self.height_of(cur) > short_h + 1 {
            let next = match self.nodes[cur].as_ref().unwrap() {
                Node::Internal { children, .. } => children[0],
                _ => unreachable!(),
            };
            let next_is_four = matches!(
                self.nodes[next].as_ref().unwrap(),
                Node::Internal { children, .. } if children.len() == 4
            );
            if next_is_four {
                self.split_four_node(next);
            }
            cur = next;
        }
        if let Node::Internal { children, .. } = self.nodes[cur].as_mut().unwrap() {
            children.push(short_root);
        }
        self.set_parent(short_root, Some(cur));
        self.bubble_small_up(Some(cur));
        map
    }

    /// Check all structural invariants (used in tests): degrees in 2..=4, uniform leaf depth, exact `small` values.
    fn check_invariants(&self) {
        let Some(r) = self.root else {
            assert_eq!(self.len, 0);
            return;
        };
        fn dfs(h: &TwoThreeFourHeap, id: usize, expected_h: usize, parent: Option<usize>) -> (i64, usize) {
            match h.nodes[id].as_ref().unwrap() {
                Node::Leaf { key, parent: p, alive } => {
                    assert!(*alive);
                    assert_eq!(*p, parent);
                    assert_eq!(expected_h, 0);
                    (*key, 1)
                }
                Node::Internal { small, height, parent: p, children } => {
                    assert_eq!(*p, parent);
                    assert_eq!(*height, expected_h);
                    assert!((2..=4).contains(&children.len()), "bad deg {}", children.len());
                    let mut min_k = i64::MAX;
                    let mut total = 0;
                    for &c in children {
                        let (ck, cnt) = dfs(h, c, expected_h - 1, Some(id));
                        min_k = min_k.min(ck);
                        total += cnt;
                    }
                    assert_eq!(*small, min_k);
                    (*small, total)
                }
            }
        }
        let root_h = self.height_of(r);
        let (_, count) = dfs(self, r, root_h, None);
        assert_eq!(count, self.len);
    }
}

pub fn demo() -> String {
    let mut o = String::new();
    let mut h1 = TwoThreeFourHeap::new();
    let mut handles = Vec::new();
    for k in [24, 15, 38, 7, 19] {
        handles.push(h1.insert(k));
    }
    outln!(o, "h1 after inserts [24,15,38,7,19]: min = {:?}", h1.minimum());
    h1.decrease_key(handles[2], 3);
    outln!(o, "after decrease_key(38 -> 3)    : min = {:?}", h1.minimum());
    let mut h2 = TwoThreeFourHeap::new();
    for k in [11, 2, 30] {
        h2.insert(k);
    }
    h1.union(h2);
    outln!(o, "after union with [11,2,30]     : len = {}, min = {:?}", h1.len(), h1.minimum());
    let mut popped = Vec::new();
    while let Some(v) = h1.extract_min() {
        popped.push(v);
    }
    outln!(o, "extract_min sequence           : {:?}", popped);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::Rng;
    use std::collections::BTreeMap;

    #[test]
    fn randomized_2000_ops_match_btreemap_oracle() {
        let mut r = Rng::new(1901);
        let mut heap = TwoThreeFourHeap::new();
        let mut oracle: BTreeMap<usize, i64> = BTreeMap::new();
        let mut live: Vec<usize> = Vec::new();

        for step in 0..2000 {
            let choice = r.range(0, 99);
            if choice < 45 || live.is_empty() {
                let k = r.range(0, 10_000) as i64 - 5000;
                let id = heap.insert(k);
                oracle.insert(id, k);
                live.push(id);
            } else if choice < 65 {
                let idx = r.range(0, live.len() - 1);
                let id = live[idx];
                let old = oracle[&id];
                let new_k = old - r.range(0, 200) as i64;
                heap.decrease_key(id, new_k);
                oracle.insert(id, new_k);
            } else if choice < 85 {
                let (ext_id, got) = heap.extract_min_with_handle().unwrap();
                let &min_val = oracle.values().min().unwrap();
                assert_eq!(got, min_val);
                let removed = oracle.remove(&ext_id).unwrap();
                assert_eq!(removed, got);
                let pos = live.iter().position(|&x| x == ext_id).unwrap();
                live.swap_remove(pos);
            } else {
                let idx = r.range(0, live.len() - 1);
                let id = live.swap_remove(idx);
                let got = heap.delete(id);
                let want = oracle.remove(&id).unwrap();
                assert_eq!(got, want);
            }

            let want_min = oracle.values().min().copied();
            assert_eq!(heap.minimum(), want_min);
            if step % 25 == 0 {
                heap.check_invariants();
            }
        }
        heap.check_invariants();
    }

    #[test]
    fn union_at_various_heights_preserves_invariants_and_handles() {
        let mut r = Rng::new(1902);
        for _ in 0..30 {
            let mut h1 = TwoThreeFourHeap::new();
            let mut h2 = TwoThreeFourHeap::new();
            let mut expected = Vec::new();
            let n1 = r.range(0, 40);
            let n2 = r.range(0, 40);
            for _ in 0..n1 {
                let v = r.range(0, 400) as i64 - 200;
                h1.insert(v);
                expected.push(v);
            }
            let mut h2_handles = Vec::new();
            for _ in 0..n2 {
                let v = r.range(0, 400) as i64 - 200;
                h2_handles.push((h2.insert(v), v));
                expected.push(v);
            }
            let map = h1.union(h2);
            h1.check_invariants();
            if let Some((old_h, v)) = h2_handles.first().copied() {
                let new_h = map[old_h].unwrap();
                h1.decrease_key(new_h, v - 100);
                if let Some(pos) = expected.iter().position(|&x| x == v) {
                    expected[pos] = v - 100;
                }
                h1.check_invariants();
            }
            let mut got = Vec::new();
            while let Some(v) = h1.extract_min() {
                got.push(v);
            }
            expected.sort();
            assert_eq!(got, expected);
        }
    }
}
