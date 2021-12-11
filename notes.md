#  Notes

There is no implicit mapping between files and modules - this is built explicitly using `mod`.

---

`Iterator`s built using `iter` return borrowed refs to the value in the container
 
`IntoIterator`s built using `into_iter` (by convention) move the value

# Tools, Tips, Tricks

- [cargo expand](https://github.com/dtolnay/cargo-expand) can be used to view code after macro expansion

- An `Option<i32>` is 8 bytes, but an `i32` is 4 bytes.

```rust
println!("{} {}", std::mem::size_of::<Option<i32>>(), std::mem::size_of::<i32>());
>8 4
```

However the `NonZeroX` types use the null to represent `None` when boxed into an `Option`.

```
println!("{}", std::mem::size_of::<Option<core::num::NonZeroU32>>())
>4
```

There is also this hack:

```
#![feature(rustc_attrs)]

#[rustc_layout_scalar_valid_range_end(2147483648)]
struct NonNegativeI32(i32);

dbg!(std::mem::size_of::<Option<NonNegativeI32>>());
dbg!(std::mem::size_of::<Option<Option<NonNegativeI32>>>());
```

[although it shouldn't be used unless you know what you're doing?](https://users.rust-lang.org/t/option-usize-in-8-bytes-via-custom-none-value/46299/7)


---


# Puzzles

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

# Fighting the borrow checker

```rust
pub fn part1(input: &str) -> i32 {
    let lines = shared::parse(input);
    let mut total_score = 0;
    for line in lines {
        let line_score = match Self::first_illegal(line) {
            Either::Left(bad_index) => {
                let bad_char =
                    &line.chars().nth(bad_index).expect("illegal char");
                let score = Self::score_illegal(bad_char);
                score
            }
            _ => &0,
        };
        total_score += line_score;
    }
    total_score
}
```

doesnt compile:

```rust
rror[E0716]: temporary value dropped while borrowed
  --> day10/src/soln1.rs:77:26
   |
74 |             let line_score = match Self::first_illegal(line) {
   |                 ---------- borrow later stored here
...
77 |                         &line.chars().nth(bad_index).expect("illegal char");
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
...
80 |                 }
   |                 - temporary value is freed at the end of this statement
   |
   = note: consider using a `let` binding to create a longer lived value

For more information about this error, try `rustc --explain E0716`.
error: could not compile `day10` due to previous error```