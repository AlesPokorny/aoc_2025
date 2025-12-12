pub fn part_1(lines: &[String]) -> usize {
    let sizes = [7, 7, 7, 7, 7, 5];
    let mut count = 0;

    for (i, line) in lines.iter().enumerate() {
        if i > 29 {
            let (size, pc) = line.split_once(": ").unwrap();
            let (x, y) = size.split_once("x").unwrap();

            let total_size_ish = pc
                .split(" ")
                .zip(sizes)
                .map(|(c, size)| c.parse::<usize>().unwrap() * size)
                .sum::<usize>();
            if total_size_ish <= x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap() {
                count += 1;
            }
            continue;
        }
    }
    count
}

#[allow(unused_variables)]
pub fn part_2(lines: &[String]) -> usize {
    0
}

#[cfg(test)]
mod day_12_tests {
    use crate::day_12::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/day_12.txt");

        assert_eq!(part_1(&lines), 497);
    }
}
