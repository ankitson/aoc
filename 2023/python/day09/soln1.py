import math, itertools, sys
from collections import defaultdict, deque

def parse(raw_input):
  lines = raw_input.split("\n")
  parsed = []
  for line in lines:
    nums = [int(x) for x in line.replace("  ", " ").split(" ") if len(line) > 2 ]
    parsed.append(nums)
  return parsed

def bfs(nodes):
  to_visit = deque([next(iter(nodes.keys()))])
  next_layer = deque([])
  seen = set()
  while len(to_visit) > 0:
    visit = to_visit.popleft()
    seen.add(visit)
    print(f"Visit {visit}")
    nbrs = nodes.get(visit, [])
    print(f"nbrs: = {nbrs}")
    for nbr in nbrs:
      if not nbr in seen and not nbr in next_layer:
        next_layer.append(nbr)
    print(f"next = {next_layer}")
    if len(to_visit) == 0:
      to_visit = next_layer
      next_layer = deque([])
  return

def part1(input):
  # graph = {'A': ['B','C','D'], 'B': ['D','E'], 'E': ['D', 'F', 'A"']}
  # bfs(graph)
  input = parse(input)
  print(f"Part 1 input = {input}")

  #next[d] = 0
  #next[d-1] 
  curr = input
  next = []
  depth = 0
  
  sum = 0
  for curr in input:
    if len(curr) < 1:
      continue
    print(f"input = {curr}")
    tree = []
    next = []
    while not all([x == 0 for x in curr]):
      for i in range(len(curr)-1):
        a = curr[i]
        b = curr[i+1]
        next.append(b-a)
      depth += 1
      tree.append(curr)
      curr = next
      next = []
    add = 0
    curr = 0
    tree.append([0] * (len(tree[-1]) - 1))
    print(f"tree begin = {tree}")
    tree[-1].append(0)

    for j in range(len(tree)-2,-1,-1):
      # this_one = tree[j]
      # extrapolate = tree[j][-1]
      tree[j].append(tree[j][-1] + tree[j+1][-1])
      # tree[j-1].append(extrapolate+tree[j-1][-1])
      # curr = curr + rev_list[-1] + add
      # add = 
    print(f"tree end = {tree[0]}")
    sum += tree[0][-1]

      # print(f"add {curr} to {rev_list}")
    # print(tree)
  return sum


    
  
  return depth

  # raise Exception("not implemented!")

def part2(input):
  print(f"Part 2 input = {input}")
    # graph = {'A': ['B','C','D'], 'B': ['D','E'], 'E': ['D', 'F', 'A"']}
  # bfs(graph)
  input = parse(input)
  print(f"Part 1 input = {input}")

  #next[d] = 0
  #next[d-1] 
  curr = input
  next = []
  depth = 0
  
  sum = 0
  for curr in input:
    if len(curr) < 1:
      continue
    print(f"input = {curr}")
    tree = []
    next = []
    while not all([x == 0 for x in curr]):
      for i in range(len(curr)-1):
        a = curr[i]
        b = curr[i+1]
        next.append(b-a)
      depth += 1
      tree.append(curr)
      curr = next
      next = []
    add = 0
    curr = 0
    tree.append([0] * (len(tree[-1]) - 1))
    print(f"tree begin = {tree}")
    tree[-1].append(0)

    for j in range(len(tree)-2,-1,-1):
      # this_one = tree[j]
      # extrapolate = tree[j][-1]
      #first + below = second . first = second - below
      tree[j].insert(0, tree[j][0] - tree[j+1][0])
      # tree[j-1].append(extrapolate+tree[j-1][-1])
      # curr = curr + rev_list[-1] + add
      # add = 
    print(f"tree end = {tree[0]}")
    sum += tree[0][0]

      # print(f"add {curr} to {rev_list}")
    # print(tree)
  return sum


    
  raise Exception("not implemented!")

def main(sample, input1):
  print(f"Hello Day 09!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

