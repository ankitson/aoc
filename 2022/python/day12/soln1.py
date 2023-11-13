from collections import deque
import curses
import time

def climb(grid, stdscr=None, fps=1) -> int:
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
  
  def bfs():
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
        nbr = nbrs((vr,vc))
        for n in nbr:
          if not n in seen and not n in next_layer:
            next_layer.append(n)
      if len(to_visit) == 0:
        dist += 1
        to_visit = deque([x for x in next_layer])
      
        if stdscr is not None:
          c = stdscr.getch()
          if c == ord('>'):
            fps += 1
          elif c == ord('<'):
            fps -= 1            

          stdscr.move(0,0)
          for rownum in range(0,len(grid)):
            for colnum in range(0,len(grid[rownum])):
              item = grid[rownum][colnum]
              item = chr(item + ord('a'))
              if (rownum, colnum) in to_visit:
                stdscr.addstr(item, curses.color_pair(1) | curses.A_STANDOUT)
              elif (rownum, colnum) in seen:
                stdscr.addstr(item, curses.color_pair(2) | curses.A_STANDOUT)
              else:
                stdscr.addstr(item)
            stdscr.addstr("\n")
          stdscr.addstr(f"Visited at dist {dist-1}\n")
          stdscr.addstr(f"Controls:\n>: faster\t<: slower\n")
          stdscr.addstr(f"Current FPS: {fps}")
          stdscr.refresh()
          time.sleep(1/fps)
        next_layer = deque([])
  
  def nbrs(point):
    (prow,pcol) = point
    cands = [(prow-1,pcol),(prow+1,pcol),(prow,pcol-1),(prow,pcol+1)]
    valid_coord = [(nrow,ncol) for (nrow,ncol) in cands if nrow >= 0 and nrow < len(grid) and ncol >= 0 and ncol < len(grid[0])]
    valid = [(nrow,ncol) for (nrow,ncol) in valid_coord if grid[nrow][ncol]-grid[prow][pcol] <= 1]
    return valid

  dist = bfs()
  return dist

def main(stdscr):
  curses.init_pair(1, curses.COLOR_GREEN, curses.COLOR_BLACK)
  curses.init_pair(2, curses.COLOR_RED, curses.COLOR_BLACK)
  stdscr.nodelay(True)

  print("Welcome to Day 12!")
  f = open('../inputs/sample12.txt', 'r')
  raw_input = f.read()
  grid = [list(row.strip()) for row in raw_input.split('\n')]
  soln = climb(grid, stdscr, fps=60)

  stdscr.nodelay(False)
  stdscr.addstr(f"\nPart 1 (sample) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  stdscr.addstr("Press any key to continue...\n")
  stdscr.getch()

  stdscr.nodelay(True)
  
  f = open('../inputs/day12.txt', 'r')
  raw_input = f.read()
  grid = [list(row.strip()) for row in raw_input.split('\n') if len(row) > 0]
  soln = climb(grid, stdscr, fps=60)
  stdscr.nodelay(False)
  stdscr.move(0,0)
  stdscr.addstr(f"\nPart 1 (input) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  stdscr.addstr("Press any key to continue...\n")
  stdscr.getch()

try:
  curses.wrapper(main)
except curses.error as e:
  print("Curses error. Likely your terminal is too small to display the grid")
  import traceback
  traceback.print_exc()
