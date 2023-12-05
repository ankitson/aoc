import collections
def parse(raw_input):
  raise Exception("not implemented!")

def part1(input):
  lines = [line for line in input.split("\n") if len(line) > 1]
  seeds = [int(x) for x in lines[0].split(': ')[1].split(' ')]
  maps = [{},{},{},{},{},{},{},{}]
  nmaps = []
  mi = 0
  curr_map = []
  for line in lines[3:]:
    # print(line)
    if len(line) < 2:
      continue

    print(line)
    if line[0].isdigit():
      parts = [int(x) for x in line.split(' ')]
      dest = parts[0]
      source = parts[1]
      rnglen = parts[2]
      curr_map.append((source,dest,rnglen))
      # for i in range(rnglen):
        # maps[mi][source+i] = dest+i
    else:
      nmaps.append(curr_map)
      curr_map = []
      mi += 1
  if len(curr_map) > 0:
    nmaps.append(curr_map)

  print(nmaps)
  #[1,2,3,4,5]
  idxs = []
  for seed in seeds:
    # print(f"Starting seed {seed}")
    curr = seed
    for mi in range(len(nmaps)):
      print(f"\tcurr: {curr}")
      mapped = False
      for (src,dest,rnglen) in nmaps[mi]:
        srcoff = curr-src
        print(f"\tfor range ({src} -> {dest}, {rnglen}) offset = {srcoff}")
        if srcoff < 0:
          continue
        elif srcoff > rnglen:
          continue
        elif srcoff < rnglen and srcoff >= 0:
          mapped = True
          next = dest+srcoff
          break
      if not mapped:
        next = curr
      
      print(f"\tmaps to {next}")

      
      
      curr = next
      
    print(f"Seed {seed} maps to {curr}")
    idxs.append(curr)
  return min(idxs)

def part2(input):
  lines = [line for line in input.split("\n") if len(line) > 1]
  seed_ranges = [int(x) for x in lines[0].split(': ')[1].split(' ')]
  print(f"ranges {seed_ranges}")
  # seeds = []
  # for i in range(0,len(seed_ranges)-1,2):
    # s1 = seed_ranges[i]
    # sl = seed_ranges[i+1]
    # for i in range(sl):
      # seeds.append(s1+i)
    # print(seeds)
  print("here")
  nmaps = []
  mi = 0
  curr_map = []
  for line in lines[3:]:
    # print(line)
    if len(line) < 2:
      continue

    # print(line)
    if line[0].isdigit():
      parts = [int(x) for x in line.split(' ')]
      dest = parts[0]
      source = parts[1]
      rnglen = parts[2]
      curr_map.append((source,dest,rnglen))
      # for i in range(rnglen):
        # maps[mi][source+i] = dest+i
    else:
      nmaps.append(curr_map)
      curr_map = []
      mi += 1
  if len(curr_map) > 0:
    nmaps.append(curr_map)

  print(nmaps)
  #[1,2,3,4,5]
  min_idx = float('inf')
  memos = [{},{},{},{},{},{},{}]
  for i in range(0,len(seed_ranges)-1,2):
    start = seed_ranges[i]
    le = seed_ranges[i+1]
    for seed in range(start,start+le+1):
      dist_from_start = seed - start
      curr = seed
      for mi in range(len(nmaps)):
        mapped = False
        for (src,dest,rnglen) in nmaps[mi]:
          srcoff = curr-src
          # print(f"\tfor range ({src} -> {dest}, {rnglen}) offset = {srcoff}")
          if srcoff < 0:
            continue
          elif srcoff > rnglen:
            continue
          elif srcoff < rnglen and srcoff >= 0:
            mapped = True
            next = dest+srcoff
            break
        if not mapped:
          next = curr
        memos[mi][curr] = next
        curr = next
      min_idx = min(min_idx,curr)
  return min_idx

def main(sample, input1):
  print(f"Hello Day 05!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")

