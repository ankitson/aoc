
import copy

def parse(raw_input):
  grids = raw_input.split("\n\n")
  ret = []
  for grid in grids:
    lines = grid.split("\n")
    gridp = [list(line) for line in lines if len(line) > 2]
    ret.append(gridp)
  return ret

def pgrid(grid):
  for row in grid:
    for col in row:
      print(col,end="")
    print()

def solve(grid):
  width = len(grid[0])
  candidates = set(list(range(1,width)))
  reflect_sizes = {}
  for (ri,row) in enumerate(grid):
    reflect_sizes[ri] = {}
    for i in range(1,len(row)): #mirror is before position i
      fwd = i
      bwd = i -1
      if row[fwd] != row[bwd]:
        if i in candidates:
          candidates.remove(i)
      reflect_size = 1
      while fwd < width and bwd >= 0 and row[fwd] == row[bwd]:
        reflect_size += 1
        fwd += 1
        bwd -= 1
        if fwd < width and bwd >=0 and row[fwd] != row[bwd]:
          if i in candidates:
            candidates.remove(i)
      reflect_sizes[ri][i] = reflect_size 
  return candidates

def part1(input):
  grids = parse(input)
  print(len(grids))
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

    if reflect_vertical and reflect_horiz:
      if reflect_vertical > reflect_horiz:
        print(f"v {reflect_vertical} for grid")
        total += reflect_vertical
      else:
        print(f"h {reflect_horiz} for grid")
        total += (100*reflect_horiz)
    elif reflect_vertical is not None or reflect_horiz is not None:
      if reflect_vertical is not None:
        total += reflect_vertical
      else:
        total += (100*reflect_horiz) # type: ignore
    else:
      print(f"no refles")
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
        new = "#" if grid[smudge_row][smudge_col] == '.' else '.'
        new_grid = copy.deepcopy(grid)
        new_grid[smudge_row][smudge_col] = new

        reflect_vertical = solve(new_grid) - found_v
        reflect_vertical = list(reflect_vertical)[0] if len(reflect_vertical) > 0 else None

        transpose = [[0 for i in range(len(new_grid))] for j in range(len(new_grid[0]))]
        for i in range(len(grid)):
          for j in range(len(grid[0])):
            transpose[j][i] = new_grid[i][j]

        reflect_horiz = solve(transpose) - found_h
        reflect_horiz = list(reflect_horiz)[0] if reflect_horiz and len(reflect_horiz) > 0 else None

        if reflect_vertical and reflect_horiz:
          if reflect_vertical > reflect_horiz and not reflect_vertical in found_v:
            print(f"v={reflect_vertical}",end="")
            total += reflect_vertical
            found_v.add(reflect_vertical)
          elif reflect_horiz and not reflect_horiz in found_h:
            print(f"h={reflect_horiz}",end="")
            total += (100*reflect_horiz)
            found_h.add(reflect_horiz)
          done = True
        elif reflect_vertical is not None or reflect_horiz is not None:
          if reflect_vertical is not None and not reflect_vertical in found_v:
            print(f"v={reflect_vertical}",end="")
            total += reflect_vertical
            found_v.add(reflect_vertical)
          elif reflect_horiz and (not found_h or not reflect_horiz in found_h):
            print(f"h={reflect_horiz}",end="")
            total += (100*reflect_horiz)
            found_h.add(reflect_horiz)
          done = True
        else:
          pass
    print(f" total = {total}")
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

