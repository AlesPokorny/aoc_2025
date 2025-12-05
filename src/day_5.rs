use std::ops::RangeInclusive;

pub fn part_1(lines: &[String]) -> u64 {
    let mut fresh = 0;

    let mut is_range = true;
    let mut ranges = Vec::new();

    for line in lines {
        if line.is_empty() {
            is_range = false;
            continue;
        }

        if is_range {
            let (left, right) = line.split_once("-").unwrap();
            ranges.push(left.parse::<u64>().unwrap()..=right.parse::<u64>().unwrap());
            continue;
        }

        let id = line.parse::<u64>().unwrap();
        for range in ranges.iter() {
            if range.contains(&id) {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

pub fn part_2(lines: &[String]) -> u64 {
    let mut ranges = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }

        let (left, right) = line.split_once("-").unwrap();

        let start = left.parse::<u64>().unwrap();
        let end = right.parse::<u64>().unwrap();
        ranges.push((start, end));
        continue;
    }
    ranges.sort_by_key(|range| range.0);
    let mut n_ids = 0;
    let mut last_end = 0;

    for (new_start, new_end) in ranges.into_iter() {
        if new_start > last_end {
            n_ids += new_end - new_start + 1;
            last_end = new_end;
        } else if new_end > last_end {
            n_ids += new_end - last_end;
            last_end = new_end;
        }
    }

    n_ids
}


#[cfg(test)]
mod day_5_tests {
    use crate::day_5::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_5.txt");

        assert_eq!(part_1(&lines), 3);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_5.txt");

        assert_eq!(part_2(&lines), 14);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_5.txt");

        assert_eq!(part_1(&lines), 652);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_5.txt");

        assert_eq!(part_2(&lines), 341753674214273);
    }
}
