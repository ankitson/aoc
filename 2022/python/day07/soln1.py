from typing import Dict

def wspc(s: str):
  i = 0
  while i < len(s) and (s[i] == ' ' or s[i] == '\n'):
    i += 1
  return s[i:]

def expect(s: str, tok: str):
  s = wspc(s)
  substr = s[0:len(tok)]
  if (substr == tok):
    return s[len(tok):]
  else:
    raise Exception(f"expected {tok} got {substr} at '{s[0:5]}'..")

def accept(s:str, typ):
  s = wspc(s)
  i = 0
  if typ == int:
    num = []
    while i < len(s) and s[i].isdigit():
      num.append(s[i])
      i += 1
    if i == 0:
      return None
    else:
      nval = int(''.join(num))
      return (s[i:],nval)
  elif typ == str:
    name = []
    while i < len(s) and not (s[i].isdigit() or s[i].isspace()):
      name.append(s[i])
      i += 1
    if i == 0:
      return None
    else:
      name = ''.join(name)
      return (s[i:],name)
  else:
    raise Exception(f"unhandled type {typ}")

def parse_tree(content: str) -> Dict:
  rem = content
  dirtree = {'/': {}}
  currdir = []
  while len(rem) > 0:
    rem = expect(rem, "$")
    (rem, cmd) = accept(rem, str)
    if cmd == "ls":
      while True:
        try:
          expect(rem, "$")
          break
        except Exception:
          pass
        trynum = accept(rem, int)
        if trynum:
          (rem, nval) = trynum
          (rem, fname) = accept(rem, str)
          inode = dirtree
          for path in currdir:
            inode = inode[path]
          inode[fname] = nval
        else:
          if len(rem) == 0:
            break
          rem = expect(rem, "dir")
          (rem, dname) = accept(rem, str)
          inode = dirtree
          for path in currdir:
            inode = inode[path]
          inode[dname] = {}
    elif cmd == "cd":
      (rem, dir) = accept(rem, str)
      if dir == "..":
        currdir.pop()
      else:
        currdir.append(dir)
  return dirtree

def pop_sizes(dirtree: Dict):
  dirsize = 0
  for (key,val) in dirtree.items():
    if type(val) == dict:
      subdir_size = pop_sizes(val)
      dirtree[key] = (subdir_size, val)
      dirsize += subdir_size
    elif type(val) == int:
      dirsize += val
      continue
  return dirsize

def pretty_print_recurse(dirtree: Dict, ident=0) -> None:
  for (key,val) in dirtree.items():
    print(" "*ident, end="")
    if type(val) == tuple:
      (subdirsize, dir) = val
      print(f"- {key} (dir, size={subdirsize})")
      pretty_print_recurse(dir, ident+2)
    elif type(val) == int:
      print(f"- {key} (file, size={val})")

def part1(input_str: str, max_size: int) -> int:
  dirtree = parse_tree(input_str)
  pop_sizes(dirtree)
  print("Directory Tree: ")
  pretty_print_recurse(dirtree)

  def helper(dir, max_size):
    if len(dir) == 0:
      return 0
    sum = 0
    for (key,val) in dir.items():
      if type(val) == tuple:
        (dirsize, subdir) = val
        if dirsize <= max_size:
          sum += dirsize
        sum += helper(subdir, max_size)
    return sum
  return helper(dirtree, max_size)

def part2(input_str: str, total_space: int, free_space: int) -> int:
  dirtree = parse_tree(input_str)
  pop_sizes(dirtree)

  needed_space = free_space - (total_space - dirtree['/'][0])
  best = float('inf')
  def helper(dir):
    nonlocal best
    if len(dir) == 0:
      return
    for (key,val) in dir.items():
      if type(val) == tuple:
        (subdirsize, subdir) = val
        if (subdirsize < best and subdirsize >= needed_space):
          best = subdirsize
        helper(subdir)
  helper(dirtree)
  return best

sample = open('../inputs/sample07.txt').read()
input1 = open('../inputs/day07.txt').read()

print("Hello Day 7!")
print(f"Part 1 (sample) = {part1(sample, 100000)}") #95437
print(f"Part 1 (realinput) = {part1(input1, 100000)}") #1642503
print(f"Part 2 (sample) = {part2(sample, 70_000_000, 30_000_000)}") #24933642
print(f"Part 2 (realinput) = {part2(input1, 70_000_000, 30_000_000)}") #6999588
