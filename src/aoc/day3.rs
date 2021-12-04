pub fn input() -> (impl Iterator<Item = u32>, u16) {
    let contents: &str = include_str!("../../input/day3.txt");
    let lines = contents
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| u32::from_str_radix(x, 2).expect("illegal int ahah"));

    (lines, 12)
}

pub fn widen_mul(a: u16, b: u16) -> u32 {
    let mut result: u32 = 0;
    let mut ac = a;
    while ac > 0 {
        result += b as u32;
        ac -= 1;
    }
    result
}

fn most_least_common(nums: &[u32], bit_index: u32) -> (Option<u16>, Option<u16>) {
    let mut c1 = 0;
    let mut c0 = 0;
    for num in nums {
        let bit = (num >> bit_index) & 1;
        if bit == 1 {
            c1 += 1
        } else {
            c0 += 1
        }
    }
    let most_common = {
        if c1 > c0 {
            Some(1)
        } else if c0 > c1 {
            Some(0)
        } else {
            None
        }
    };
    let least_common = most_common.map(|x| 1 - x);
    (most_common, least_common)
}
pub fn part1(nums: &Vec<u32>, bit_width: u16) -> (u16, u16) {
    let mut gamma: u16 = 0; //most common
    for bi in 0..bit_width {
        let mcb = most_least_common(nums, bi.into()).0.unwrap();
        gamma |= mcb << bi;
    }
    let mask = 0b111111111111;
    let epsilon = (!gamma) & mask;
    (gamma, epsilon)
    //println!("gamma = {} epsilon = {}", gamma, epsilon);
    //let prod = widen_mul(gamma, epsilon);
    //format!("{}", prod)
}

pub fn part2(nums: &Vec<u32>, bit_width: u16) -> (u16, u16) {
    let mut nums1 = nums.clone();
    let mut nums2 = nums.clone();
    let mut bi: i32 = (bit_width - 1).into();
    let mut mask = 1 << bi;
    while nums1.len() > 1 || nums2.len() > 1 {
        let mlc = most_least_common(&nums1, bi as u32);
        let mc = mlc.0.unwrap_or(1);
        let lc = mlc.1.unwrap_or(0);

        // if (bi <= 4) {
        //     println!("bit {} most common {} least common {}", bi, mc, lc);
        //     println!("nums1");
        //     println!("---------------");
        //     println!(
        //         //"nums1: {:?}",
        //         "{}",
        //         nums1
        //             .iter()
        //             .map(|x| format!("{:#014b}", x))
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //     );
        //     println!("nums2");
        //     println!("---------------");
        //     println!(
        //         //"nums2: {:?}",
        //         "{}",
        //         nums2
        //             .iter()
        //             .map(|x| format!("{:#014b}", x))
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //     );
        //     println!("---------------");
        // }

        //println!("nums2: {:?}", nums)

        if (nums.len() > 1) {
            nums1.retain(|x| (*x & mask) >> bi == (mc as u32));
        }

        if (nums2.len() > 1) {
            nums2.retain(|x| (*x & mask) >> bi == (lc as u32));
        }
        mask >>= 1;
        bi -= 1;
    }
    assert!(nums1.len() == 1 && nums2.len() == 1);
    let oxygen = nums1[0].try_into().unwrap();
    let co2 = nums2[0].try_into().unwrap();
    (oxygen, co2)

    // println!("{:#014b}     - oxygen", nums1[0]);
    // println!("{:#014b}     - co2", nums2[0]);
    // println!("oxygen = {} co2 = {}", nums1[0], nums2[0]);
    // format!(
    // "{}",
    // widen_mul(nums1[0].try_into().unwrap(), nums2[0].try_into().unwrap())
    // )
}
