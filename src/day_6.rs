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
        match operator {
            "+" => result += values.iter().map(|v| v[i]).sum::<u64>(),
            "*" => {
                let mut temp_result = 1;
                for v in values.iter() {
                    temp_result *= v[i];
                }
                result += temp_result;
            }
            _ => panic!("invalid operator"),
        }
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
    let mut operator = ' ';
    let mut values = Vec::new();
    for number_chars in transposed {
        if number_chars.iter().all(|x| x == &' ') {
            result += do_operation(&values, operator);
            values.clear();
            operator = ' ';
            continue;
        }
        let x = number_chars[..rows - 1]
            .iter()
            .collect::<String>()
            .trim()
            .to_owned();
        if !x.is_empty() {
            values.push(x.parse::<u64>().unwrap());
            if number_chars[rows - 1] != ' ' {
                operator = number_chars[rows - 1];
            }
        }
    }
    result += do_operation(&values, operator);

    result
}

fn do_operation(values: &[u64], operator: char) -> u64 {
    match operator {
        '+' => values.iter().sum::<u64>(),
        '*' => {
            let mut temp_result = 1;
            for v in values.iter() {
                temp_result *= v;
            }
            temp_result
        }
        _ => panic!("invalid operator"),
    }
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
