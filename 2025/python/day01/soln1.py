def parse(raw_input):
  return [int(line.replace("L", "-").replace("R", "+")) for line in raw_input.strip().splitlines()]


def part1(raw_input):
  input_data = parse(raw_input)
  pos = 50
  num_zeros = 0
  for offset in input_data:
    pos = (pos + offset) % 100
    if pos == 0:
      num_zeros += 1
  return num_zeros


def part2(raw_input):
  input_data = parse(raw_input)
  num_zeros = 0
  pos = 50
  for offset in input_data:
    if offset < 0:
      num_zeros += abs(offset) // 100
      if pos != 0 and abs(offset) % 100 >= pos:
        num_zeros += 1
      pos = (pos + offset) % 100
    else:
      pos += offset
      num_zeros += pos // 100
      pos = pos % 100
  return num_zeros


def main(sample, input1):
  print(f"Hello Day 01!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
