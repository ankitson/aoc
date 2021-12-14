pub struct Soln1 {}
impl Soln1 {
    pub fn parse(input: &str) -> (impl Iterator<Item = u32> + '_, u16) {
        let lines = input
            .split('\n')
            .filter(|x| !x.is_empty())
            .map(|x| u32::from_str_radix(x, 2).expect("illegal int ahah"));

        (lines, 12)
    }

    pub fn unparse(output: (u16, u16)) -> String {
        (u32::from(output.0) * u32::from(output.1)).to_string()
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
        let most_common = match c1.cmp(&c0) {
            std::cmp::Ordering::Greater => Some(1),
            std::cmp::Ordering::Less => Some(0),
            std::cmp::Ordering::Equal => None,
        };
        let least_common = most_common.map(|x| 1 - x);
        (most_common, least_common)
    }

    pub fn part1(nums: &[u32], bit_width: u16) -> (u16, u16) {
        let mut gamma: u16 = 0; //most common
        for bi in 0..bit_width {
            let mcb = Self::most_least_common(nums, bi.into()).0.unwrap();
            gamma |= mcb << bi;
        }
        let mask = (1 << bit_width) - 1; //0b111111111111;
        let epsilon = (!gamma) & mask;
        (gamma, epsilon)
    }

    pub fn part2(nums: &Vec<u32>, bit_width: u16) -> (u16, u16) {
        let mut nums1 = nums.clone();
        let mut nums2 = nums.clone();
        let mut bi: i32 = (bit_width - 1).into();
        let mut mask = 1 << bi;
        while nums1.len() > 1 || nums2.len() > 1 {
            let mlc1 = Self::most_least_common(&nums1, bi as u32);
            let mlc2 = Self::most_least_common(&nums2, bi as u32);
            let mc = mlc1.0.unwrap_or(1);
            let lc = mlc2.1.unwrap_or(0);

            if nums1.len() > 1 {
                nums1.retain(|x| (*x & mask) >> bi == (mc as u32));
            }

            if nums2.len() > 1 {
                nums2.retain(|x| (*x & mask) >> bi == (lc as u32));
            }
            mask >>= 1;
            bi -= 1;
        }
        assert!(nums1.len() == 1 && nums2.len() == 1, "{:?} {:?}", nums1, nums2);
        let oxygen = nums1[0].try_into().unwrap();
        let co2 = nums2[0].try_into().unwrap();
        (oxygen, co2)
    }
}
