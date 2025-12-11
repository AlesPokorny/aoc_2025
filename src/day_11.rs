use std::collections::HashMap;

pub fn part_1(lines: &[String]) -> usize {
    let mut nodes = HashMap::new();
    for line in lines {
        let (start_node, rest) = line.split_once(": ").unwrap();
        let ends = rest.split(' ').collect::<Vec<&str>>();
        nodes.insert(start_node, ends);
    }

    find_paths("you", "out", &nodes, &mut HashMap::new())
}

pub fn part_2(lines: &[String]) -> usize {
    let mut nodes: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in lines {
        let (start_node, rest) = line.split_once(": ").unwrap();
        let ends = rest.split(' ').collect::<Vec<&str>>();
        nodes.insert(start_node, ends);
    }

    let dac_fft = find_paths("dac", "fft", &nodes, &mut HashMap::new());
    let fft_dac = find_paths("fft", "dac", &nodes, &mut HashMap::new());
    let svr_dac = find_paths("svr", "dac", &nodes, &mut HashMap::new());
    let svr_fft = find_paths("svr", "fft", &nodes, &mut HashMap::new());
    let fft_out = find_paths("fft", "out", &nodes, &mut HashMap::new());
    let dac_out = find_paths("dac", "out", &nodes, &mut HashMap::new());

    svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out
}

fn find_paths<'a>(
    from: &'a str,
    to: &'a str,
    map: &'a HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if from == to {
        return 1;
    }
    if let Some(n_paths) = cache.get(from) {
        return *n_paths;
    }

    let nodes = match map.get(from) {
        Some(n) => n,
        _ => return 0,
    };

    let paths = nodes
        .iter()
        .map(|node| find_paths(node, to, map, cache))
        .sum::<usize>();

    cache.insert(from, paths);

    paths
}

#[cfg(test)]
mod day_11_tests {
    use crate::day_11::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = vec![
            "aaa: you hhh".to_owned(),
            "you: bbb ccc".to_owned(),
            "bbb: ddd eee".to_owned(),
            "ccc: ddd eee fff".to_owned(),
            "ddd: ggg".to_owned(),
            "eee: out".to_owned(),
            "fff: out".to_owned(),
            "ggg: out".to_owned(),
            "hhh: ccc fff iii".to_owned(),
            "iii: out".to_owned(),
        ];

        assert_eq!(part_1(&lines), 5);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_11.txt");

        assert_eq!(part_2(&lines), 2);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_11.txt");

        assert_eq!(part_1(&lines), 500);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_11.txt");

        assert_eq!(part_2(&lines), 287039700129600);
    }
}
