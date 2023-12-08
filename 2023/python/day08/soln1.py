import copy
import math
from collections import defaultdict, deque

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
  # print(nodes)
  return (instructions, nodes)
  raise Exception("not implemented!")

def part1(input):
  (ins,ns) = parse(input)
  print(ins, ns)
  start = 'AAA'
  curr = start
  nins = 0
  print(ins)
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
  ends = [node for node in ns.keys() if node[-1] == 'Z']
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
  print(vals)
  return math.lcm(*vals)



  # currs = copy.deepcopy(starts)
  # next_currs = []
  # end = False
  # nins = 0
  # # print(ns)

  # print(f"starts: {len(starts)}")
  # print(f"ends: {len(ends)}")
  # # print(starts)
  # # return
  # while not end:
  #   cins = ins[nins % len(ins)]
  #   # print(cins)
  #   # print(f"curr = {currs}")
  #   for curr in currs:
  #     if cins == 'L':
  #       next_currs.append(ns[curr][0])
  #     else:
  #       # print(f"{ns[curr]}")
  #       # print(f"append right: {ns[curr][1]}")
  #       next_currs.append(ns[curr][1])
  #   nins += 1
  #   # print(f"at {next_currs} after {nins}")
  #   # if nins > 7:
  #     # break
  #   if all([x[-1] == 'Z' for x in next_currs]):
  #     end = True
  #   currs = next_currs
  #   next_currs = []
  
  # rns = defaultdict(list)
  # # 11B -> (_,11Z)
  # for (key,(l,r)) in ns.items():
  #   rns[l].append((key,0)) #rns[_] 
  #   rns[r].append((key,1)) #rns[11Z]
    
  # end = False
  # nins = 0
  # next_currs = []
  # currs = copy.deepcopy(ends)
  # while not end:
  #   cins = ins[(len(ins) - 1 - nins) % len(ins)]
  #   # print(f"currs = {currs} at {nins}")
  #   for curr in currs:
  #     if cins == 'L':
  #       next_currs.extend([t[0] for t in rns[curr] if t[1] == 0])
  #       # next_currs.append(ns[curr][0])
  #     else:
  #       # print(f"{ns[curr]}")`
  #       rts = [t[0] for t in rns[curr] if t[1] == 1]
  #       # print(f"append right: {rts}")
  #       next_currs.extend(rts)
  #       # next_currs.append(ns[curr][1])
  #   # if nins > 10:
  #     # break
  #   nins += 1
  #   if nins % 10000 == 0:
  #     print(nins)
  #   valids = [x for x in next_currs if x[-1] == 'A']
  #   # print(valids)
  #   # print(len(ends))
  #   if len(valids) == len(ends):
  #     end = True
  #   currs = next_currs
  #   next_currs = []
  # return nins


    

  return nins
    


  # to_visit = deque()
  raise Exception("not implemented!")

def main(sample, sample2, input1):
  print(f"Hello Day 08!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample2)
  print(f"Part 2 (sample2) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

