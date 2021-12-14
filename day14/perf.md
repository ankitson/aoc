# Naive (f8abcb8)
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

----

# V2 

Idea: 

`FN -> FON -> FAOBN -> ...`

Each pair has a fixed expansion after `n` steps.
memoize computed expansions?

```
dp[c1][c2][n] = str

dp[F][N][1] = FON
df[F][N][2] = FAOBN
..
```

```
day14-naive/total/5     time:   [142.13 us 142.46 us 142.83 us]                               
day14-naive/total/6     time:   [343.90 us 344.25 us 344.52 us]                               
day14-naive/total/7     time:   [924.13 us 924.20 us 924.29 us]                                
day14-naive/total/8     time:   [2.7890 ms 2.7903 ms 2.7916 ms]                                
day14-naive/total/9     time:   [9.3626 ms 9.3674 ms 9.3736 ms]                               
day14-naive/total/10    time:   [33.767 ms 33.884 ms 34.018 ms]                                
day14-naive/total/11    time:   [127.68 ms 128.02 ms 128.28 ms]                               
day14-naive/total/12    time:   [496.40 ms 497.15 ms 498.38 ms]                               
day14-v2/total/5        time:   [31.993 us 32.043 us 32.089 us]                             
day14-v2/total/6        time:   [59.627 us 59.652 us 59.708 us]                            
day14-v2/total/7        time:   [115.20 us 115.24 us 115.27 us]                            
day14-v2/total/8        time:   [223.85 us 223.91 us 223.99 us]                            
day14-v2/total/9        time:   [440.94 us 441.26 us 441.54 us]                            
day14-v2/total/10       time:   [878.10 us 878.34 us 878.70 us]                              
day14-v2/total/11       time:   [1.7533 ms 1.7538 ms 1.7546 ms]                              
day14-v2/total/12       time:   [3.5018 ms 3.5039 ms 3.5067 ms]   
```

This is 50-150x better. 

Extrapolated runtime at size 40 = 3.5ms * 2^28 = 10 days!

# V3

the good algo.. only keep track of counts of pairs of chars - this is enough to apply rules and recover character counts from.

```
day14-fast/total/5      time:   [43.428 us 43.446 us 43.477 us]                               
day14-fast/total/10     time:   [92.610 us 92.656 us 92.734 us]                               
day14-fast/total/20     time:   [191.12 us 191.20 us 191.37 us]                               
day14-fast/total/40     time:   [385.70 us 385.83 us 385.98 us]                               
```

