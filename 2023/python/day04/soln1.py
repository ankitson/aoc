from collections import defaultdict

# Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
# Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
# Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
# Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
# Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
# Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
def parse(raw_input):
  lines = [line for line in raw_input.split("\n") if len(line) > 1]
  cards = []
  for line in lines:
    parts1 = line.split(":")
    nums = parts1[1]
    parts2 = nums.split("|")
    winning = [int(x.strip()) for x in parts2[0].split(" ") if len(x) > 0]
    have = [int(x.strip()) for x in parts2[1].split(" ") if len(x) > 0]
    cards.append((winning,have))
  return cards

def part1(input):
  cards = parse(input)
  points = 0  
  for (winning, have) in cards:
    card_points = 0
    for havec in have:
      if havec in winning:
        if card_points == 0:
          card_points = 1
        else:
          card_points *= 2
    points += card_points
  return points

def part2(input):
  cards = parse(input)
  card_counts = defaultdict(int) #1 - 180
  for i in range(0,len(cards)):
    card_counts[i] = 1
  
  for card_num, count in card_counts.items():
    (winning, have) = cards[card_num]
    num_match = len(set(winning) & set(have)) #Card 0 has 4 matches
    for i in range(num_match):
      card_counts[card_num+i+1] += count #Card 1,2,3,4
  return sum(card_counts.values())

def main(sample, input1):
  print(f"Hello Day 04!")

  soln = part1(sample)
  print(f"Part 1 (sample) = {soln}")
  assert(soln == 13)

  soln = part1(input1)
  print(f"Part 1 (realinput) = {soln}")
  assert(soln == 19135)

  soln = part2(sample)
  print(f"Part 2 (sample) = {soln}")
  assert(soln == 30)

  soln = part2(input1)
  print(f"Part 2 (realinput) = {soln}")
  assert(soln == 5704953)

