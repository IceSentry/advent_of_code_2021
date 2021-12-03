//   fn count_bit(nums: &[u32], bit: usize) -> (usize,usize) {
//     let mut c = [0,0];
//     for &x in nums {
//       c[(x as usize >> bit) & 1] += 1
//     }
//     (c[0], c[1])
//   }

//   fn part1(nums: &[u32]) -> u32 {
//     let (mut x, mut y) = (0,0);
//     for i in 0..12 {
//       let (zero, one) = count_bit(nums, i);
//       let tmp = if one > zero {&mut x} else {&mut y};
//       *tmp += 1 << i;
//     }
//     x * y
//   }

//   fn part2(nums: &[u32], a: u32, b: u32) -> u32 {
//     let mut nums = nums.to_vec();
//     for i in (0..12).rev() {
//       let (zero, one) = count_bit(&nums, i);
//       let keep = if one >= zero {a} else {b};
//       nums.retain(|x| (x>>i) & 1 == keep);
//       if nums.len() == 1 { break }
//     }
//     nums[0]
//   }

//   aoc2021::main! {
//     let input = INPUT.lines()
//       .map(|l| u32::from_str_radix(l, 2).unwrap())
//       .collect::<Vec<_>>();
//     let p1 = part1(&input);
//     let p2 = part2(&input, 1, 0) * part2(&input, 0, 1);
//     (p1,p2)
//   }

type Data = Vec<Vec<char>>;

pub fn parse(input: &str) -> Data {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn part_1(input: &Data) -> usize {
    let len = input.first().unwrap().len();
    let mut acc_0 = vec![0; len];
    let mut acc_1 = vec![0; len];
    for report in input {
        for (i, bit) in report.iter().enumerate() {
            if *bit == '0' {
                acc_0[i] += 1;
            } else {
                acc_1[i] += 1;
            }
        }
    }
    let mut gamma_rate = 0b0000_0000;
    let mut epsilon_rate = 0b0000_0000;
    for i in 0..len {
        if acc_0[i] > acc_1[i] {
            gamma_rate |= 0 << (len - 1 - i);
            epsilon_rate |= 1 << (len - 1 - i);
        } else {
            gamma_rate |= 1 << (len - 1 - i);
            epsilon_rate |= 0 << (len - 1 - i);
        }
    }
    gamma_rate * epsilon_rate
}

pub fn part_2(input: &Data) -> usize {
    let oxygen = find(input, 0, true);
    let co2 = find(input, 0, false);
    let oxygen = char_to_binary(oxygen);
    let co2 = char_to_binary(co2);

    oxygen * co2
}

fn char_to_binary(chars: Vec<char>) -> usize {
    let mut result = 0;
    for (i, &c) in chars.iter().enumerate() {
        if c == '0' {
            result |= 0 << (chars.len() - 1 - i);
        } else {
            result |= 1 << (chars.len() - 1 - i);
        }
    }
    result
}

fn find(input: &Data, i: usize, is_common: bool) -> Vec<char> {
    let mut acc_0 = 0;
    let mut acc_1 = 0;
    for report in input {
        if report[i] == '0' {
            acc_0 += 1;
        } else {
            acc_1 += 1;
        }
    }

    let common = if is_common {
        if acc_0 > acc_1 {
            '0'
        } else {
            '1'
        }
    } else if acc_0 <= acc_1 {
        '0'
    } else {
        '1'
    };

    let mut next = vec![];
    for report in input {
        if report[i] == common {
            next.push(report.clone());
        }
    }
    if next.len() > 1 {
        find(&next, i + 1, is_common)
    } else {
        next[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUTS: &str = indoc! {"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    "};

    #[test]
    pub fn part_1() {
        let input = super::parse(INPUTS);
        let result = super::part_1(&input);
        assert_eq!(result, 198);
    }

    #[test]
    pub fn part_2() {
        let input = super::parse(INPUTS);
        let result = super::part_2(&input);
        assert_eq!(result, 230);
    }
}
