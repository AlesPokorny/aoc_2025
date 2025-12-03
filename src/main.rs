#![allow(unused_imports)]
#![allow(dead_code)]

use crate::utilities::read_lines;

pub mod utilities;

use aoc_2025::day_1;
use aoc_2025::day_2;
use aoc_2025::day_3;

fn main() {
    // let lines = read_lines("inputs/examples/day_3.txt");
    let lines = read_lines("inputs/day_3.txt");

    println!("part 1 {}", day_3::part_1(&lines));
    println!("part 2 {}", day_3::part_2(&lines));
}
