#![allow(unused_imports)]
#![allow(dead_code)]

use crate::utilities::read_lines;

pub mod utilities;

use aoc_2025::day_1;
use aoc_2025::day_2;
use aoc_2025::day_3;
use aoc_2025::day_4;
use aoc_2025::day_5;

fn main() {
    let lines = read_lines("inputs/day_5.txt");

    println!("part 1 {}", day_5::part_1(&lines));
    println!("part 2 {}", day_5::part_2(&lines));
}
