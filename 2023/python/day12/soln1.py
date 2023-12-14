import copy

def parse(input):
  lines = input.split("\n")
  grid = []
  conds = []
  for line in lines:
    if len(line) < 2:
      continue
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
  curr_run = row[0]
  run_len = 1 if row[0] != '.' else 0
  run_start = 0 if row[0] != '.' else None
  for i in range(1,len(row)):
    char = row[i]
    if char == '.':
      pass
    elif char == curr_run:
      run_len += 1
    else:
      if run_len > 0:
        runs.append((run_start,run_len))
      run_start = i
      run_len = 1
    curr_run = char
  if run_len > 0:
    runs.append((run_start,run_len))
  return runs

seen = {}

def part1(raw_input):
  global seen
  (grid, conds) = parse(raw_input)
  # print(f"Conds = {conds}")
  total = 0
  for i in range(len(grid)):
    row = grid[i]
    N = len(row)
    runs = genruns(row)
    # print(f"Runs for row {''.join(row)} , cond {conds[i]} = {runs}")
    seen = {}
    soln = solve(row, runs, conds[i], [], set())
    print(f"{soln}")
    total += soln
  return total


def solve(row, runs, conds, trace, taken):
  # print(f"SOLVE {row} {runs} {conds} {taken}")
  trace.append(f"solve {''.join(row)} {runs} {conds}")
  # print(f"solve {runs} {conds}")
  hashes = [i for i in range(len(row)) if row[i] == '#']
  if len(conds) == 0:
    taken_str = ''.join([str(x) for x in sorted(taken)])
    if taken_str in seen: #we have already covered this path
      trace.append(f"already covered path {taken_str}")
      # print(trace)
      return 0
    #all hashes must be used
    row_print = copy.deepcopy(row)
    for i in taken:
      row_print[i] = "#"
    rowstr = ''.join(row_print)
    # trace = []
    trace.append(f"row: {rowstr}")

    # taken_all = []
    # for (i,c) in taken:
    #   for j in range(i,i+c):
    #     taken_all.append(j)
    unused_hashes = [i for i in hashes if not i in taken]
    if len(unused_hashes) > 0:
      # trace = []
      trace.append(f"unused {unused_hashes}")
      # print(trace)
      return 0
    # trace = []
    trace.append(f"taken: {taken}")
    trace.append(f"returning 1")
    # print(trace)
    seen[taken_str] = 1
    return 1
  if len(runs) == 0 and len(conds) > 0:
    taken_str = ''.join([str(x) for x in taken])
    row_print = copy.deepcopy(row)
    for i in taken:
      row_print[i] = "#"
    rowstr = ''.join(row_print)

    # trace = []
    trace.append(f"row: {rowstr}")
    trace.append(f"taken: {taken}")
    trace.append(f"returning 0")
    # print(trace)
    return 0
  
  (run_start, run_len) = runs[0]

  if len(runs) == 1 and run_start == 9:
    # print("consider run start 9")
    trace.append(f"consider run start 9: {runs[0]}")
  
  nways_without = 0
  if not all([ch == '#' for ch in row[run_start:run_start+run_len]]):
    # we can skip ? runs but not "#" runs
    nways_without = solve(row, runs[1:], conds, trace + ["skip run"], taken)

  if len(runs) == 1 and run_start ==9:
    # print("consider 9 without skip")
    
    trace.append(f"consider run start 9 after skip: {runs[0]}")

  # or use it - we can use part of it and split, or all of it
  cond = conds[0]
  nways_with = 0

  for start_idx in range(0,run_len-cond+1):  #[0,3-1+1-1] = [0,2]
    abs_start = run_start + start_idx
    if abs_start < 0  or (abs_start-1) in taken: #row[abs_start-1] == '#': #can't split with a "#" before
      continue
    (remain_start, remain_len) = (run_start + start_idx + cond + 1,run_len-start_idx- cond - 1) # we have to skip 1 for spacing
    next_runs = runs[1:]
    if remain_len > 0:
      if not remain_start -1 in taken:
        next_runs.insert(0, (remain_start, remain_len))
    next_taken = copy.deepcopy(taken)
    for j in range(abs_start, abs_start+cond):
      next_taken.add(j)
    nways_with += solve(row, next_runs, conds[1:], trace + [f"take run idx {start_idx + run_start}", f"taken: {next_taken}"], next_taken)
  
  #or we can merge this run and the next run if they are adjacent
  nways_merge = 0
  if len(runs) > 1:
    (next_run_start, nrl) = runs[1]
    if run_start + run_len == next_run_start:
      next_runs = runs[1:]
      next_runs[0] =  (run_start, run_len + nrl)
      nways_merge += solve(row, next_runs, conds, trace + [f"merge runs"], taken)
  
  return nways_without + nways_with + nways_merge

def part2(raw_input):
  global seen
  (grid, conds) = parse(raw_input)
  # print(f"Conds = {conds}")
  total = 0
  for i in range(len(grid)):
    print(f"orig row: {grid[i]}")
    new_row = []
    for j in range(5):
      new_row.extend(grid[i])
      if j != 4:
        new_row.append("?")
    row = new_row
    # row = grid[i] * 5
    N = len(row)
    runs = genruns(row)
    cond = conds[i] * 5#[x for cs in conds[i] * 5 for x in cs]
    print(f"row: {row} conds: {cond}")
    # print(f"Runs for row {''.join(row)} , cond {conds[i]} = {runs}")
    seen = {}
    soln = solve(row, runs, cond, [], set())
    print(f"{soln}")
    total += soln
  return total

def main(sample, input1):
  print(f"Hello Day 12!")

  # soln = part1(sample)
  # print(f"Part 1 (sample) = {soln}")

  # soln = part1(input1)
  # print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  # soln = part2(input1)
  # print(f"Part 2 (realinput) = {soln}")
