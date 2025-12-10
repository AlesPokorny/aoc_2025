use good_lp::{
    Expression, IntoAffineExpression, Solution, SolverModel, Variable, default_solver, variable,
    variables,
};
use itertools::Itertools;

struct Machine {
    n_switches: usize,
    n_lights: usize,
    switches: Vec<Vec<u8>>,
    target: Vec<u8>,
    joltage: Vec<u16>,
}

impl Machine {
    fn new(lights: Vec<u8>, switches: Vec<Vec<u8>>, joltage: Vec<u16>) -> Self {
        Machine {
            n_switches: switches.len(),
            n_lights: lights.len(),
            switches,
            target: lights,
            joltage,
        }
    }

    fn try_lights(&self, switch_idxs: &[usize]) -> bool {
        for i in 0..self.n_lights {
            let mut light = 0;
            for switch_idx in switch_idxs.iter() {
                light ^= self.switches[*switch_idx][i];
            }
            if light != self.target[i] {
                return false;
            }
        }
        true
    }
}

fn parse_data(line: &str) -> Machine {
    let (lights_str, rest) = line.split_once(']').unwrap();
    let target_lights = lights_str[1..]
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("unexpected char"),
        })
        .collect::<Vec<u8>>();

    let n_lights = target_lights.len();

    let (switches_str, joltages_str) = rest.split_once('{').unwrap();
    let switches = switches_str
        .trim()
        .split(' ')
        .map(|s| {
            let mut output = vec![0; n_lights];
            for c in s[1..s.len() - 1].split(',') {
                output[c.parse::<usize>().unwrap()] = 1;
            }
            output
        })
        .collect::<Vec<Vec<u8>>>();
    let joltages = joltages_str[..joltages_str.len() - 1]
        .split(',')
        .map(|s| s.parse::<u16>().unwrap())
        .collect::<Vec<u16>>();

    Machine::new(target_lights, switches, joltages)
}

pub fn part_1(lines: &[String]) -> usize {
    let mut n_switch_presses = 0;
    'machine_loop: for line in lines {
        let machine = parse_data(line);
        for n_combinations in 1..=machine.n_switches {
            for combination in (0..machine.n_switches).combinations(n_combinations) {
                if machine.try_lights(&combination) {
                    n_switch_presses += n_combinations;
                    continue 'machine_loop;
                }
            }
        }
    }

    n_switch_presses
}

pub fn part_2(lines: &[String]) -> usize {
    let mut n_presses = 0;
    for line in lines {
        let machine = parse_data(line);

        let mut vars = variables!();

        let press_vars = (0..machine.n_switches)
            .map(|j| vars.add(variable().integer().min(0).name(format!("x_{}", j))))
            .collect::<Vec<Variable>>();
        let objective_fn: Expression = press_vars.iter().map(|&v| Expression::from(v)).sum();

        let mut problem = vars.minimise(objective_fn).using(default_solver);
        for i in 0..machine.n_lights {
            let mut lhs = Expression::with_capacity(machine.n_switches);

            #[allow(clippy::needless_range_loop)]
            for j in 0..machine.n_switches {
                let coefficient = machine.switches[j][i];
                if coefficient != 0 {
                    lhs += press_vars[j].into_expression() * coefficient;
                }
            }

            let rhs = machine.joltage[i];
            problem.add_constraint(lhs.eq(rhs));
        }

        problem.set_parameter("log", "0"); // STFU
        match problem.solve() {
            Ok(solution) => {
                n_presses += press_vars
                    .iter()
                    .map(|var| solution.value(*var).round() as usize)
                    .sum::<usize>()
            }
            Err(_) => panic!("Infeasible solution"),
        }
    }

    n_presses
}

#[cfg(test)]
mod day_10_tests {
    use crate::day_10::*;
    use crate::utilities::read_lines;

    #[test]
    fn test_part_1_example() {
        let lines = read_lines("inputs/examples/day_10.txt");

        assert_eq!(part_1(&lines), 7);
    }

    #[test]
    fn test_part_2_example() {
        let lines = read_lines("inputs/examples/day_10.txt");

        assert_eq!(part_2(&lines), 33);
    }

    #[test]
    fn test_part_1_real() {
        let lines = read_lines("inputs/day_10.txt");

        assert_eq!(part_1(&lines), 558);
    }

    #[test]
    fn test_part_2_real() {
        let lines = read_lines("inputs/day_10.txt");

        assert_eq!(part_2(&lines), 20317);
    }
}
