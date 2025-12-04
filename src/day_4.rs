const DIRECTIONS: [[i32; 2]; 8] = [
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
    [0, -1],
];

pub fn part_1(lines: &[String]) -> u64 {
    let mut accessible_rolls = 0;
    let mut room = Vec::with_capacity(lines.len());
    for line in lines {
        room.push(line.chars().collect::<Vec<char>>());
    }

    let x_max = room[0].len() as i32;
    let y_max = room.len() as i32;
    for row in 0..room.len() {
        'col_loop: for col in 0..room[0].len() {
            if room[row][col] == '.' {
                continue;
            }
            let mut n_rolls = 0;
            for direction in DIRECTIONS {
                let x = col as i32 + direction[0];
                let y = row as i32 + direction[1];

                if !(0..x_max).contains(&x) || !(0..y_max).contains(&y) {
                    continue;
                }
                if room[y as usize][x as usize] == '@' {
                    n_rolls += 1;
                }
                if n_rolls >= 4 {
                    continue 'col_loop;
                }
            }
            if n_rolls < 4 {
                accessible_rolls += 1;
            }
        }
    }
    accessible_rolls
}

// not as pretty as part 1 but it is fast
pub fn part_2(lines: &[String]) -> u64 {
    let mut accessible_rolls = 0;
    let mut room = Vec::with_capacity(lines.len());
    for line in lines {
        room.push(line.chars().collect::<Vec<char>>());
    }

    let x_max = room[0].len() as i32;
    let y_max = room.len() as i32;
    let mut row = 0;
    let mut col = 0;
    'outer_boii: loop {
        row = (row - 1).max(0);
        col = (col - 1).max(0);
        #[allow(unused_labels)]
        'row_loop: loop {
            'col_loop: loop {
                if room[row as usize][col as usize] == '.' {
                    col += 1;
                    if col == x_max {
                        break 'col_loop;
                    }
                    continue 'col_loop;
                }
                let mut n_rolls = 0;
                for direction in DIRECTIONS {
                    let x = col + direction[0];
                    let y = row + direction[1];

                    if !(0..x_max).contains(&x) || !(0..y_max).contains(&y) {
                        continue;
                    }
                    if room[y as usize][x as usize] == '@' {
                        n_rolls += 1;
                    }
                    if n_rolls >= 4 {
                        col += 1;
                        if col == x_max {
                            break 'col_loop;
                        }
                        continue 'col_loop;
                    }
                }
                if n_rolls < 4 {
                    accessible_rolls += 1;
                    room[row as usize][col as usize] = '.';
                    continue 'outer_boii;
                }

                col += 1;
                if col == x_max {
                    break 'col_loop;
                }
            }

            row += 1;
            col = 0;
            if row == y_max {
                break 'outer_boii;
            }
        }
    }
    accessible_rolls
}

#[cfg(test)]
mod day_4_tests {
    use crate::day_4::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_4.txt");

        assert_eq!(part_1(&lines), 13);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_4.txt");

        assert_eq!(part_2(&lines), 43);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_4.txt");

        assert_eq!(part_1(&lines), 1449);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_4.txt");

        assert_eq!(part_2(&lines), 8746);
    }
}
