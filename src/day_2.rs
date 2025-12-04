use std::{ops::RangeInclusive, str};

pub fn part_1(lines: &[String]) -> u64 {
    let mut result = 0;

    for range in lines[0].split(",") {
        if range.is_empty() {
            continue;
        }

        for x in gen_range(range) {
            let n_digits = x.ilog10() + 1;
            if n_digits % 2 != 0 {
                continue;
            }
            let split = 10_u64.pow(n_digits / 2);
            if x / split == x % split {
                result += x;
            }
        }
    }

    result
}

pub fn part_2(lines: &[String]) -> u64 {
    let mut result = 0;

    for range in lines[0].split(",") {
        if range.is_empty() {
            continue;
        }

        'number_loop: for x in gen_range(range) {
            let n_digits = x.ilog10() + 1;

            for i in 1..=(n_digits / 2) {
                if n_digits % i != 0 {
                    continue;
                }
                let pow = 10_u64.pow(i);
                let n_reps = n_digits / i;
                let last_digits = x % pow;

                let mut value = last_digits;
                for rep in 1..n_reps {
                    value += last_digits * pow.pow(rep);
                }
                if value == x {
                    result += x;
                    continue 'number_loop;
                }
            }
        }
    }

    result
}

fn gen_range(range_str: &str) -> RangeInclusive<u64> {
    let (left, right) = range_str.split_once("-").unwrap();

    let start = left.parse::<u64>().unwrap();
    let end = right.parse::<u64>().unwrap();

    start..=end
}

#[cfg(test)]
mod day_2_tests {
    use crate::day_2::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_2.txt");

        assert_eq!(part_1(&lines), 1227775554);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_2.txt");

        assert_eq!(part_2(&lines), 4174379265);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_2.txt");

        assert_eq!(part_1(&lines), 38310256125);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_2.txt");

        assert_eq!(part_2(&lines), 58961152806);
    }
}
