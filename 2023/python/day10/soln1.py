import curses
import math, itertools, sys
from collections import defaultdict, deque
import time

def nbrs(grid, x, y):
  cands = [(x-1,y),(x+1,y),(x,y-1),(x,y+1)]


  valid = [(a,b) for (a,b) in cands if a >= 0 and b >= 0 and a < len(grid) and b < len(grid[0])]
  if x == 52 and y == 98:
    print(f"valid = {valid}")
  curr = (grid[x][y])

  typ = {
    '|': 'UD',
    '-' : 'LR',
    'L' : 'UR',
    'J' : 'UL',
    '7' : 'DL',
    'F' : 'DR',
    '.' : 'G',
    'S' : 'S'
  }

  filtered = []
  cur_typ = typ[curr]
  for (dx,dy) in [(-1,0),(1,0),(0,-1),(0,1)]:
    nx,ny = x+dx,y+dy
    if (nx,ny) in valid:
      if x == 52 and y == 98 and nx == 52 and ny == 97:
        print(f"consider left nbr for dx,dy={dx},{dy}")
      if (dx,dy) == (-1,0): #UP
        if typ[grid[x][y]].startswith('U') or typ[grid[x][y]] in ['UD','S'] and (typ[grid[nx][ny]].startswith('D') or grid[nx][ny] in ['|','S']):
          filtered.append(((dx,dy), (nx,ny)))
      elif (dx,dy) == (1,0): #DOWN
        if typ[grid[x][y]].startswith('D') or typ[grid[x][y]] in ['UD','S'] and (typ[grid[nx][ny]].startswith('U') or grid[nx][ny] in ['|','S']):
          filtered.append(((dx,dy), (nx,ny)))
      elif (dx,dy) == (0,-1): #LEFT
        if typ[grid[x][y]].endswith('L') or typ[grid[x][y]] in ['LR','S'] and (typ[grid[nx][ny]].endswith('R') or grid[nx][ny] in ['-','S']):
          filtered.append(((dx,dy), (nx,ny)))
      elif (dx,dy) == (0,1): #RIGHT
        if typ[grid[x][y]].endswith('R') or typ[grid[x][y]] in ['LR','S'] and (typ[grid[nx][ny]].endswith('L') or grid[nx][ny] in ['-','S']):
          filtered.append(((dx,dy), (nx,ny)))
  if x == 52 and y == 98:
    print(f"filtered = {filtered}")
  return filtered

def bfs(nodes, start, stdscr, fps_cap, fps):
  to_visit = deque([(start,0)])
  # to_visit = deque([(next(iter(nodes.keys())),0)])
  next_layer = deque([])
  seen = set()
  dseen = {}
  grid = nodes
  # cdepth = 0
  while len(to_visit) > 0:
    ((vx,vy), depth) = to_visit.popleft()
    if stdscr is not None:
      # fps_cap = viz_data['fps_cap']
      c = stdscr.getch()
      if c == ord('>'):
        fps += 1
      elif c == ord('<'):
        fps -= 1
      elif c == ord('u'):
        fps_cap = not fps_cap

      stdscr.move(0,0)
      for rownum in range(0,len(grid)):
        for colnum in range(0,len(grid[0])):
          item = grid[rownum][colnum]
          if (rownum,colnum) == start:
            stdscr.addstr(item, curses.color_pair(1) | curses.A_STANDOUT)
          elif (rownum, colnum) in [a[1] for (a,b) in next_layer]:
            stdscr.addstr(item, curses.color_pair(3) | curses.A_STANDOUT)
          elif (rownum, colnum) in seen:
            stdscr.addstr(item, curses.color_pair(4) | curses.A_STANDOUT)
          else:
            stdscr.addstr(item)
        stdscr.addstr("\n")
      stdscr.addstr(f"Controls:",curses.A_UNDERLINE)
      stdscr.addstr(f"CUrrent NOde = {vx},{vy}")
      stdscr.addstr(f"\">\":faster \"<\":slower \"u\":toggle (un)cap fps\t")
      stdscr.addstr(f"FPS:",curses.A_UNDERLINE)
      stdscr.addstr(f"{'uncapped' if not fps_cap else fps}\t")
      
          # stdscr.addstr(f"Starts = {starts[0]}.., current dist = {dist}, current min dist = {viz_data['min_dist']}\n")
      stdscr.refresh()
      if fps_cap:
        if fps == 0:
            stdscr.nodelay(False)
            stdscr.addstr("Press any key to continue...\n")
            c = stdscr.getch()
            if c == ord('>'):
              fps += 1
            stdscr.nodelay(True)
        time.sleep(1/fps)

    seen.add((vx,vy))
    dseen[(vx,vy)] = depth
    # print(f"Visit {visit}")
    this_nbrs = nbrs(nodes, vx,vy)

    # annotated = [(nx,ny,nodes[nx][ny]) for ((dx,dy),(nx,ny)) in this_nbrs]
    # if (vx,vy) == (52,98):
      # print(f"NBRS OF THIS ONE = {annotated}")
    # print(f"nbrs: = {annotated}")
    for ((dx,dy),(nx,ny)) in this_nbrs:
      if not (nx,ny) in seen and not (nx,ny) in [t[0] for t in next_layer]:
        next_layer.append(((nx,ny),depth+1))
    
    if len(to_visit) == 0:
      # annotated_next = [(nx,ny,nodes[nx][ny]) for ((nx,ny),depth) in next_layer]
      # stdscr.addstr(f"Next layer  at depth {depth+1 }= {next_layer}")
      # print(f"layer at depth {depth+1} = {annotated_next}")
      to_visit = next_layer
      next_layer = deque([])
  return (seen,dseen)


def stack_dfs(nodes, start):
  stack = [(start, 0, [])]  # Initialize stack with start node, depth 0, and empty path
  seen = set()  # Initialize set to keep track of visited nodes

  while stack:
    curr, depth, path = stack.pop()  # Pop the top node from the stack
    cx, cy = curr

    if (cx, cy) == start and depth > 0:  # We made it back to start
      return depth, path

    if (cx, cy) in seen:
      continue

    seen.add((cx, cy))
    mynbrs = nbrs(nodes, cx, cy)

    for nbdst, nbr in mynbrs:
      if nbr not in seen or (nbr == start and depth > 1):
        stack.append((nbr, depth + 1, path + [nbr]))  # Push neighbor node to stack with increased depth and updated path

  return float('-inf'), []  # If no path found, return negative infinity and empty path

def dfs(nodes, curr, start, seen, depth=0):
  print(f"curr = {curr}, start = {start}, depth = {depth}")
  (cx,cy) = curr
  (sx,sy) = start
  if (cx,cy) == (sx,sy) and depth > 0: #we made it back to start
    return depth
  elif (cx,cy) in seen:
    return float('-inf')
  seen.add((cx,cy))
  mynbrs = nbrs(nodes,cx,cy)
  max_dist = float('-inf')
  # print(f"seen = {seen}")
  for (nbdst,nbr) in mynbrs:
    print(nbr)
    if (not nbr in seen) or ((nbr == start) and depth > 1):
      dist = dfs(nodes, nbr, start, seen, depth+1)
      max_dist = max(max_dist, dist)
  return max_dist // 2

def parse(raw_input):
  lines = [list(x) for x in raw_input.split("\n") if len(x) > 1]
  start_idx = None
  for i in range(len(lines)):
    for j in range(len(lines[i])):
      if lines[i][j] == 'S':
        start_idx = (i,j)
        break
  

  return (lines,start_idx)

def part1(input):
  input,start_idx = parse(input)
  # print(f"Part 1 input = {input}")
  # (seen,dseen) = bfs(input, start_idx, None, None, None)

  (dist,path) = stack_dfs(input, start_idx)
  print(path)
  return dist // 2

def part2(input):
  input,start_idx = parse(input)
  (dist,path) = stack_dfs(input, start_idx)
  
  (sx,sy) = start_idx
  (odx,ody) = (path[1][0]-sx,path[1][1]-sy)
  # (nx,ny) = path[1]


  print(f"path = {path}")
  #(below us means (1,0))
  in_the_thing = set()
  for i in range(1,len(path)): #ssuming it loops around on our left    
    x,y = path[i]
    thing = input[x][y]
    if thing == 'L':
      (odx,ody) = ()
    (px,py) = path[i-1]
    dx = px - x
    dy = py - y
    print(f"dx,dy = {dx},{dy}")
    if dx == 0 and dy == 1:
      (ldx,ldy) = (-1,0)
    elif dx == 1 and dy == 0:
      (ldx,ldy) = (0,1)
    elif dx == 0 and dy == -1:
      (ldx,ldy) = (1,0)
    elif dx == -1 and dy == 0:
      (ldx,ldy) = (0,-1)
    else:
      raise Exception("wat")
    # (ldx,ldy) = (dx,dy-1)
    (lx,ly) = (x+ldx,y+ldy)
    print(f"im at {x},{y} and left is {lx},{ly}")
    if input[lx][ly] == '.':
      print(f"adding left {lx},{ly}")
      in_the_thing.add((lx,ly))
  return len(in_the_thing)


    


  raise Exception("not implemented!")

def main(sample, sample2, sample3, sample4, sample5, sample6, input1):
  print(f"Hello Day 10!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(sample2)
  print(f"Part 1 (sample2) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample5)
  print(f"Part 2 (sample5) = {soln}")
  # soln = part2(sample6)
  # print(f"Part 2 (sample6) = {soln}")

  # soln = part2(input1)
  # print(f"Part 2 (realinput) = {soln}")

def wait_for_anykey(stdscr):
  stdscr.nodelay(False)
  stdscr.addstr("Press any key to continue...\n")
  stdscr.getch()
  stdscr.nodelay(True)

def anim_main(sample, sample2, input , stdscr):
  curses.init_pair(1, curses.COLOR_BLUE, curses.COLOR_BLACK)
  curses.init_pair(2, curses.COLOR_MAGENTA, curses.COLOR_BLACK)
  curses.init_pair(3, curses.COLOR_GREEN, curses.COLOR_BLACK)
  curses.init_pair(4, curses.COLOR_RED, curses.COLOR_BLACK)
  stdscr.nodelay(True)
  FPS = 1

  input,start_idx = parse(input)
  # print(f"Part 1 input = {input}")
  # (seen,dseen) = bfs(input, start_idx)
  # return max(dseen.values())

  soln = bfs(input, start_idx, stdscr, fps_cap=True, fps=FPS)
  stdscr.addstr(f"\nPart 1 (sample) = {soln}\n", curses.color_pair(1) | curses.A_STANDOUT)
  wait_for_anykey(stdscr)
  stdscr.clear()