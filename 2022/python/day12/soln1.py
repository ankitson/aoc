from collections import deque
import curses
import time

def nbrs(point, grid):
  (prow,pcol) = point
  cands = [(prow-1,pcol),(prow+1,pcol),(prow,pcol-1),(prow,pcol+1)]
  valid_coord = [(nrow,ncol) for (nrow,ncol) in cands if nrow >= 0 and nrow < len(grid) and ncol >= 0 and ncol < len(grid[0])]
  valid = [(nrow,ncol) for (nrow,ncol) in valid_coord if grid[nrow][ncol]-grid[prow][pcol] <= 1]
  return valid
  
def climb(grid, stdscr=None, fps=1, part2=False) -> int:
  starts = []
  end = None
  for (rown,row) in enumerate(grid):
    for (coln, char) in enumerate(row):
      if char == 'S' or (char == 'a' and part2):
        starts.append((rown, coln))
        grid[rown][coln] = 0
      elif char == 'E':
        end = (rown,coln)
        grid[rown][coln] = 25
      else:
        grid[rown][coln] = ord(char) - ord('a')
  
  def bfs(starts, end, viz_data):
    nonlocal grid, stdscr, fps
    dist = 0
    to_visit = deque(starts)
    next_layer = deque([])
    seen = set()
    while len(to_visit) > 0:
      (vr,vc) = to_visit.popleft()
      seen.add((vr,vc))
      if (vr,vc) == end:
        return dist
      else:
        nbr = nbrs((vr,vc), grid)
        for n in nbr:
          if not n in seen and not n in next_layer:
            next_layer.append(n)
      if len(to_visit) == 0:
        dist += 1
        to_visit = deque([x for x in next_layer])
        next_layer = deque([])
        if stdscr is not None:
          fps_cap = viz_data['fps_cap']
          c = stdscr.getch()
          if c == ord('>'):
            fps += 1
          elif c == ord('<'):
            fps -= 1
          elif c == ord('u'):
            fps_cap = not fps_cap
            viz_data['fps_cap'] = fps_cap

          stdscr.move(0,0)
          for rownum in range(0,len(grid)):
            for colnum in range(0,len(grid[rownum])):
              item = grid[rownum][colnum]
              item = chr(item + ord('a'))
              if (rownum,colnum) in starts:
                stdscr.addstr(item, curses.color_pair(1) | curses.A_STANDOUT)
              elif (rownum,colnum) == end:
                stdscr.addstr(item, curses.color_pair(2) | curses.A_STANDOUT)
              elif (rownum, colnum) in to_visit:
                stdscr.addstr(item, curses.color_pair(3) | curses.A_STANDOUT)
              elif (rownum, colnum) in seen:
                stdscr.addstr(item, curses.color_pair(4) | curses.A_STANDOUT)
              else:
                stdscr.addstr(item)
            stdscr.addstr("\n")
          stdscr.addstr(f"Controls:",curses.A_UNDERLINE)
          stdscr.addstr(f"\">\":faster \"<\":slower \"u\":toggle (un)cap fps\t")
          stdscr.addstr(f"FPS:",curses.A_UNDERLINE)
          stdscr.addstr(f"{'uncapped' if not fps_cap else fps}\t")
          stdscr.addstr(f"Starts = {starts[0]}.., current dist = {dist}, current min dist = {viz_data['min_dist']}\n")
          stdscr.refresh()
          if fps_cap:
            time.sleep(1/fps)
    return float('inf')

  viz_data = {'min_dist': float('inf'), 'fps_cap': False}
  min_dist = bfs(starts, end, viz_data)
  return min_dist

def parse(raw_input):
  grid = [list(row.strip()) for row in raw_input.split('\n') if len(row) > 0]
  return grid

def part1(raw_input):
  grid = parse(raw_input)
  return part1_core(grid)

def part1_core(grid):
  return climb(grid)

def part2(raw_input):
  grid = parse(raw_input)
  return part2_core(grid)

def part2_core(grid):
  return climb(grid, part2=True)

def wait_for_anykey(stdscr):
  stdscr.nodelay(False)
  stdscr.addstr("Press any key to continue...\n")
  stdscr.getch()
  stdscr.nodelay(True)

def main(sample, input1):
  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")
  assert(soln == 31)

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")
  assert(soln == 425)

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")
  assert(soln == 29)

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
  assert(soln == 418)
  
def anim_main(sample, input1, stdscr: curses.window):
  curses.init_pair(1, curses.COLOR_BLUE, curses.COLOR_BLACK)
  curses.init_pair(2, curses.COLOR_MAGENTA, curses.COLOR_BLACK)
  curses.init_pair(3, curses.COLOR_GREEN, curses.COLOR_BLACK)
  curses.init_pair(4, curses.COLOR_RED, curses.COLOR_BLACK)
  stdscr.nodelay(True)
  FPS = 15
  
  grid = [list(row.strip()) for row in sample.split('\n')]
  soln = climb(grid, stdscr, fps=FPS)
  stdscr.addstr(f"\nPart 1 (sample) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()
  
  grid = [list(row.strip()) for row in input1.split('\n') if len(row) > 0]
  soln = climb(grid, stdscr, fps=FPS)
  stdscr.move(0,0)
  stdscr.addstr(f"\nPart 1 (input) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()

  grid = [list(row.strip()) for row in sample.split('\n')]
  soln = climb(grid, stdscr, fps=FPS, part2=True)
  stdscr.addstr(f"\nPart 2 (sample) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()

  grid = [list(row.strip()) for row in input1.split('\n') if len(row) > 0]
  soln = climb(grid, stdscr, fps=FPS, part2=True)
  stdscr.move(0,0)
  stdscr.addstr(f"\nPart 2 (input) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)