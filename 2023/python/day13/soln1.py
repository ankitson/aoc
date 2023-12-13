
import copy


def parse(raw_input):
  grids = raw_input.split("\n\n")
  ret = []
  for grid in grids:
    lines = grid.split("\n")
    gridp = [list(line) for line in lines if len(line) > 2]
    ret.append(gridp)
  return ret
  # return raw_input

def pgrid(grid):
  for row in grid:
    for col in row:
      print(col,end="")
    print()

def solve(grid):
  # print("SOLVE: ")
  # pgrid(grid)
  width = len(grid[0])
  candidates = set(list(range(1,width)))
  reflect_sizes = {}
  for (ri,row) in enumerate(grid):
    reflect_sizes[ri] = {}
    #mirror is before position i
    # print(f"row = {row}")
    for i in range(1,len(row)):
      fwd = i
      bwd = i -1
      # print(fwd,bwd)
      if row[fwd] != row[bwd]:
        if i in candidates:
          candidates.remove(i)
      # if i == 7:
        # print(f"i = 7, fwd = {row[fwd]} bwd = {row[bwd]}")
      reflect_size = 1
      while fwd < width and bwd >= 0 and row[fwd] == row[bwd]:
        reflect_size += 1
        fwd += 1
        bwd -= 1
        if fwd < width and bwd >=0 and row[fwd] != row[bwd]:
          if i in candidates:
            candidates.remove(i)
      reflect_sizes[ri][i] = reflect_size    
    # print(candidates)
  if len(candidates) == 1:
    size = list(candidates)[0]
    # print(f"size = {size}")
    return set([size])
  if len(candidates) > 1:
    return candidates
    best_rlen = float('-inf')
    best_cand = None
    print(f"MULTIPLE CANDIDATES = {list(candidates)}")
    for cand in list(candidates):
      rlen = reflect_sizes[0][cand]
      if rlen > best_rlen:
        best_rlen = rlen
        best_cand = cand
    return best_cand
    print(candidates)
    raise Exception("todo")
  else:
    return None

def part1(input):
  grids = parse(input)
  print(len(grids))
  total = 0
  for grid in grids:
    
    reflect_vertical = solve(grid)

    transpose = [[0 for i in range(len(grid))] for j in range(len(grid[0]))]
    # print(len(transpose),len(grid))
    # print(len(transpose[0]),len(grid[0]))
    for i in range(len(grid)):
      for j in range(len(grid[0])):
        transpose[j][i] = grid[i][j]

    reflect_horiz = solve(transpose)

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
        total += (100*reflect_horiz)
    else:
      print(f"no refles")
  return total

def part2(input):
  grids = parse(input)
  print(len(grids))
  total = 0
  print(f"ALL GRIDS: {grids} LENGTH = {len(grids)}")
  for grid in grids:
    print("NEW GRID")
    pgrid(grid)
    orig_v = solve(grid)
    transpose = [[0 for i in range(len(grid))] for j in range(len(grid[0]))]
    # print(len(transpose),len(grid))
    # print(len(transpose[0]),len(grid[0]))
    for i in range(len(grid)):
      for j in range(len(grid[0])):
        transpose[j][i] = grid[i][j]
    orig_r = solve(transpose)
    print(f"Orig = {orig_v} {orig_r}")

    found_h = orig_r
    if found_h is None:
      found_h = set()
    found_v = orig_v
    if found_v is None:
      found_v = set()
    for smudge_row in range(len(grid)):
      for smudge_col in range(len(grid[0])):

        # print(f"Try smudge {smudge_row} {smudge_col}")
        new = "#" if grid[smudge_row][smudge_col] == '.' else '.'
        new_grid = copy.deepcopy(grid)
        new_grid[smudge_row][smudge_col] = new
        # print(grid[0])

        reflect_vertical = solve(new_grid)
        new_reflect_vertical = reflect_vertical
        if reflect_vertical and found_v:
          new_reflect_vertical = reflect_vertical - found_v
        if new_reflect_vertical and len(new_reflect_vertical) > 0:
          reflect_vertical = list(new_reflect_vertical)[0]
        else:
          reflect_vertical = None
        # if reflect_vertical == orig_v:
          # reflect_vertical = None

        transpose = [[0 for i in range(len(new_grid))] for j in range(len(new_grid[0]))]
        # print(len(transpose),len(grid))
        # print(len(transpose[0]),len(grid[0]))
        for i in range(len(grid)):
          for j in range(len(grid[0])):
            transpose[j][i] = new_grid[i][j]

        reflect_horiz = solve(transpose)
        new_reflect_horiz = reflect_horiz
        if reflect_horiz and found_h:
          new_reflect_horiz = reflect_horiz - found_h
        if reflect_horiz and new_reflect_horiz and len(new_reflect_horiz) > 0:
          reflect_horiz = list(new_reflect_horiz)[0]
        else:
          reflect_horiz = None
        # print(f"HORIZ = {reflect_horiz} org = {orig_r} vertical = {reflect_vertical}")
        # if reflect_horiz == orig_r:
          # reflect_horiz = None

        if reflect_vertical and reflect_horiz:
          if reflect_vertical > reflect_horiz and not reflect_vertical in found_v:
            print(f"FOUND v {reflect_vertical} for grid")
            total += reflect_vertical
            found_v.add(reflect_vertical)
          elif reflect_horiz and not reflect_horiz in found_h:
            print(f"FOUND h {reflect_horiz} for grid")
            total += (100*reflect_horiz)
            found_h.add(reflect_horiz)
        elif reflect_vertical is not None or reflect_horiz is not None:
          if reflect_vertical is not None and not reflect_vertical in found_v:
            print(f"FOUND v {reflect_vertical} for grid")
            total += reflect_vertical
            found_v.add(reflect_vertical)
          elif reflect_horiz and (not found_h or not reflect_horiz in found_h):
            print(f"FOUND h {reflect_horiz} for grid")
            total += (100*reflect_horiz)
            found_h.add(reflect_horiz)
        else:
          pass
          # print(f"no refles")
    print(f"Total for grid = {total}")
  return total

    # for candidate in candidates:
      # all_equal = True
      # all_best = None
      # for (ri,row) in enumerate(grid[0:1]):
        # if all_best is None:
          # all_best


  # return candidates






def main(sample, input1):
  print(f"Hello Day 13!")

  # soln = part1(sample)
  # print(f"Part 1 (sample) = {soln}")

  # soln = part1(input1)
  # print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

