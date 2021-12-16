##  Notes
---

There is no implicit mapping between files and modules - this is built explicitly using `mod`.

---

`Iterator`s built using `iter` return borrowed refs to the value in the container
 
`IntoIterator`s built using `into_iter` (by convention) move the value

## Tools & Tricks
---

- [cargo expand](https://github.com/dtolnay/cargo-expand) can be used to view code after macro expansion

## Puzzles

----
```rust
let contents: &str = include_str!("../inputs/day2.txt");
c.bench_with_input(BenchmarkId::new("day2", contents.len()), contents, |b, c| {
    b.iter(|| Soln1::part1_core(Soln1::parse(contents)));
});
```

does not compile -> the "contents" param in second should be "&contents". its a simple bug. 

but it leads to something interesting anyways. what is `Sized` and why does `bench_with_input` require the type to be `Sized`?

the defintion of the fn is here:

```rust
// Benchmark the given parameterized function inside this benchmark group.
pub fn bench_with_input<ID: IntoBenchmarkId, F, I>(
    &mut self,
    id: ID,
    input: &I,
    f: F,
) -> &mut Self
    where
    F: FnMut(&mut Bencher<'_, M>, &I),
    I: ?Sized,
{
```

Note the `?Sized` constraint on `I`. `Sized` means the size of the type must be known at compile time. Type params are sized by default, but this constraint can be relaxed using the `?Sized` constraint. 

Also see [rustlangref](https://doc.rust-lang.org/reference/special-types-and-traits.html?highlight=Sized#sized), [rustonomicon](https://doc.rust-lang.org/nomicon/exotic-sizes.html?highlight=Sized#zero-sized-types-zsts)

-----

from the [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait docs:   

> It’s important to note that in these two examples, the only difference is whether you are allowed to access x after the assignment. Under the hood, both a copy and a move can result in bits being copied in memory, although this is sometimes optimized away.

in what cases would a move result in a memory copy?

-------

```rust
error[E0308]: mismatched types
  --> day12/src/soln1.rs:36:51
   |
36 |             npaths += Self::npaths_from(*end_adj, &adj_list);
   |                                                   ^^^^^^^^^ expected struct `shared::AdjList`, found struct `soln1::shared::AdjList`
   |
   = note: expected reference `&shared::AdjList`
              found reference `&soln1::shared::AdjList`
```

----

```rust
let f3: &[char; 3] = &input[..3];
```

does not typecheck. it is typed as a `&[char]` but doesnt have the length.

this is a workaround that compiles. here it will check the length at runtime:

```rust
let f3: &[char; 3] = &input[..3].try_into().unwrap();
```