def parse(raw_input):
  lines = [line for line in raw_input.split("\n") if len(line) > 1]
  return lines

def part1(input):
  parsed = parse(input)
  print(f"Parsed P1 = {parsed}")
  raise Exception("not implemented!")

def part2(input):
  raise Exception("not implemented!")

def main(sample, input1):
  print(f"Hello Day 15!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
