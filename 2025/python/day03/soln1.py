def parse(raw_input):
  lines = [line.strip() for line in raw_input.strip().splitlines()]
  return [[int(c) for c in line] for line in lines]


def best_digits(bank, num_digits):
  digits = []
  range_start,range_end_incl = 0,len(bank)-num_digits
  while len(digits) < num_digits:
    best_digit = max(bank[range_start:range_end_incl+1])
    best_digit_idx = bank.index(best_digit, range_start, range_end_incl+1)
    digits.append(best_digit)
    range_start, range_end_incl = best_digit_idx+1, range_end_incl+1
  return int(''.join([str(n) for n in digits]))


def part1(raw_input):
  banks = parse(raw_input)
  output_p1 = 0
  for bank in banks:
    output_p1 += best_digits(bank, 2)
  return output_p1


def part2(raw_input):
  banks = parse(raw_input)
  output_p2 = 0
  for bank in banks:
    output_p2 += best_digits(bank, 12)
  return output_p2


def main(sample, input1):
  print(f"Hello Day 03!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
