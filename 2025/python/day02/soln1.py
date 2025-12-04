def parse(raw_input):
  line = raw_input.strip()
  ranges = line.split(',')
  result = []
  for range_str in ranges:
    parts = range_str.split('-')
    start = int(parts[0])
    end = int(parts[1])
    result.append((start, end))
  return result


def is_repeat(strin):
  for repeat_len in range(1,len(strin)//2+1):
    seq = strin[0:repeat_len]
    window_start = 0
    this_win = True
    while window_start < len(strin):
      if strin[window_start:window_start+repeat_len] != seq:
        this_win = False
        break
      window_start += repeat_len
    if this_win:
      return True
  return False


def part1(raw_input):
  ranges = parse(raw_input)
  count_p1 = 0
  for (start, end) in ranges:
    for i in range(start, end + 1):
      str_i = str(i)
      if str_i[0:len(str_i)//2] == str_i[len(str_i)//2:]:
        count_p1 += i
  return count_p1


def part2(raw_input):
  ranges = parse(raw_input)
  count_p2 = 0
  for (start, end) in ranges:
    for i in range(start, end + 1):
      str_i = str(i)
      if is_repeat(str_i):
        count_p2 += i
  return count_p2


def main(sample, input1):
  print(f"Hello Day 02!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
