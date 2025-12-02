use std::{ops::RangeInclusive, str};

pub fn part_1(lines: &[String]) -> u64 {
    let mut result = 0;

    for range in lines[0].trim().split(",") {
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

    for range in lines[0].trim().split(",") {
        if range.is_empty() {
            continue;
        }

        'number_loop: for x in gen_range(range){
            let n_digits = x.ilog10() + 1;
            let mut digits_vec: Vec<u8> = Vec::with_capacity(n_digits as usize);

            let mut temp_x = x;
            while temp_x > 0 {
                digits_vec.push((temp_x % 10) as u8);
                temp_x /= 10;
            }

            for i in (1..=(n_digits / 2)).rev() {
                if n_digits % i != 0 {
                    continue;
                }

                let chunks = digits_vec.chunks(i as usize).collect::<Vec<&[u8]>>();
                let first = chunks[0];
                if chunks.into_iter().any(|el| el != first) {
                    continue;
                }
                result += x;
                continue 'number_loop;
            }
        }
    }

    result
}

fn gen_range(range_str: &str) -> RangeInclusive<u64>{
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
