//! # ternary-symmetry
//! 
//! Group theory and symmetry operations in ternary space.
//! Permutation groups, Cayley tables, Burnside's lemma, and group actions.

#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;
use alloc::{vec, vec::Vec};

/// A permutation of n elements, represented as a mapping i -> perm[i]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Permutation {
    pub mapping: Vec<usize>,
}

impl Permutation {
    pub fn identity(n: usize) -> Self {
        Self { mapping: (0..n).collect() }
    }

    pub fn new(mapping: Vec<usize>) -> Self {
        Self { mapping }
    }

    pub fn compose(&self, other: &Self) -> Self {
        let mapping = self.mapping.iter()
            .map(|&i| other.mapping[i])
            .collect();
        Self { mapping }
    }

    pub fn inverse(&self) -> Self {
        let n = self.mapping.len();
        let mut inv = vec![0; n];
        for (i, &j) in self.mapping.iter().enumerate() {
            inv[j] = i;
        }
        Self { mapping: inv }
    }

    pub fn apply<T: Clone>(&self, items: &[T]) -> Vec<T> {
        self.mapping.iter().map(|&i| items[i].clone()).collect()
    }

    pub fn order(&self) -> usize {
        let mut current = self.clone();
        let mut count = 1;
        let identity = Permutation::identity(self.mapping.len());
        while current != identity {
            current = current.compose(self);
            count += 1;
        }
        count
    }

    pub fn is_identity(&self) -> bool {
        self.mapping.iter().enumerate().all(|(i, &j)| i == j)
    }

    pub fn sign(&self) -> i8 {
        let n = self.mapping.len();
        let mut visited = vec![false; n];
        let mut cycles = 0usize;
        for i in 0..n {
            if !visited[i] {
                cycles += 1;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = self.mapping[j];
                }
            }
        }
        if (n - cycles) % 2 == 0 { 1 } else { -1 }
    }
}

/// A finite group represented by its Cayley table
#[derive(Debug, Clone)]
pub struct FiniteGroup {
    pub cayley_table: Vec<Vec<usize>>,
    pub order: usize,
}

impl FiniteGroup {
    /// Build Z₃ (the cyclic group of order 3)
    pub fn z3() -> Self {
        Self {
            cayley_table: vec![
                vec![0, 1, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
            ],
            order: 3,
        }
    }

    /// Build Z₃ × Z₃ (direct product)
    pub fn z3xz3() -> Self {
        let n = 9;
        let mut table = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                let a1 = i % 3;
                let a2 = i / 3;
                let b1 = j % 3;
                let b2 = j / 3;
                table[i][j] = (a1 + b1) % 3 + ((a2 + b2) % 3) * 3;
            }
        }
        Self { cayley_table: table, order: n }
    }

    pub fn multiply(&self, a: usize, b: usize) -> usize {
        self.cayley_table[a][b]
    }

    pub fn identity(&self) -> usize {
        // Find element e such that e*x = x for all x
        for e in 0..self.order {
            let mut is_identity = true;
            for x in 0..self.order {
                if self.cayley_table[e][x] != x {
                    is_identity = false;
                    break;
                }
            }
            if is_identity { return e; }
        }
        0
    }

    pub fn inverse(&self, a: usize) -> usize {
        let e = self.identity();
        for b in 0..self.order {
            if self.cayley_table[a][b] == e {
                return b;
            }
        }
        e
    }

    /// Check if the group is abelian (commutative)
    pub fn is_abelian(&self) -> bool {
        for i in 0..self.order {
            for j in (i + 1)..self.order {
                if self.cayley_table[i][j] != self.cayley_table[j][i] {
                    return false;
                }
            }
        }
        true
    }

    /// Count elements of each order
    pub fn order_histogram(&self) -> Vec<(usize, usize)> {
        let mut counts = vec![0; self.order + 1];
        for a in 0..self.order {
            let mut current = a;
            let mut ord = 1;
            let e = self.identity();
            while current != e {
                current = self.cayley_table[current][a];
                ord += 1;
                if ord > self.order { break; }
            }
            counts[ord] += 1;
        }
        counts.iter().enumerate()
            .filter(|(_, &c)| c > 0)
            .map(|(ord, &c)| (ord, c))
            .collect()
    }
}

/// Burnside's lemma: count distinct colorings under a group action
/// group: the permutation group acting on positions
/// n_colors: number of available colors (3 for ternary)
pub fn burnside_count(group: &[Permutation], n_colors: usize) -> usize {
    let n = group[0].mapping.len();
    let mut total_fixed = 0usize;
    for perm in group {
        let mut fixed = 1usize;
        let mut visited = vec![false; n];
        for i in 0..n {
            if !visited[i] {
                let mut cycle_len = 0;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    j = perm.mapping[j];
                    cycle_len += 1;
                }
                // A coloring is fixed iff all elements in a cycle have the same color
                // So each cycle contributes n_colors choices
                fixed *= n_colors;
                let _ = cycle_len;
            }
        }
        total_fixed += fixed;
    }
    total_fixed / group.len()
}

/// Generate the symmetric group S₃ (all permutations of 3 elements)
pub fn symmetric_group_3() -> Vec<Permutation> {
    vec![
        Permutation::new(vec![0, 1, 2]),
        Permutation::new(vec![0, 2, 1]),
        Permutation::new(vec![1, 0, 2]),
        Permutation::new(vec![1, 2, 0]),
        Permutation::new(vec![2, 0, 1]),
        Permutation::new(vec![2, 1, 0]),
    ]
}

/// Generate the cyclic group C₃ (rotations of 3 elements)
pub fn cyclic_group_3() -> Vec<Permutation> {
    vec![
        Permutation::new(vec![0, 1, 2]),
        Permutation::new(vec![1, 2, 0]),
        Permutation::new(vec![2, 0, 1]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let p = Permutation::identity(4);
        assert!(p.is_identity());
        assert_eq!(p.apply(&[10, 20, 30, 40]), vec![10, 20, 30, 40]);
    }

    #[test]
    fn test_compose() {
        let p1 = Permutation::new(vec![1, 0, 2]);
        let p2 = Permutation::new(vec![0, 2, 1]);
        let composed = p1.compose(&p2);
        assert_eq!(composed.mapping, vec![2, 0, 1]);
    }

    #[test]
    fn test_inverse() {
        let p = Permutation::new(vec![1, 2, 0]);
        let inv = p.inverse();
        let identity = p.compose(&inv);
        assert!(identity.is_identity());
    }

    #[test]
    fn test_order() {
        let p = Permutation::new(vec![1, 2, 0]); // 3-cycle
        assert_eq!(p.order(), 3);
    }

    #[test]
    fn test_sign_even() {
        let p = Permutation::identity(3);
        assert_eq!(p.sign(), 1);
    }

    #[test]
    fn test_sign_odd() {
        let p = Permutation::new(vec![1, 0, 2]); // transposition
        assert_eq!(p.sign(), -1);
    }

    #[test]
    fn test_sign_3cycle() {
        let p = Permutation::new(vec![1, 2, 0]);
        assert_eq!(p.sign(), 1); // 3-cycle is even
    }

    #[test]
    fn test_z3_group() {
        let g = FiniteGroup::z3();
        assert_eq!(g.order, 3);
        assert!(g.is_abelian());
        assert_eq!(g.identity(), 0);
    }

    #[test]
    fn test_z3_multiply() {
        let g = FiniteGroup::z3();
        assert_eq!(g.multiply(1, 2), 0); // 1+2 = 3 ≡ 0
        assert_eq!(g.multiply(1, 1), 2); // 1+1 = 2
    }

    #[test]
    fn test_z3_inverse() {
        let g = FiniteGroup::z3();
        assert_eq!(g.inverse(1), 2); // 1+2 = 0
        assert_eq!(g.inverse(0), 0);
    }

    #[test]
    fn test_z3xz3() {
        let g = FiniteGroup::z3xz3();
        assert_eq!(g.order, 9);
        assert!(g.is_abelian());
    }

    #[test]
    fn test_z3_order_histogram() {
        let g = FiniteGroup::z3();
        let hist = g.order_histogram();
        assert!(hist.contains(&(1, 1))); // identity has order 1
        assert!(hist.contains(&(3, 2))); // two elements of order 3
    }

    #[test]
    fn test_burnside_s3_3colors() {
        let group = symmetric_group_3();
        let count = burnside_count(&group, 3);
        // S₃ acting on 3 positions with 3 colors
        // Burnside: (27 + 3*3 + 2*3) / 6 = (27+9+6)/6 = 42/6 = 7
        // Wait, let me compute: 
        // identity fixes 3³ = 27
        // 3 transpositions fix 3² = 9 each (the swapped pair must be same color)
        // 2 3-cycles fix 3¹ = 3 each (all three must be same color)
        // (27 + 3*9 + 2*3) / 6 = (27+27+6)/6 = 60/6 = 10
        assert_eq!(count, 10);
    }

    #[test]
    fn test_burnside_c3_3colors() {
        let group = cyclic_group_3();
        let count = burnside_count(&group, 3);
        // C₃: identity fixes 27, two rotations fix 3 each
        // (27 + 3 + 3) / 3 = 11
        assert_eq!(count, 11);
    }

    #[test]
    fn test_symmetric_group_3() {
        let s3 = symmetric_group_3();
        assert_eq!(s3.len(), 6);
    }

    #[test]
    fn test_cyclic_group_3() {
        let c3 = cyclic_group_3();
        assert_eq!(c3.len(), 3);
    }
}
