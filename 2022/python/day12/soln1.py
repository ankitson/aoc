from collections import deque
import curses
import itertools
import time

def nbrs(point, grid):
  (prow,pcol) = point
  cands = [(prow-1,pcol),(prow+1,pcol),(prow,pcol-1),(prow,pcol+1)]
  valid_coord = [(nrow,ncol) for (nrow,ncol) in cands if nrow >= 0 and nrow < len(grid) and ncol >= 0 and ncol < len(grid[0])]
  valid = [(nrow,ncol) for (nrow,ncol) in valid_coord if grid[nrow][ncol]-grid[prow][pcol] <= 1]
  return valid

def wait_for_anykey(stdscr):
  stdscr.nodelay(False)
  stdscr.addstr("Press any key to continue...\n")
  stdscr.getch()
  stdscr.nodelay(True)
  
def climb(grid, stdscr=None, fps=1, part2=False) -> int:
  start = None
  end = None
  for (rown,row) in enumerate(grid):
    for (coln, char) in enumerate(row):
      if char == 'S':
        start = (rown, coln)
        grid[rown][coln] = 0
      elif char == 'E':
        end = (rown,coln)
        grid[rown][coln] = 25
      else:
        grid[rown][coln] = ord(char) - ord('a')
  
  def bfs(start, end, viz_data):
    nonlocal grid, stdscr, fps
    dist = 0
    to_visit = deque([start])
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
              if (rownum,colnum) == start:
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
          stdscr.addstr(f"Start = {start}, current dist = {dist}, current min dist = {viz_data['min_dist']}\n")
          stdscr.refresh()
          if fps_cap:
            time.sleep(1/fps)
        next_layer = deque([])
    return float('inf')

  start_at = [start]
  if part2:
    start_at = [(srow,scol) for (srow,scol) in itertools.product(range(0,len(grid)),range(0,len(grid[0]))) if grid[srow][scol] == 0]
  min_dist = float('inf')
  viz_data = {'min_dist': min_dist, 'fps_cap': False}
  for start_pos in start_at:
    dist = bfs(start_pos,end,viz_data)
    min_dist = min(min_dist, dist)
    viz_data['min_dist'] = min_dist
  return min_dist

def main(stdscr: curses.window):
  curses.init_pair(1, curses.COLOR_BLUE, curses.COLOR_BLACK)
  curses.init_pair(2, curses.COLOR_MAGENTA, curses.COLOR_BLACK)
  curses.init_pair(3, curses.COLOR_GREEN, curses.COLOR_BLACK)
  curses.init_pair(4, curses.COLOR_RED, curses.COLOR_BLACK)
  stdscr.nodelay(True)
  FPS = 15
  
  sample = open('../inputs/sample12.txt', 'r').read()
  day12 = open('../inputs/day12.txt', 'r').read()
  
  grid = [list(row.strip()) for row in sample.split('\n')]
  soln = climb(grid, stdscr, fps=FPS)
  assert(soln == 31)
  stdscr.addstr(f"\nPart 1 (sample) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()
  
  grid = [list(row.strip()) for row in day12.split('\n') if len(row) > 0]
  soln = climb(grid, stdscr, fps=FPS)
  assert(soln == 425)
  stdscr.move(0,0)
  stdscr.addstr(f"\nPart 1 (input) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()

  grid = [list(row.strip()) for row in sample.split('\n')]
  soln = climb(grid, stdscr, fps=FPS, part2=True)
  assert(soln == 29)
  stdscr.addstr(f"\nPart 2 (sample) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()

  grid = [list(row.strip()) for row in day12.split('\n') if len(row) > 0]
  soln = climb(grid, stdscr, fps=FPS, part2=True)
  assert(soln == 418)
  stdscr.move(0,0)
  stdscr.addstr(f"\nPart 2 (input) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)

if __name__ == '__main__':
  try:
    curses.wrapper(main)
  except curses.error as e:
    print("Curses error. Likely your terminal is too small to display the grid")
    import traceback
    traceback.print_exc()

