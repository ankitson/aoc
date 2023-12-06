#Time:      7  15   30
#Distance:  9  40  200
import math
def parse(raw_input):
  lines = raw_input.split("\n")
  times = [int(x) for x in lines[0].split(" ")[1:] if len(x) > 0]
  dists = [int(x) for x in lines[1].split(" ")[1:] if len(x) > 0] 
  return (times,dists)

def part1(input):
  (ts,ds) = parse(input)
  ans = 1
  for i in range(len(ts)):
    t = ts[i]
    d = ds[i]
    num_bet = 0
    for hold in range(t):
      dist_covered = (t - hold) * hold
      if dist_covered > d:
        num_bet += 1
    ans *= num_bet
  return ans

def part2(input):
  (ts,ds) = parse(input)
  T = int(''.join([str(x) for x in ts]))
  D = int(''.join([str(x) for x in ds]))

  # y = (T-x)*x > D between its two roots. find the roots of:
  # -x^2 + Tx - D = 0
  root1 = -T + math.sqrt(T*T - 4*(-D)*(-1)) / 2*(-1)
  root2 = -T - math.sqrt(T*T - 4*(-D)*(-1)) / 2*(-1)
  return abs(int(root1 - root2))

def main(sample, input1):
  print(f"Hello Day 06!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")
  assert(soln == 288)

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")
  assert(soln == 1731600)

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")
  assert(soln == 71503)

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
  assert(soln == 40087680)
