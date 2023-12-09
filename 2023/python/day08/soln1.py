import math

def parse(raw_input):
  lines = raw_input.split("\n")
  instructions = [x for x in lines[0]]

  nodes = {}
  for line in lines[2:]:
    if len(line) < 2:
      continue
    splits = line.split(" = ")
    node = splits[0]
    splits2 = splits[1].split(", ")
    lnbr = splits2[0][1:]
    rnbr = splits2[1][:len(splits2[1])-1]
    nodes[node] = (lnbr,rnbr)
  return (instructions, nodes)

def part1(input):
  (ins,ns) = parse(input)
  start = 'AAA'
  curr = start
  nins = 0
  while curr != 'ZZZ':
    cins = ins[nins % len(ins)]
    if cins == 'L':
      curr = ns[curr][0]
    else:
      curr = ns[curr][1]
    nins += 1
  return nins

def part2(input):
  (ins,ns) = parse(input)
  starts = [node for node in ns.keys() if node[-1] == 'A']
  reaches = {}
  for start in starts:
    curr = start
    nins = 0
    while curr[-1] != 'Z':
      cins = ins[nins % len(ins)]
      if cins == 'L':
        curr = ns[curr][0]
      else:
        curr = ns[curr][1]
      nins += 1
    reaches[start] = nins
  vals = list(reaches.values())
  return math.lcm(*vals)

def main(sample, sample2, input1):
  print(f"Hello Day 08!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")
  assert(soln == 2)

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")
  assert(soln == 16409)

  soln = part2(sample2)
  print(f"Part 2 (sample2) = {soln}")
  assert(soln == 6)

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
  assert(soln == 11795205644011)
