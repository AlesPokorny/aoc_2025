use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
    z: usize,
}

impl Pos {
    fn from_vec(coordinates: &[usize]) -> Self {
        Self {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
        }
    }

    fn get_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn from_n(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            let v = self.find(self.parent[i]);
            self.parent[i] = v;
            return v;
        }
        i
    }

    fn union(&mut self, i_1: usize, i_2: usize) -> bool {
        let root_1 = self.find(i_1);
        let root_2 = self.find(i_2);
        if root_1 != root_2 {
            self.parent[root_1] = root_2;
            return true;
        }
        false
    }
}

fn get_distances(boxes: &[Pos]) -> Vec<(usize, usize, usize)> {
    let mut distances: Vec<(usize, usize, usize)> = Vec::with_capacity(boxes.len());
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            distances.push((i, j, boxes[i].get_distance(&boxes[j])));
        }
    }
    distances.sort_unstable_by_key(|(_, _, d)| *d);
    distances
}

pub fn part_1(lines: &[String]) -> usize {
    let boxes = lines
        .iter()
        .map(|s| {
            let coordinates = s
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Pos::from_vec(&coordinates)
        })
        .collect::<Vec<Pos>>();

    let distances = get_distances(&boxes);
    let mut uf = UnionFind::from_n(boxes.len());
    let n_runs = if boxes.len() == 20 { 10 } else { 1000 };

    for (i_1, i_2, _) in distances.into_iter().take(n_runs) {
        uf.union(i_1, i_2);
    }

    let mut sizes = vec![0; boxes.len()];
    for (i, _) in boxes.iter().enumerate() {
        let i_2 = uf.find(i);
        sizes[i_2] += 1;
    }

    sizes.iter().sorted().rev().take(3).product()
}

pub fn part_2(lines: &[String]) -> usize {
    let boxes = lines
        .iter()
        .map(|s| {
            let coordinates = s
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Pos::from_vec(&coordinates)
        })
        .collect::<Vec<Pos>>();

    let distances = get_distances(&boxes);
    let mut uf = UnionFind::from_n(boxes.len());
    let mut c = boxes.len();

    for (i_1, i_2, _) in distances.into_iter() {
        if uf.union(i_1, i_2) {
            c -= 1;
            if c == 1 {
                return boxes[i_1].x * boxes[i_2].x;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod day_8_tests {
    use crate::day_8::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_8.txt");

        assert_eq!(part_1(&lines), 40);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_8.txt");

        assert_eq!(part_2(&lines), 25272);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_8.txt");

        assert_eq!(part_1(&lines), 175440);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_8.txt");

        assert_eq!(part_2(&lines), 3200955921);
    }
}
