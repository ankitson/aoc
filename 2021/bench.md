
Generated via `cargo bench -q --bench corebench -- --noplot --output-format bencher --color never --sample-size 10`

# 2021

## Main
```
test day1.part1.realinput/day1/9618 ...                       bench:              31,544 ns/iter (+/- 296)
test day1.part2.realinput/day1/9618 ...                       bench:              31,368 ns/iter (+/- 136)
test day2.part1.realinput/day2/7875 ...                       bench:              30,697 ns/iter (+/- 112)
test day2.part2.realinput/day2/7875 ...                       bench:              30,966 ns/iter (+/- 267)
test day3.part1.realinput/day3/13000 ...                      bench:              27,561 ns/iter (+/- 71)
test day3.part2.realinput/day3/13000 ...                      bench:              35,490 ns/iter (+/- 127)
test day4.part1.realinput/day4/7890 ...                       bench:              68,936 ns/iter (+/- 104)
test day4.part2.realinput/day4/7890 ...                       bench:             343,728 ns/iter (+/- 2452)
test day5.part1.realinput/part1/1000 ...                      bench:             566,755 ns/iter (+/- 273034)
test day5.part1.realinput/part1/2000 ...                      bench:           1,764,697 ns/iter (+/- 159177)
test day5.part1.realinput/part1/4000 ...                      bench:           4,471,871 ns/iter (+/- 4236509)
test day5.part2.realinput/part2/1000 ...                      bench:             987,906 ns/iter (+/- 144625)
test day5.part2.realinput/part2/2000 ...                      bench:           1,681,507 ns/iter (+/- 210324)
test day5.part2.realinput/part2/4000 ...                      bench:           4,322,551 ns/iter (+/- 94192)
test day6.part1.realinput/part1/64 ...                        bench:             899,418 ns/iter (+/- 4300)
test day6.part1.realinput/part1/128 ...                       bench:          50,730,075 ns/iter (+/- 109610)
test day6.part2.realinput/part2/64 ...                        bench:               2,601 ns/iter (+/- 3)
test day6.part2.realinput/part2/128 ...                       bench:               3,009 ns/iter (+/- 6)
test day6.part2.realinput/part2/256 ...                       bench:               3,869 ns/iter (+/- 11)
test day7.part1.realinput/part1 ...                           bench:             253,324 ns/iter (+/- 842)
test day7.part1.fast.realinput/part1_fast ...                 bench:              58,743 ns/iter (+/- 259)
test day7.part2.realinput/part2 ...                           bench:             748,505 ns/iter (+/- 417)
test day7.part2.fast.realinput/part2_fast ...                 bench:              18,418 ns/iter (+/- 126)
test day8.part1.realinput/part1 ...                           bench:              99,282 ns/iter (+/- 443)
test day8.part2.realinput/part2 ...                           bench:         254,175,252 ns/iter (+/- 10904510)
test day9.part1.realinput/part1 ...                           bench:            ,338,660 ns/iter (+/- 259)
test day9.part2.realinput/part2 ...                           bench:            ,811,852 ns/iter (+/- 27437)
test day9.part2.mut.realinput/part2_mut ...                   bench:            ,903,901 ns/iter (+/- 14006)
test day10.part1.realinput/part1 ...                          bench:              52,116 ns/iter (+/- 693)
test day10.part2.realinput/part2 ...                          bench:              53,530 ns/iter (+/- 640)
test day11.part1.realinput/part1 ...                          bench:            ,307,138 ns/iter (+/- 6510)
test day11.part2.realinput/part2 ...                          bench:            ,730,912 ns/iter (+/- 1399)
test day12.part1.realinput/part1 ...                          bench:           1,465,554 ns/iter (+/- 78255)
test day12.part2.realinput/part2 ...                          bench:          57,983,720 ns/iter (+/- 1655310)
test day13.part1.staticgrid.realinput/part1 ...               bench:           1,216,395 ns/iter (+/- 1101592)
test day13.part2.staticgrid.realinput/part2 ...               bench:           2,695,726 ns/iter (+/- 654803)
test day14.part1.naive.realinput/part1 ...                    bench:           3,945,229 ns/iter (+/- 96952)
test day14.part1.recursive_memo.realinput/part1 ...           bench:            ,882,509 ns/iter (+/- 2524)
test day14.part1.mapcount.realinput/part1 ...                 bench:              92,585 ns/iter (+/- 235)
test day14.part2.mapcount.realinput/part2 ...                 bench:            ,387,112 ns/iter (+/- 681)
test day15.part1.realinput/part1 ...                          bench:           1,107,229 ns/iter (+/- 1473)
test day15.part2.realinput/part2 ...                          bench:          33,960,206 ns/iter (+/- 28809)
test day16.part1.realinput/part1 ...                          bench:              65,971 ns/iter (+/- 151)
test day16.part2.realinput/part2 ...                          bench:              67,509 ns/iter (+/- 184)
test day17.part1.realinput/part1 ...                          bench:         114,434,422 ns/iter (+/- 188458)
test day17.part2.realinput/part2 ...                          bench:      11,026,866,587 ns/iter (+/- 12683967)
test day18.part1.realinput/part1 (copy) ...                   bench:           2,757,639 ns/iter (+/- 11480)
test day18.part2.realinput/part2 (copy) ...                   bench:          50,614,155 ns/iter (+/- 1105099)
test day18.part1.realinput/part1 (mutable) ...                bench:             451,844 ns/iter (+/- 14153)
test day18.part2.realinput/part2 (mutable) ...                bench:           9,001,033 ns/iter (+/- 523578)
test day23.part1.realinput/part1 ...                          bench:         218,200,387 ns/iter (+/- 285848)
test day23.part2.realinput/part2 ...                          bench:         169,785,296 ns/iter (+/- 414510)
```

## Others
```
test day14.naive.input_scaling/total/5 ...                    bench:      139560 ns/iter (+/- 2302)
test day14.naive.input_scaling/total/6 ...                    bench:      361089 ns/iter (+/- 3779)
test day14.naive.input_scaling/total/7 ...                    bench:      918855 ns/iter (+/- 12057)
test day14.naive.input_scaling/total/8 ...                    bench:     2854848 ns/iter (+/- 49828)
test day14.naive.input_scaling/total/9 ...                    bench:     9445885 ns/iter (+/- 25640)
test day14.naive.input_scaling/total/10 ...                   bench:    34430973 ns/iter (+/- 1064898)
test day14.naive.input_scaling/total/11 ...                   bench:   129806496 ns/iter (+/- 2552538)
test day14.naive.input_scaling/total/12 ...                   bench:   503650264 ns/iter (+/- 13696374)

test day14.recursive_memo.input_scaling/total/5 ...           bench:       31678 ns/iter (+/- 280)
test day14.recursive_memo.input_scaling/total/6 ...           bench:       59476 ns/iter (+/- 1147)
test day14.recursive_memo.input_scaling/total/7 ...           bench:      114415 ns/iter (+/- 846)
test day14.recursive_memo.input_scaling/total/8 ...           bench:      225641 ns/iter (+/- 818)
test day14.recursive_memo.input_scaling/total/9 ...           bench:      435426 ns/iter (+/- 143)
test day14.recursive_memo.input_scaling/total/10 ...          bench:      870542 ns/iter (+/- 1469)
test day14.recursive_memo.input_scaling/total/11 ...          bench:     1732304 ns/iter (+/- 2198)
test day14.recursive_memo.input_scaling/total/12 ...          bench:     3542464 ns/iter (+/- 17549)

test day14.mapcount.input_scaling/total/5 ...                 bench:       44567 ns/iter (+/- 265)
test day14.mapcount.input_scaling/total/6 ...                 bench:       53279 ns/iter (+/- 229)
test day14.mapcount.input_scaling/total/7 ...                 bench:       63446 ns/iter (+/- 679)
test day14.mapcount.input_scaling/total/8 ...                 bench:       73355 ns/iter (+/- 1084)
test day14.mapcount.input_scaling/total/9 ...                 bench:       84388 ns/iter (+/- 417)
test day14.mapcount.input_scaling/total/10 ...                bench:       94796 ns/iter (+/- 156)
test day14.mapcount.input_scaling/total/11 ...                bench:      104722 ns/iter (+/- 294)
test day14.mapcount.input_scaling/total/12 ...                bench:      114704 ns/iter (+/- 76)
test day14.mapcount.input_scaling/total/20 ...                bench:      192705 ns/iter (+/- 6652)
test day14.mapcount.input_scaling/total/40 ...                bench:      389371 ns/iter (+/- 737)
```
