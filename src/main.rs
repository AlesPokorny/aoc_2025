#![allow(unused_imports)]
#![allow(dead_code)]

use crate::utilities::read_lines;

pub mod utilities;

use aoc_2025::day_1;
use aoc_2025::day_2;
use aoc_2025::day_3;
use aoc_2025::day_4;
use aoc_2025::day_5;
use aoc_2025::day_6;
use aoc_2025::day_7;

fn main() {
    let lines = read_lines("inputs/day_7.txt");

    println!("part 1 {}", day_7::part_1(&lines));
    println!("part 2 {}", day_7::part_2(&lines));
}
