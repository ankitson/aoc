# Advent of Code 2023

## Table of Contents

1. [Day 1](#day-1-trebuchet?!)
2. [Day 2](#day-2-tbd)

# Day 2: TBD

# Day 1: Trebuchet?!

## Part 1

Part 1 was simple enough.

## Part 2

On part 2, i spent a little time fumbling with `matches` and `match_indices`. I assumed these would match regexes so I had something like `str.matches("one|two|three...")` but they don't. So i replaced that with a pile of 9 `matches`, one for each number. But it turns out we need the indices, so i found the `match_indices` function to do that.

I spent a fair bit of time debugging too - i was a bit confused on the logic for bounds checking the lengths and then one logic bug - i did not sort the combined matches array by index before looking at the first/last indices so it was picking random matches.

### Soln 2

I made a faster solution for part 2 using regexes. This one runs in 361.05µs compared to around double the time for the earlier solution. Initially I had this by finding all the matches using a regex and then looking at the first and the last. This does NOT work because rusts `regex` will not find overlapping matches. So i had to write it to find the first match in the beginning of the string and the first match in the reverse.
