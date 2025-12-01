pub fn solve(lines: &Vec<String>) -> (i32, i32) {
    let mut dial = 50;
    let mut pass_1 = 0;
    let mut rot = 0;

    for line in lines {
        let (direction, clicks) = line.split_at(1);
        let mut clicks = clicks.parse::<i32>().unwrap();

        if direction == "L" {
            clicks = -clicks;
        }

        dial += clicks;
        rot += dial.abs() / 100;
        if dial <= 0 && dial != clicks {
            rot += 1;
        }

        dial = dial.rem_euclid(100);

        if dial == 0 {
            pass_1 += 1;
        }
    }
    (pass_1, rot)
}

#[cfg(test)]
mod day_1_tests {
    use crate::day_1::solve;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_1.txt");

        assert_eq!(solve(&lines).0, 3);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_1.txt");

        assert_eq!(solve(&lines).1, 6);
    }

    #[test]
    fn test_solutions() {
        let lines = read_lines("inputs/day_1.txt");

        assert_eq!(solve(&lines), (1018, 5815));
    }
}
