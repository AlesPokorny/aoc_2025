struct Point(usize, usize);

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    fn rectangle_size(&self, other: &Self) -> usize {
        (self.0.abs_diff(other.0) + 1) * (self.1.abs_diff(other.1) + 1)
    }
}

pub fn part_1(lines: &[String]) -> usize {
    let points = lines
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<Point>>();

    let mut max_size = 0;

    for (i, point_a) in points.iter().enumerate() {
        for point_b in points[i + 1..points.len()].iter() {
            max_size = max_size.max(point_a.rectangle_size(point_b));
        }
    }

    max_size
}

pub fn part_2(lines: &[String]) -> usize {
    let points = lines
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point::new(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<Point>>();

    let mut sizes = Vec::new();

    for (i, point_a) in points.iter().enumerate() {
        for point_b in points[i + 1..points.len()].iter() {
            sizes.push(((point_a, point_b), point_a.rectangle_size(point_b)));
        }
    }

    sizes.sort_unstable_by_key(|(_, size)| *size);

    let n_points = points.len();

    'point_loop: for ((point_a, point_b), size) in sizes.into_iter().rev() {
        let (x_min, x_max) = get_min_max(point_a.0, point_b.0);
        let (y_min, y_max) = get_min_max(point_a.1, point_b.1);

        for i in 0..n_points {
            let j = (i + 1) % n_points;
            let (edge_a, edge_b) = (&points[i], &points[j]);
            let (edge_x_min, edge_x_max) = get_min_max(edge_a.0, edge_b.0);
            let (edge_y_min, edge_y_max) = get_min_max(edge_a.1, edge_b.1);
            if x_min < edge_x_max && x_max > edge_x_min && y_min < edge_y_max && y_max > edge_y_min
            {
                continue 'point_loop;
            }
        }
        return size;
    }

    unreachable!()
}

fn get_min_max(x: usize, y: usize) -> (usize, usize) {
    if x < y { (x, y) } else { (y, x) }
}

#[cfg(test)]
mod day_9_tests {
    use crate::day_9::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_9.txt");

        assert_eq!(part_1(&lines), 50);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_9.txt");

        assert_eq!(part_2(&lines), 24);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_9.txt");

        assert_eq!(part_1(&lines), 4741848414);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_9.txt");

        assert_eq!(part_2(&lines), 1508918480);
    }
}
