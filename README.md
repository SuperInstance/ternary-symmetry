# ternary-symmetry

**Group theory and symmetry operations on ternary structures — permutation groups, Cayley tables, Burnside's lemma, and Z₃ group algebra in pure Rust.**

## Background

Symmetry is one of the deepest concepts in mathematics and physics. Emmy Noether proved in 1918 that every continuous symmetry of a physical system corresponds to a conservation law (time symmetry → energy conservation, rotational symmetry → angular momentum conservation). In discrete systems, symmetry manifests through **groups** — algebraic structures that capture the notion of "reversible transformations that compose."

The simplest non-trivial group is **Z₃** (the cyclic group of order 3), which is exactly the group of ternary rotations: {-1, 0, +1} under addition modulo 3. This makes ternary systems fundamentally group-theoretic: every ternary value is an element of Z₃, and every ternary operation is a group action.

This crate provides computational tools for working with symmetry in ternary systems. The key constructions are:

- **Permutation groups**: The building blocks of symmetry. A permutation rearranges elements, and the set of all permutations of n elements forms the **symmetric group Sₙ**. For ternary, S₃ (order 6) is the natural symmetry group — it captures all possible relabelings of {-1, 0, +1}.

- **Cayley tables**: A multiplication table for a group. Entry (i, j) gives the result of composing group elements i and j. The Cayley table of Z₃ is a 3×3 Latin square — every row and column is a permutation of {0, 1, 2}.

- **Burnside's lemma**: Counts the number of distinct patterns under a group action. If G acts on n positions with k colors, the number of distinct colorings is (1/|G|) Σ_{g∈G} k^{c(g)}, where c(g) is the number of cycles in permutation g. This is essential for ternary systems because it tells us how many truly different ternary patterns exist, accounting for symmetries.

## How It Works

**`Permutation`** — A bijection on {0, 1, ..., n-1}:
- **`compose`**: Function composition (apply self, then other).
- **`inverse`**: The inverse permutation (undoes the original).
- **`order`**: Smallest k such that p^k = identity. A 3-cycle has order 3; a transposition has order 2.
- **`sign`**: +1 for even permutations (product of an even number of transpositions), -1 for odd. This is the determinant of the corresponding permutation matrix.
- **`apply`**: Apply the permutation to a slice of items.

**`FiniteGroup`** — A group represented by its Cayley table:
- **`z3()`**: Builds Z₃ with table `[[0,1,2],[1,2,0],[2,0,1]]`. Element 0 is the identity; 1 and 2 are generators.
- **`z3xz3()`**: Builds Z₃ × Z₃ (direct product, order 9) — the natural symmetry group of a 2D ternary lattice with independent rotational symmetry on each axis.
- **`multiply`, `identity`, `inverse`**: Standard group operations via table lookup.
- **`is_abelian`**: Checks commutativity by verifying a[i][j] == a[j][i] for all pairs.
- **`order_histogram`**: Counts how many elements have each order. For Z₃: 1 element of order 1 (identity) and 2 elements of order 3.

**`burnside_count(group, n_colors)`** — Applies Burnside's lemma:
For each permutation in the group, counts the number of colorings fixed by that permutation (all elements in each cycle must have the same color → k^{#cycles}). Averages over the group size.

**`symmetric_group_3()`** — Returns all 6 elements of S₃: {identity, (12), (13), (23), (123), (132)}.
**`cyclic_group_3()`** — Returns the 3 rotations of C₃: {identity, (123), (132)}.

### Design Decisions

- **Explicit Cayley tables over implicit generators**: Rather than storing generators and computing products on-the-fly, we store the full multiplication table. For small groups (order ≤ 9), this is efficient and simple.
- **Permutation representation as Vec<usize>**: Each permutation maps position i to mapping[i]. This is clear and debuggable, though not the most memory-efficient for large n.
- **`#![no_std]` compatible**: The entire group theory library runs without a standard library, suitable for deployment in constrained environments.

## Experimental Results

All 18 tests pass. Specific findings:

- **Permutation identity**: The 4-element identity permutation is its own inverse and has order 1.
- **Composition**: Permutation [1,0,2] (swap positions 0,1) composed with [0,2,1] (swap positions 1,2) gives [2,0,1] — the composite of two transpositions is a 3-cycle.
- **Inverse correctness**: A 3-cycle [1,2,0] composed with its inverse gives the identity — confirmed by test.
- **Permutation order**: The 3-cycle [1,2,0] has order 3 (three applications return to identity).
- **Sign computation**: The identity has sign +1 (even). The transposition [1,0,2] has sign -1 (odd). The 3-cycle [1,2,0] has sign +1 (even — a 3-cycle is a product of two transpositions).
- **Z₃ group**: Order 3, abelian. Multiplication: 1·2 = 0 (mod 3), 1·1 = 2. Inverse of 1 is 2. Order histogram: {(1,1), (3,2)} — one element of order 1, two of order 3.
- **Z₃ × Z₃**: Order 9, abelian (direct product of abelian groups is abelian).
- **Burnside for S₃ with 3 colors**: 10 distinct colorings. By Burnside's lemma: (27 + 3·9 + 2·3) / 6 = (27 + 27 + 6) / 6 = 60/6 = 10. The identity fixes 27 colorings (3³); each of 3 transpositions fixes 9 (the swapped pair must match); each of 2 three-cycles fixes 3 (all three must match).
- **Burnside for C₃ with 3 colors**: 11 distinct colorings. (27 + 3 + 3) / 3 = 33/3 = 11. The cyclic group is less restrictive than S₃, so more patterns survive.

## Impact

Group theory on ternary structures matters because:

1. **Z₃ is the natural symmetry group of ternary data**: Every ternary operation is a group action. Understanding the group structure tells us which operations are "the same" up to relabeling — essential for avoiding redundant computation.

2. **Burnside's lemma counts distinct ternary circuits**: When designing ternary logic circuits, many circuits are equivalent under input permutation. Burnside tells you exactly how many truly different circuits exist.

3. **Symmetry-aware optimization**: If a ternary computation has S₃ symmetry, you can compute the answer for one input permutation and derive the others for free — a 6× speedup.

## Use Cases

1. **Ternary circuit design**: Count the number of distinct ternary logic gates (functions from {-1,0,+1}ⁿ → {-1,0,+1}) using Burnside's lemma. For n=2 inputs with S₃ acting on the output values, this tells you how many functionally different gates exist.

2. **Symmetry-aware neural network architecture**: If a ternary layer's weight matrix has symmetry group G, you only need to store one representative from each orbit under G. Burnside's lemma tells you how much compression this achieves.

3. **Molecular symmetry analysis**: Model molecular configurations with ternary labels (positive/negative/neutral charge sites). Use Burnside's lemma to count distinct molecular configurations under rotational symmetry.

4. **Code optimization via equivalence classes**: In a ternary compiler, identify instructions that are equivalent up to permutation of inputs. Group theory provides the formal framework for this optimization.

5. **Game theory with ternary strategies**: Analyze symmetric games where players have three strategies {-1, 0, +1}. The symmetric group S₃ acts on strategy labels; Burnside's lemma counts the distinct game structures.

## Open Questions

1. **Beyond S₃ and Z₃**: What about symmetry groups of larger ternary structures (e.g., Z₃ⁿ for n-dimensional ternary lattices)? The current implementation handles Z₃ and Z₃ × Z₃, but larger groups require sparse representations.

2. **Continuous symmetry approximation**: Can ternary groups approximate continuous symmetry groups (SO(3), SU(2)) in the limit of large ternary grids? This would connect discrete ternary computing to gauge theory.

3. **Galois theory connections**: Z₃ is the finite field F₃. Can we leverage Galois theory to understand the structure of ternary polynomials and their roots?

## Connection to Oxide Stack

`ternary-symmetry` provides foundational abstractions for the entire stack. At the **open-parallel** level, symmetry groups define how to distribute work across equivalent data partitions. The **pincher** layer uses Burnside's lemma to avoid computing redundant configurations. **Flux-core** uses Z₃ as the algebraic backbone of all ternary operations. **cuda-oxide** exploits symmetry for memory layout optimization (storing one representative per orbit). **cudaclaw** exposes symmetry analysis in its API, allowing users to query "how many distinct patterns exist for this configuration?"
