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

----
# Sample Input
```
FNFPPNKPPHSOKFFHOFOC

VS -> B
SV -> C
PP -> N
NS -> N
BC -> N
PB -> F
BK -> P
NV -> V
KF -> C
KS -> C
PV -> N
NF -> S
PK -> F
SC -> F
KN -> K
PN -> K
OH -> F
PS -> P
FN -> O
```
-----

Idea 1: 

`FN -> FON -> FAOBN -> ...`

Each pair has a fixed expansion after `n` steps.
memoize computed expansions?

```
dp[c1][c2][n] = str

dp[F][N][1] = FON
df[F][N][2] = FAOBN
..
```

for 40 steps, we have 26*26*40 elems.
max length of step 1 elems = 3
max length of step 2 elems = (3-1)*3 - 1 = 5

for a string of length i, there are (i-1) new chars inserted max

L(k+1) = L(k) + L(k) -1

L(k+1) = 2L(k) - 1

L(1) = 2*2 - 1 = 3
L(2) = 2*3 - 1 = 5
L(3) = 2*5 - 1 = 9

L(n+1) = 2(2*(L(n-1)-1)) - 1 = 2*2*L(n-1) - 2*2 - 1
L(n+1) = 2^kL(n-k) - (1 + 2^k) 



