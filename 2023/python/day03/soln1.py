from collections import defaultdict
def parse(raw_input):
  return raw_input

def part1(input):
  grid = [list(x) for x in input.split("\n") if len(x) > 1]
  n = len(grid)
  m = len(grid[0])
  specials = []
  for i in range(n):
    for j in range(m):
      if grid[i][j] != '.' and not grid[i][j].isdigit():
        specials.append((i,j))
  
  nums = []
  seen = set()
  for (sx,sy) in specials:
    nbrs = [(sx-1,sy),(sx+1,sy),(sx,sy-1),(sx,sy+1),(sx-1,sy-1),(sx+1,sy+1), (sx-1,sy+1), (sx+1,sy-1)]
    valid = [nbr for nbr in nbrs if nbr[0] >= 0 and nbr[0] <n and nbr[1] >= 0 and nbr[1] < m]
    is_num = [nbr for nbr in valid if grid[nbr[0]][nbr[1]].isdigit()]

    rows = defaultdict(list)
    for (nx,ny) in is_num:
      rows[nx].append(ny)
    
    for (rn,cols) in rows.items():
      sorted_cols = sorted(cols)
      for j in sorted_cols:
        if (rn,j) in seen:
          continue
        before = max(j-1,0)
        before_num = []
        while before >=0 and grid[rn][before].isdigit() and not (rn,before) in seen:
          before_num.append(grid[rn][before])
          seen.add((rn,before))
          before -= 1
        before_num.reverse()
        
        after = min(j+1,m-1)
        after_num = []
        while after <m and grid[rn][after].isdigit() and not (rn,after) in seen:
          after_num.append(grid[rn][after])
          seen.add((rn,after))
          after += 1
        
        full_num = before_num + [grid[rn][j]] + after_num
        if len(full_num) > 0:
          nums.append(int(''.join(full_num)))
  return sum(nums)

def part2(input):
  grid = [list(x) for x in input.split("\n") if len(x) > 1]
  n = len(grid)
  m = len(grid[0])
  specials = []
  for i in range(n):
    for j in range(m):
      if grid[i][j] != '.' and not grid[i][j].isdigit():
        specials.append((i,j))
  
  gear_ratio = 0
  seen = set()
  for (sx,sy) in specials:
    nbrs = [(sx-1,sy),(sx+1,sy),(sx,sy-1),(sx,sy+1),(sx-1,sy-1),(sx+1,sy+1), (sx-1,sy+1), (sx+1,sy-1)]
    valid = [nbr for nbr in nbrs if nbr[0] >= 0 and nbr[0] <n and nbr[1] >= 0 and nbr[1] < m]
    is_num = [nbr for nbr in valid if grid[nbr[0]][nbr[1]].isdigit()]

    nums = []
    rows = defaultdict(list)
    for (nx,ny) in is_num:
      rows[nx].append(ny)
    
    for (rn,cols) in rows.items():
      sorted_cols = sorted(cols)
      for j in sorted_cols:
        if (rn,j) in seen:
          continue
        before = max(j-1,0)
        before_num = []
        while before >=0 and grid[rn][before].isdigit() and not (rn,before) in seen:
          before_num.append(grid[rn][before])
          seen.add((rn,before))
          before -= 1
        before_num.reverse()
        
        after = min(j+1,m-1)
        after_num = []
        while after <m and grid[rn][after].isdigit() and not (rn,after) in seen:
          after_num.append(grid[rn][after])
          seen.add((rn,after))
          after += 1
        
        full_num = before_num + [grid[rn][j]] + after_num
        if len(full_num) > 0:
          nums.append(int(''.join(full_num)))
    if grid[sx][sy] == '*' and len(nums) == 2:
      gear_ratio += nums[0] * nums[1]
  return gear_ratio

def main(sample, input1):
  print(f"Hello Day 03!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

