use std::ops::Mul;

pub struct Soln1 {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SquareMatrix<const N: usize>(pub [[u8; N]; N]);

//PROBLEM: ownership woes
impl<const N: usize> SquareMatrix<N> {
    fn mul(lhs: &SquareMatrix<N>, rhs: &SquareMatrix<N>) -> Self {
        let mut result = SquareMatrix([[0; N]; N]);
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result.0[i][j] += lhs.0[i][k] * rhs.0[k][j];
                }
            }
        }
        result
    }

    fn exponent(m: &SquareMatrix<N>, p: u8) -> Self {
        if p == 1 {
            *m
        } else {
            let exp = SquareMatrix::exponent(m, p / 2);
            if (p % 2 == 1) {
                SquareMatrix::mul(&SquareMatrix::mul(&exp, &exp), m)
            } else {
                SquareMatrix::mul(&exp, &exp)
            }
        }
    }
}

impl Soln1 {
    const matrix: SquareMatrix<9> = SquareMatrix([
        [0, 0, 0, 0, 0, 0, 1, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 1, 0],
    ]);
    // const N: u8 = 9;
    // fn bla(a: [u8; N]) -> [u8; N] {
    // a
    // }

    pub fn part1(input: &str, days: usize) -> usize {
        let fish = input.trim().split(',').map(|x| x.parse::<u8>().expect("illegal age"));
        let mut fish = fish.collect::<Vec<u8>>();
        for _ in 0..days {
            Self::advance(&mut fish);
        }
        // println!("Final state: {:?}", fish);
        fish.len()
    }

    pub fn part2(input: &str, days: usize) -> u64 {
        let fish = input.trim().split(',').map(|x| x.parse::<u8>().expect("illegal age"));
        let mut by_age: [u64; 9] = [0; 9];
        for age in fish {
            by_age[usize::from(age)] += 1
        }

        let before = by_age;
        for _ in 0..days {
            by_age[7] += by_age[0];
            by_age.rotate_left(1);
            let after = by_age;
        }
        by_age.iter().sum()
    }

    // fn mat_multiply(a: [[u8; N]; N], b: [[u8; N]; N]) -> [[u8; N]; N] {
    // a
    // }

    fn advance(fish: &mut Vec<u8>) -> () {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] -= 1;
            }
        }
    }
}
