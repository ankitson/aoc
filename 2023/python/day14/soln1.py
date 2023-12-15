import copy
import itertools
from collections import defaultdict, Counter, deque

def parse(raw_input):
  lines = [line for line in raw_input.split("\n") if len(line) > 2]
  grid =  [list(line) for line in lines]
  return grid

def transpose(grid):
  new_grid = [[0 for i in range(len(grid))] for j in range(len(grid[0]))]
  for row in range(len(grid)):
    for col in range(len(grid[0])):
      new_grid[col][row] = grid[row][col]
  return new_grid

def roll(grid, direction = (-1,0)):
  orig_d = direction
  # original_grid = copy.deepcopy(grid)
  if direction == (0,-1):
    # print("ROLL LEFT. transpose")
    grid = transpose(grid)
    # pgrid(grid)
    orig_d = (0,-1)
    direction = (-1,0)
  elif direction == (0,1):
    orig_d = (0,1)
    grid = transpose(grid)
    direction = (1,0)

  for col_num in range(len(grid[0])):
    rocks = set()
    for row in range(len(grid)):
      if grid[row][col_num] == '#':
        rocks.add(row)
    
    N = len(grid)

    move_up = direction == (-1,0) #else move down

    next_empty = 0
    next_roller = 0
    if move_up:
      next_empty = 0
      next_roller = 0
    else:
      next_empty = N-1
      next_roller = N-1
    
    while True:
      while next_empty < len(grid) and next_empty >=0 and grid[next_empty][col_num] != '.':
        if move_up:
          next_empty += 1
        else:
          next_empty -= 1
      if next_empty == len(grid) or next_empty == -1:
        break
      
      # print(f"next_empty = {next_empty}")
      
      while next_roller < len(grid) and next_roller >= 0 and grid[next_roller][col_num] != 'O':
        if move_up:
          next_roller += 1
        else:
          next_roller -= 1
      if next_roller == len(grid) or next_roller == -1:
        break

      # print(f"next_roll = {next_roller}")
        
      # print(f"next empty = {next_empty} roller = {next_roller}")  
      if (move_up and next_roller > next_empty) or (not move_up and next_roller < next_empty):
        # print(f"match")
        rock_between = False
        for j in range(min(next_empty,next_roller),max(next_empty,next_roller)):
          if j in rocks:
            # print(f"rock at {j}")
            rock_between = True

            # print(f"rock at {j} between")
        if not rock_between:
        # if not all([not (between in rocks) for between in range(next_empty,next_roller)]):
          # print(f"no rocks between {next_empty} and {next_roller}")
          grid[next_roller][col_num] = '.'
          grid[next_empty][col_num] = 'O'
          # pgrid(grid)
        else:
          if move_up:
            next_empty += 1
          else:
            next_empty -= 1
      else:
        if move_up:
          next_roller += 1
        else:
          next_roller -=1
  if orig_d == (0,-1):
    grid = transpose(grid)
  elif orig_d == (0,1):
    grid = transpose(grid)
  # print("AFTER")
  # pgrid(grid)
  return grid

def pgrid(grid):
  print("---------------")
  for row in grid:
    for item in row:
      print(item,end="")
    print()
  
def part1(input):
  grid = parse(input)
  pgrid(grid)
  roll(grid)
  print("---------")
  pgrid(grid)
  height = len(grid)
  total = 0
  for i in range(height):
    anti_height = height - i
    for rock in grid[i]:
      if rock == 'O':
        total += anti_height
  return total


  raise Exception("not implemented!")

def part2(input):
  grid = parse(input)
  grids = [grid]

  # period = 7
  # cycle = 100
  # (100 / 7) = 14
  # 14 * 7 =  98
  i = 0
  period = None
  pgrid(grid)
  ROLLS = 1000000000
  while i < ROLLS:
    # print(f"ROLLING i ={i}")
    grid = roll(grid, (1,0))
    # pgrid(grid)
    grid = roll(grid, (0,-1))
    # pgrid(grid)
    grid = roll(grid, (-1,0))
    # pgrid(grid)
    grid = roll(grid, (0,1))
    # pgrid(grid)
    # grids.append(copy.deepcopy(grid)) #grids 1 = gri dafter first

    if grid == grids[0]:
      period = i + 1
      print(f"found period = {period}")

    # for j in range(i-1): # j in range [0,1) = 0
    #   print(f"Comparing grid after {i}")
    #   pgrid(grid)
    #   print(f"to prev at {j}")
    #   pgrid(grid)
    #   if grid == grids[j]:
    #     period = i - j
    #     print(f"found period = {period} = {i} - {j}")
    i += 1
    if period:
      print(f"period = {period}")
      i = (1000000000 // period) * period
      print(f"fast fowrad to i ={i}")
      
  # pgrid(grid)
  height = len(grid)
  total = 0
  for i in range(height):
    anti_height = height - i
    for rock in grid[i]:
      if rock == 'O':
        total += anti_height
  return total

def main(sample, input1):
  print(f"Hello Day 14!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
