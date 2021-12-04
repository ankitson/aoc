# Solutions to Advent of Code 2021

My goal here is to use AoC to learn how to write idiomatic Rust

## Notes

* 

There is no implicit mapping between files and modules - this is built explicitly using `mod`.

* 

`Iterator`s built using `iter` return borrowed refs to the value in the container
 
`IntoIterator`s built using `into_iter` (by convention) move the value

https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html

