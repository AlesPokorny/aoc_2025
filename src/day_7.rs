use std::collections::{HashMap, HashSet};

pub fn part_1(lines: &[String]) -> u64 {
    let start = lines[0].chars().position(|c| c == 'S').unwrap();
    let mut beam_positions = HashSet::new();
    beam_positions.insert(start);

    let mut n_splits = 0;

    for line in lines[1..].iter() {
        for (i, field) in line.chars().enumerate() {
            if field == '.' {
                continue;
            }
            if beam_positions.contains(&i) {
                beam_positions.insert(i - 1);
                beam_positions.insert(i + 1);
                beam_positions.remove(&i);
                n_splits += 1;
            }
        }
    }

    n_splits
}

pub fn part_2(lines: &[String]) -> u64 {
    let start = lines[0].chars().position(|c| c == 'S').unwrap();
    let mut beam_positions = HashMap::new();
    beam_positions.insert(start, 1);

    for line in lines[1..].iter() {
        for (i, field) in line.chars().enumerate() {
            if field == '.' {
                continue;
            }
            let n_new_beams = beam_positions.get(&i);
            if let Some(&beams) = n_new_beams {
                *beam_positions.entry(i - 1).or_insert(0) += beams;
                *beam_positions.entry(i + 1).or_insert(0) += beams;
                beam_positions.remove(&i);
            }
        }
    }

    beam_positions.values().sum::<usize>() as u64
}

#[cfg(test)]
mod day_7_tests {
    use crate::day_7::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_7.txt");

        assert_eq!(part_1(&lines), 21);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_7.txt");

        assert_eq!(part_2(&lines), 40);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_7.txt");

        assert_eq!(part_1(&lines), 1640);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_7.txt");

        assert_eq!(part_2(&lines), 40999072541589);
    }
}
