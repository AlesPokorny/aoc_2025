pub fn part_1(lines: &[String]) -> u64 {
    let mut joltage = 0;

    for line in lines {
        joltage += get_max_joltage_of_bank(line, 2);
    }

    joltage
}

pub fn part_2(lines: &[String]) -> u64 {
    let mut joltage = 0;
    for line in lines {
        joltage += get_max_joltage_of_bank(line, 12);
    }
    joltage
}

fn get_max_joltage_of_bank(bank: &str, n_batteries: usize) -> u64 {
    fix_bank(&bank.chars().collect::<Vec<char>>(), n_batteries)
        .chars()
        .enumerate()
        .map(|(index, battery)| {
            battery.to_digit(10).unwrap() as u64 * 10_u64.pow(n_batteries as u32 - index as u32 - 1)
        })
        .sum()
}

fn fix_bank(bank: &[char], n_batteries: usize) -> String {
    if n_batteries == 1 {
        return bank.iter().max().unwrap().to_string();
    }

    let first_digit = bank[..=bank.len() - n_batteries].iter().max().unwrap();
    let index = bank.iter().position(|c| c == first_digit).unwrap();

    first_digit.to_string() + &fix_bank(&bank[index + 1..], n_batteries - 1)
}

#[cfg(test)]
mod day_3_tests {
    use crate::day_3::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_3.txt");

        assert_eq!(part_1(&lines), 357);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_3.txt");

        assert_eq!(part_2(&lines), 3121910778619);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_3.txt");

        assert_eq!(part_1(&lines), 17376);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_3.txt");

        assert_eq!(part_2(&lines), 172119830406258);
    }
}
