pub fn a(input: &str) -> u32 {
    let mut lines = input.lines().step_by(2);
    let mut beams = vec![lines.next().unwrap().find('S').unwrap()];
    let mut res = 0;
    for line in lines {
        let mut new_beams = vec![];

        for beam in beams {
            if line.as_bytes()[beam] == b'^' {
                res += 1;
                if new_beams.last().cloned() != Some(beam - 1) {
                    new_beams.push(beam - 1);
                }
                new_beams.push(beam + 1);
            } else if new_beams.last().cloned() != Some(beam) {
                new_beams.push(beam);
            }
        }

        beams = new_beams;
    }
    res
}

pub fn b(input: &str) -> u64 {
    let mut lines = input.lines().step_by(2);
    let mut beams = vec![(lines.next().unwrap().find('S').unwrap(), 1u64)];
    let mut res = 1;
    for line in lines {
        let mut new_beams: Vec<(usize, u64)> = vec![];
        for (beam, instances) in beams {
            if line.as_bytes()[beam] == b'^' {
                res += instances;
                if new_beams.last().cloned().is_none_or(|b| b.0 != beam - 1) {
                    new_beams.push((beam - 1, instances));
                } else {
                    new_beams.last_mut().unwrap().1 += instances;
                }
                new_beams.push((beam + 1, instances));
            } else if new_beams.last().cloned().is_none_or(|b| b.0 != beam) {
                new_beams.push((beam, instances));
            } else {
                new_beams.last_mut().unwrap().1 += instances;
            }
        }

        beams = new_beams;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "7_example.txt" => 21);
    }

    #[test]
    fn b_example() {
        test_solution!(b, "7_example.txt" => 40);
    }
}
