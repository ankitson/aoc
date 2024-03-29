def parse(raw_input):
  grids = raw_input.split("\n\n")
  ret = []
  for grid in grids:
    lines = grid.split("\n")
    gridp = [list(line) for line in lines if len(line) > 2]
    ret.append(gridp)
  return ret

def solve(grid):
  width = len(grid[0])
  candidates = set(list(range(1,width)))
  for row in grid:
    for ci in range(1,len(row)): #mirror is before position i
      fwd,bwd = ci,ci-1
      while fwd < width and bwd >= 0:
        if row[fwd] != row[bwd]:
          if ci in candidates:
            candidates.remove(ci)
          break
        fwd += 1
        bwd -= 1
  return candidates

def part1(input):
  grids = parse(input)
  total = 0
  for grid in grids:
    
    reflect_vertical = solve(grid)
    transpose = [[0 for i in range(len(grid))] for j in range(len(grid[0]))]
    for i in range(len(grid)):
      for j in range(len(grid[0])):
        transpose[j][i] = grid[i][j]
    reflect_horiz = solve(transpose)

    reflect_vertical = list(reflect_vertical)[0] if reflect_vertical and len(reflect_vertical) > 0 else None
    reflect_horiz = list(reflect_horiz)[0] if reflect_horiz and len(reflect_horiz) > 0 else None

    if reflect_vertical is not None:
      total += reflect_vertical
    elif reflect_horiz is not None:
      total += (100*reflect_horiz)
    else:
      print("UH OH")

  return total

def part2(input):
  grids = parse(input)
  total = 0
  for grid in grids:
    orig_v = solve(grid)
    transpose = [[0 for i in range(len(grid))] for j in range(len(grid[0]))]
    for i in range(len(grid)):
      for j in range(len(grid[0])):
        transpose[j][i] = grid[i][j]
    orig_r = solve(transpose)

    done = False
    found_h = orig_r
    found_v = orig_v
    for smudge_row in range(len(grid)):
      if done:
        break
      for smudge_col in range(len(grid[0])):
        if done:
          break
        
        grid[smudge_row][smudge_col] =  "#" if grid[smudge_row][smudge_col] == '.' else '.'

        reflect_vertical = solve(grid) - found_v
        reflect_vertical = list(reflect_vertical)[0] if len(reflect_vertical) > 0 else None

        transpose = [[0 for i in range(len(grid))] for j in range(len(grid[0]))]
        for i in range(len(grid)):
          for j in range(len(grid[0])):
            transpose[j][i] = grid[i][j]

        reflect_horiz = solve(transpose) - found_h
        reflect_horiz = list(reflect_horiz)[0] if reflect_horiz and len(reflect_horiz) > 0 else None

        grid[smudge_row][smudge_col] = "#" if grid[smudge_row][smudge_col] == '.' else '.'
        
        done = True
        if reflect_vertical is not None:
          total += reflect_vertical
        elif reflect_horiz is not None:
          total += (100*reflect_horiz)
        else:
          done = False
  return total

def main(sample, input1):
  print(f"Hello Day 13!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")
  assert(soln == 405)

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")
  assert(soln == 37975)

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")
  assert(soln == 400)

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
  assert(soln == 32497)
