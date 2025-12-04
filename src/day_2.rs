use std::{ops::RangeInclusive, str};

const PRIMES: [u32; 6] = [3, 5, 7, 9, 11, 13];

pub fn part_1(lines: &[String]) -> u64 {
    let mut result = 0;

    for range in lines[0].split(",") {
        if range.is_empty() {
            continue;
        }
        let mut skip = 0;

        for x in gen_range(range) {
            if skip > 0 {
                skip = (skip - 1).max(0);
                continue;
            }
            let n_digits = x.ilog10() + 1;
            if n_digits % 2 != 0 {
                skip = (skip - 1).max(0);
                continue;
            }
            let split = 10_u64.pow(n_digits / 2);
            if x / split == x % split {
                skip = 10_i32.pow(n_digits / 2);
                result += x;
                continue;
            }
            skip = (skip - 1).max(0);
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
        let mut pattern_repeats_in = 0;

        'number_loop: for x in gen_range(range) {
            let n_digits = x.ilog10() + 1;
            if pattern_repeats_in > 0 {
                pattern_repeats_in = (pattern_repeats_in - 1).max(0);
                continue;
            }

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
                    if (x + 1).ilog10() + 1 > n_digits {
                        pattern_repeats_in = 2;
                    } else if PRIMES.contains(&n_digits) {
                        pattern_repeats_in =
                            "1".repeat(n_digits as usize).parse::<i64>().unwrap() - 1;
                    } else {
                        pattern_repeats_in = 10 * (n_digits as i64 - 1);
                    }
                    continue 'number_loop;
                }
            }
            pattern_repeats_in = (pattern_repeats_in - 1).max(0);
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
    fn test_part_1_a() {
        let lines = vec!["11-22".to_owned()];

        assert_eq!(part_1(&lines), 33);
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
