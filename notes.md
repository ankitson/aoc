##  Notes
---

There is no implicit mapping between files and modules - this is built explicitly using `mod`.

---

`Iterator`s built using `iter` return borrowed refs to the value in the container
 
`IntoIterator`s built using `into_iter` (by convention) move the value

---

`sort_by_key` moves the key, so is unusable when copy is expensive. use `sort_by` instead which allows borrows

---

Names like `col_idx` are confusing because it could mean either the "index of the column" or the "index inside the column" which are opposite dimensions.

## Tools & Tricks
---

- use `{:#?}` to pretty print Debugs

- [cargo expand](https://github.com/dtolnay/cargo-expand) can be used to view code after macro expansion

- [cargo watch](https://lib.rs/crates/cargo-watch). `cargo watch` (same as `cargo watch -x check` to check without full compilation (faster), or `cargo watch -x build` to do a full compile.

## Puzzles


___
```rust
        fn tunnels(&self) -> Vec<usize> { ... }
        ...
        self.tunnels()
            .into_iter()
            .map(|col_idx| {
                if let Some(bad_row) = (0..N).rev().filter(|ri| !self.correct_pos(col_idx, *ri)).next() {
                    let x = sides(col_idx, 0)
                        .into_iter()
                        .map(|dest_col| (dest_col, 0usize, bad_row + col_idx - dest_col));
                    x
                } else {
                    iter::empty::<(usize, usize, usize)>().map(|x| x).into_iter()
                }
            })
            .flatten()
```

fails with:
```rust
error[E0308]: `if` and `else` have incompatible types
   --> day23/src/soln1.rs:142:21
    |
136 | /                 if let Some(bad_row) = (0..N).rev().filter(|ri| !self.correct_pos(col_idx, *ri)).next() {
137 | |                     let x = sides(col_idx, 0)
138 | |                         .into_iter()
139 | |                         .map(|dest_col| (dest_col, 0usize, bad_row + col_idx - dest_col));
140 | |                     x
    | |                     - expected because of this
141 | |                 } else {
142 | |                     iter::empty::<(usize, usize, usize)>().map(|x| x).into_iter()
    | |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::vec::IntoIter`, found struct `std::iter::Empty`
143 | |                 }
    | |_________________- `if` and `else` have incompatible types
```

----

```rust
pub fn parse(input: &str) -> ([&[u8]; 4]) {
    let mut lines = input.lines().skip(2);

    let row1 = lines.next().unwrap();
    let row2 = lines.next().unwrap();
    let mut cols: [[u8; 2]; 4] = [[0u8; 2]; 4];
    let mut col = [0; 2];

    let mut col_idx = 0;
    for i in 0..row1.len() {
        let chtop = row1.chars().nth(i).unwrap();
        let chbot = row2.chars().nth(i).unwrap_or(' ');
        if (chtop == 'A' || chtop == 'B' || chtop == 'C' || chtop == 'D') {
            col[0] = ((chtop as u8) - ('A' as u8));
            col[1] = ((chbot as u8) - ('A' as u8));
            cols[col_idx] = col;
            col_idx += 1;
        }
    }
    cols
}
``` 

does not compile:

```rust
3  | pub fn parse(input: &str) -> ([&[u8]; 4]) {
   |                               ---------- expected `[&[u8]; 4]` because of return type
...
22 |     cols
   |     ^^^^ expected `&[u8]`, found array `[u8; 2]`
   |
   = note: expected array `[&[u8]; 4]`
              found array `[[u8; 2]; 4]`
```              

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

----


```rust
type BS = BitSlice<Msb0, u8>;
pub fn parse_packet(input: &BS) -> Option<(&BS, u64)> {
pub fn parse_literal(input: &BS) -> Option<(&BS, BitVec)> {
...

let literal1 = "D2FE28";

//doesnt compile
let (rem, parsed) = parse_packet(&parse_bv(literal1)).unwrap();
println!("rem: {:?}", &rem); 

//compiles
let bv = parse_bv(literal1);
let (rem, parsed) = parse_packet(&bv).unwrap();
println!("rem: {:?}", &rem); 
```


----

```rust
pub fn split(fishnum: &mut FN) -> Option<&FN> ...

pub fn explode(fishnum: &mut FN) -> Option<&FN> ...

pub fn reduce(fishnum: &mut FN) -> &FN {
    if let Some(next) = Self::explode(fishnum) {
        next
    } else if let Some(next) = Self::split(fishnum) {
        next
    } else {
        fishnum
    }
}


error[E0499]: cannot borrow `*fishnum` as mutable more than once at a time
  --> day18/src/soln1.rs:54:48
   |
51 |     pub fn reduce(fishnum: &mut FN) -> &FN {
   |                            - let's call the lifetime of this reference `'1`
52 |         if let Some(next) = Self::explode(fishnum) {
   |                                           ------- first mutable borrow occurs here
53 |             next
   |             ---- returning this value requires that `*fishnum` is borrowed for `'1`
54 |         } else if let Some(next) = Self::split(fishnum) {
   |                                                ^^^^^^^ second mutable borrow occurs here

error[E0502]: cannot borrow `*fishnum` as immutable because it is also borrowed as mutable
  --> day18/src/soln1.rs:57:13
   |
51 |     pub fn reduce(fishnum: &mut FN) -> &FN {
   |                            - let's call the lifetime of this reference `'1`
52 |         if let Some(next) = Self::explode(fishnum) {
   |                                           ------- mutable borrow occurs here
53 |             next
   |             ---- returning this value requires that `*fishnum` is borrowed for `'1`
...
57 |             fishnum
   |             ^^^^^^^ immutable borrow occurs here
```
