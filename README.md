# ternary-symmetry

**Group theory in three symbols. Permutations, Cayley tables, and Burnside's lemma over Z₃.**

Symmetry is the mathematics of "what stays the same when everything else changes." A group captures the operations that leave a structure invariant: rotations of a triangle, permutations of elements, additions modulo 3. This crate implements finite group theory for ternary systems — permutation groups, Cayley tables (the multiplication tables of groups), and Burnside's lemma for counting distinct structures under symmetry.

## What's Inside

- **`Permutation`** — permutation with compose, inverse, order, sign (parity)
- **`FiniteGroup`** — group via Cayley table (Z₃, Z₃×Z₃ built-in)
- **`burnside_count()`** — count distinct colorings under a group action
- **`symmetric_group_3()`** — all 6 permutations of S₃
- **`cyclic_group_3()`** — the 3 rotations of C₃

## Quick Example

```rust
use ternary_symmetry::*;

// How many distinct ternary colorings of 3 positions, up to rotation?
let group = cyclic_group_3();
let count = burnside_count(&group, 3);
assert_eq!(count, 11); // 11 distinct ternary colorings up to C₃ rotation

// Z₃ group
let g = FiniteGroup::z3();
assert!(g.is_abelian());
assert_eq!(g.multiply(1, 2), 0); // 1 + 2 ≡ 0 mod 3
```

## The Deeper Truth

**Burnside's lemma is the bridge between symmetry and counting.** It says: the number of distinct objects (modulo symmetry) equals the average number of objects fixed by each symmetry operation. For ternary systems, this gives exact counts of distinct ternary structures. With 3 colors and C₃ symmetry on 3 positions, there are 11 distinct colorings — not 27/3 = 9 as naive counting would suggest, because the identity fixes all 27, while rotations fix only 3 each. (27 + 3 + 3) / 3 = 11.

## See Also

- **ternary-crystal** — crystallographic symmetry groups
- **ternary-ring** — Z₃ as an algebraic ring
- **ternary-permutation** — deeper permutation algorithms
- **ternary-topology** — topological symmetry and invariants

## Install

```bash
cargo add ternary-symmetry
```

## License

MIT
