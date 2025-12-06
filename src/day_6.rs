// TODO: Clean both parts up a bit and speed it up

pub fn part_1(lines: &[String]) -> u64 {
    let mut result = 0;
    let mut values = Vec::new();
    let mut operators = Vec::new();
    for line in lines {
        let trimmed = line.trim();
        if trimmed.chars().next().unwrap().is_ascii_digit() {
            values.push(
                line.split(' ')
                    .filter_map(|s| {
                        if !s.is_empty() {
                            Some(s.parse::<u64>().unwrap())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<u64>>(),
            );
            continue;
        }
        operators = line
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
    }
    for (i, operator) in operators.into_iter().enumerate() {
        if operator == "+" {
            result += values.iter().map(|v| v[i]).sum::<u64>();
            continue;
        }
        let mut temp_result = 1;
        for v in values.iter() {
            temp_result *= v[i];
        }
        result += temp_result;
    }

    result
}

pub fn part_2(lines: &[String]) -> u64 {
    let rows = lines.len();
    let cols = lines[0].len();
    let transposed: Vec<Vec<char>> = (0..cols)
        .map(|col| {
            (0..rows)
                .map(|row| lines[row].chars().nth(col).unwrap())
                .collect()
        })
        .collect();

    let mut result = 0;
    let mut values = Vec::new();
    let mut operators = Vec::new();
    let mut temp_values = Vec::new();
    for number_chars in transposed {
        if number_chars.iter().all(|x| x == &' ') {
            values.push(temp_values);
            temp_values = Vec::new();
            continue;
        }
        let x = number_chars[..rows - 1]
            .iter()
            .collect::<String>()
            .trim()
            .to_owned();
        if !x.is_empty() {
            temp_values.push(x.parse::<u64>().unwrap());
            if number_chars[rows - 1] != ' ' {
                operators.push(number_chars[rows - 1]);
            }
        }
    }
    values.push(temp_values);
    for (i, operator) in operators.into_iter().enumerate() {
        if operator == '+' {
            result += values[i].iter().sum::<u64>();
            continue;
        }
        let mut temp_result = 1;
        for v in values[i].iter() {
            temp_result *= v;
        }
        result += temp_result;
    }

    result
}

#[cfg(test)]
mod day_6_tests {
    use crate::day_6::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_6.txt");

        assert_eq!(part_1(&lines), 4277556);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_6.txt");

        assert_eq!(part_2(&lines), 3263827);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_6.txt");

        assert_eq!(part_1(&lines), 6295830249262);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_6.txt");

        assert_eq!(part_2(&lines), 9194682052782);
    }
}
