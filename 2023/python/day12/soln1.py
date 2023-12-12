import copy

def parse(input):
  lines = input.split("\n")
  grid = []
  conds = []
  for line in lines:
    parts = line.strip().split(" ")
    row = list(parts[0])
    cond = [int(x) for x in parts[1].split(",")]
    grid.append(row)
    conds.append(cond)
  return (grid,conds)

def around(row, i):
  broken_before = 0
  j = i - 1
  while j >= 0 and row[j] == '#':
    j -= 1
    broken_before += 1
  
  broken_after = 0
  j = i + 1
  while j < len(row) and (row[j] == '#' or row[j] == '?'):
    j += 1
    broken_after += 1
  return (broken_before,broken_after)

def genruns(row):
  runs = []
  start = 0
  rl = 0
  for i in range(len(row)):
    c = row[i]
    if c == '?' or c == '#':
      rl += 1
    else:
      if rl > 0:
        runs.append((start,rl))
      start = i+1
      rl = 0
  if rl > 0:
    runs.append((start,rl))
  return runs

def part1(raw_input):
  (grid, conds) = parse(raw_input)
  # print(f"Conds = {conds}")
  for i in range(len(grid)):
    row = grid[i]
    N = len(row)
    runs = genruns(row)
    print(f"Runs for row {row} , cond {conds[i]} = {runs}")
    soln = solve(runs, conds[i])
    print(soln)

def solve(runs, conds):
  print(f"solve {runs} {conds}")
  if len(conds) == 0:
    print(f"\tret 1")
    return 1
  if len(runs) == 0 and len(conds) > 0:
    print(f"\tret 0")
    return 0
  cond = conds[0]
  run = runs[0]
  (nways, remainders) = solve_recurse(run, cond)
  print(f"\tnways = {nways} remainders = {remainders}")
  # if nways == 0: #the first run doesnt fit the first cond. but we could not use that run perchance
    # return solve(runs[1:],conds)
  
  # if len(remainders) == 0 and nways > 0:
    # return nways

  total = 0
  total += solve(runs[1:], conds)
  for remainder in remainders: #len=nways
    rem_runs = [remainder] + runs[1:]
    nways_tail = solve(rem_runs, conds[1:])
    total += nways_tail
  # rem_runs = remainders + runs[1:]
  # nways_tail = solve(rem_runs, conds[1:])
  return total
  # return nways * nways_tail
  # return ways

#run = (1,2)
def solve_recurse(run,cond):
  print(f"solve rec: {run} {cond}")
  C = cond
  remaining = []
  (run_start,run_len) = run
  nways = 0
  for start_pt in range(0,run_len-C+1): #C = 3 , rl = 3, (0,1) #[0,1]
    nways += 1 #Y
    remainder_run = (run_start+start_pt+C+1,run_len-C-1) #put a "." to seperate #1+0+1+1,2-1 = 3,1
    if remainder_run[0] < run_start + run_len: #3 < 1+2 NO
      remaining.append(remainder_run)
  return (nways, remaining)

  #try to use all of run for cond, or it could be split up into pieces..



    # cond = conds[i]
    # dp = [0] * N
    # for j in range(N-1,-1,-1):

    # nworked = recurse(row, conds[i])
    # print(f"{nworked} ways for row {i}")

# def recurse(row, conds):
#     print(f"recurse {row} {conds}")
#     if len(conds) == 0:
#       print(f"\tworks")
#       return 1
#     cond = conds[0]
#     # print(f"cond: {cond}")

#     row_spots = [i for i in range(len(row)) if row[i] == '?']
#     nbroken = 0
#     total = 0
#     for i in range(len(row)):
#       if row[i] == '#':
#         nbroken += 1
#       elif row[i] == '.':
#         nbroken = 0
#       elif row[i] == '?':
#         new_row = copy.deepcopy(row)
#         for j in range(i,i+cond-nbroken):
#           new_row[j] = '#'
#         if new_row[i+cond-nbroken] == '#':
#           continue #does not work
#         else:
#           new_row[i+cond-nbroken] = '.'
#           works = recurse(new_row[i+cond-nbroken:],conds[1:])
#           total += works
#           nbroken = 0
#     if nbroken > 0:
#       if nbroken == cond:
#         total += 1
#     return total

  # print(f"Parsed = {parsed}")


def part2(input):
  raise Exception("not implemented!")

def main(sample, input1):
  print(f"Hello Day 12!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  # soln = part1(input1)
  # print(f"Part 1 (realinput) = {soln}")

  # soln = part2(sample)
  # print(f"Part 2 (sample) = {soln}")

  # soln = part2(input1)
  # print(f"Part 2 (realinput) = {soln}")

