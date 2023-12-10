import math, itertools, sys
from collections import defaultdict, deque

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

def parse(raw_input):
  return raw_input
  raise Exception("not implemented!")

def part1(input):
  raise Exception("not implemented!")

def part2(input):
  raise Exception("not implemented!")

def main(sample, input1):
  print(f"Hello Day 10!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

