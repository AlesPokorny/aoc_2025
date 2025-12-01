use crate::utilities::read_lines;

mod utilities;

mod day_1;

fn main() {
    let lines = read_lines("inputs/day_1.txt");

    println!("{}", day_1::solve(&lines).0);
    println!("{}", day_1::solve(&lines).1);
}
