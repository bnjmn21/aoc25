#[cfg(feature = "10b")]
use good_lp::{Expression, Solution, SolverModel, default_solver, variables};

pub fn a(input: &str) -> u32 {
    let mut machines = vec![];
    for line in input.lines() {
        let mut machine = MachineA::default();
        for part in line.split_ascii_whitespace() {
            if part.starts_with('[') {
                for (i, light) in part[1..(part.len() - 1)].chars().enumerate() {
                    machine.lights |= ((light == '#') as u32) << i;
                }
            } else if part.starts_with('(') {
                let mut button = 0;
                for bit in part[1..(part.len() - 1)].split(',') {
                    button |= 1 << bit.parse::<u32>().unwrap();
                }
                machine.buttons.push(button);
            }
        }
        machines.push(machine);
    }

    let mut res = 0;
    for machine in machines {
        let mut min = u32::MAX;
        for i in 0u32..(1 << machine.buttons.len()) {
            let mut lights = 0;
            for (bit, button) in machine.buttons.iter().enumerate() {
                if (i >> bit) & 1 == 1 {
                    lights ^= button;
                }
            }
            if lights == machine.lights {
                min = min.min(i.count_ones());
            }
        }
        res += min;
    }
    res
}

#[derive(Default)]
pub struct MachineA {
    lights: u32,
    buttons: Vec<u32>,
}

#[cfg(feature = "10b")]
pub fn b(input: &str) -> u32 {
    let mut machines = vec![];
    for line in input.lines() {
        let mut machine = MachineB::default();
        for part in line.split_ascii_whitespace() {
            if part.starts_with('(') {
                machine.buttons.push(
                    part[1..(part.len() - 1)]
                        .split(',')
                        .map(|v| v.parse().unwrap())
                        .collect(),
                );
            } else if part.starts_with('{') {
                machine.counters = part[1..(part.len() - 1)]
                    .split(',')
                    .map(|v| v.parse().unwrap())
                    .collect();
            }
        }
        machines.push(machine);
    }

    let mut res = 0;
    for machine in machines {
        variables! {
            problem:
            0 <= x[machine.buttons.len()] (integer)
        }
        let objective: Expression = x.iter().sum();
        let solution = problem
            .minimise(&objective)
            .using(default_solver)
            .with_all(machine.counters.iter().enumerate().map(|(i, count)| {
                machine
                    .buttons
                    .iter()
                    .enumerate()
                    .filter(|(_, btn)| btn.contains(&i))
                    .map(|(bi, _)| x[bi])
                    .sum::<Expression>()
                    .eq(*count)
            }))
            .solve()
            .unwrap();

        res += solution.eval(objective).round() as u32;
    }

    res
}

#[derive(Default)]
pub struct MachineB {
    buttons: Vec<Vec<usize>>,
    counters: Vec<u32>,
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "10_example.txt" => 7);
    }

    #[cfg(feature = "10b")]
    #[test]
    fn b_example() {
        test_solution!(b, "10_example.txt" => 33);
    }
}
