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
use aoc_2025::day_8;
use aoc_2025::day_9;
use aoc_2025::day_10;

fn main() {
    let lines = read_lines("inputs/day_10.txt");

    println!("part 1 {}", day_10::part_1(&lines));
    println!("part 2 {}", day_10::part_2(&lines));
}
