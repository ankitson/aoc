#Time:      7  15   30
#Distance:  9  40  200
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
    

  #hold for 0 - speed = 0
  #hold for 1 - speed = 1
  #hold for 2 - speed = 2. distance = (total - hold)*hold
  print(t)
  print(d)

  raise Exception("not implemented!")

def part2(input):
  (ts,ds) = parse(input)
  time = int(''.join([str(x) for x in ts]))
  dist = int(''.join([str(x) for x in ds]))
  print(time)
  print(dist)

  # y = (T-x)*x
  # y > D
  # y -D > 0
  # (T-x)*x - D > 0
  # Tx - x^2 - D> 0
  # -x^2 + Tx - D >  0
  # x(-x+T) - D = 0
  # x = -
  import math

  T = time
  D = dist

  root1 = -T + math.sqrt(T*T - 4*(-D)*(-1)) / 2*(-1)
  root2 = -T - math.sqrt(T*T - 4*(-D)*(-1)) / 2*(-1)
  print(root1)
  print(root2)

  return abs(int(root1 - root2))
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
  raise Exception("not implemented!")

def main(sample, input1):
  print(f"Hello Day 06!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

