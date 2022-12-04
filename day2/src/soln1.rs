pub struct Soln1 {}
impl Soln1 {
  pub fn parse(input: &str) -> Vec<(char, char)> {
    let mut pairs = Vec::new();
  
    for line in input.lines() {
  
      if line.is_empty() {
        continue;
      }
      let mut chars = line.chars();
      let key = chars.next().unwrap();
			chars.next();
      let value = chars.next().unwrap();
      pairs.push((key, value));
    }
  
    pairs
  }

  pub fn part1(input: &str) -> u32 {
    let inputs = Self::parse(input);
		let mut score = 0;
    for (opponent,mine) in inputs {
			println!("opponent: {} mine: {}", opponent, mine);
      let score1 = match mine {'X' => 1, 'Y' => 2, 'Z' => 3, _ => panic!("no") };
      let score2 = match Self::winner(opponent, mine, false) { -1 => 0, 1 => 6, 0 => 3, _ => panic!("no") };
			score += score1 + score2;
    }
		score
  }

  fn winner(p1: char, p2: char, invert: bool) -> i32 {
		let mapped_p1 = (p1 as i32) - { if (!invert) { ('A' as i32) } else { 'X' as i32 } };
    let mapped_p2 = (p2 as i32) - { if (!invert) { 'X' as i32 } else { 'A' as i32 } };
		println!("opp: {} mine: {}", mapped_p1, mapped_p2);
		if (mapped_p1 > mapped_p2) { return -1 * Self::winner(p2,p1, !invert); }
		let score = match (mapped_p1, mapped_p2) {
			(0,0) => 0, (1,1) => 0, (2,2) => 0,
			(0,1) => 1, (0,2) => -1,
			(1,2) => 1,
			//(1,0) => 1, (2,0) => -1, (2,1) => 1,
			_     => panic!("no")
		};
		fn rev(i: i32) -> String {
			match i { 0 => "rock".to_string(), 1 => "paper".to_string(), 2 => "scissor".to_string(), _ => panic!("no") }
		}


		println!("p1 = {} p2 = {} mp1 = {} mp2 = {} {:?} {:?} {}", p1, p2, mapped_p1, mapped_p2, rev(mapped_p1), rev(mapped_p2), score);
			
		score
  }
}
