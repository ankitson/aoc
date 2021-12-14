Naive (f8abcb8)
------

```
day14/total/5           time:   [142.13 us 142.60 us 143.42 us]                         
day14/total/6           time:   [345.76 us 347.66 us 352.11 us]                         
day14/total/7           time:   [924.57 us 926.80 us 930.17 us]                          
day14/total/8           time:   [2.8036 ms 2.8127 ms 2.8275 ms]                          
day14/total/9           time:   [9.3489 ms 9.3599 ms 9.3789 ms]                         
day14/total/10          time:   [34.089 ms 34.302 ms 34.503 ms]                          
day14/total/11          time:   [129.18 ms 129.58 ms 130.22 ms]                         
day14/total/12          time:   [505.42 ms 507.37 ms 509.25 ms] 
```

exponential

Let S = length(poly)
    R = length(rules)
    N = num iters

for each pair of characters - O(S) pairs
  for each rule - O(R) rules
    get pair - O(1) assuming the iterator has a fast skip
    add char - O(1) amortized append to array
apply = O(S*R) time

assuming string length grows exponentially,

apply_nth = O(2^n * S * R) time





