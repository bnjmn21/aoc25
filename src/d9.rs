pub fn a(input: &str) -> u64 {
    let coords: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let len = coords.len();

    let mut max = 0;
    for a in 0..(len - 1) {
        for b in (a + 1)..len {
            let size = ((coords[a].0 - coords[b].0).unsigned_abs() + 1)
                * ((coords[a].1 - coords[b].1).unsigned_abs() + 1);
            if size > max {
                max = size;
            }
        }
    }
    max
}

pub fn b(input: &str) -> u64 {
    let coords: Vec<(i64, i64)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<_>>();

    let len = coords.len();
    let mut max = 0;

    for a in 0..(len - 1) {
        for b in (a + 1)..len {
            let ca = &coords[a];
            let cb = &coords[b];
            let size = ((ca.0 - cb.0).unsigned_abs() + 1) * ((ca.1 - cb.1).unsigned_abs() + 1);
            if size > max {
                let minpt = (ca.0.min(cb.0), ca.1.min(cb.1));
                let maxpt = (ca.0.max(cb.0), ca.1.max(cb.1));
                let mut invalid = false;
                let mut prev = &coords[coords.len() - 1];
                for curr in &coords {
                    let line_crosses_box = (prev.0 < maxpt.0
                        && prev.1 < maxpt.1
                        && curr.0 > minpt.0
                        && curr.1 > minpt.1)
                        || (prev.0 > minpt.0
                            && prev.1 > minpt.1
                            && curr.0 < maxpt.0
                            && curr.1 < maxpt.1);
                    if line_crosses_box {
                        invalid = true;
                        break;
                    }
                    prev = curr;
                }
                if !invalid {
                    max = size;
                }
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "9_example.txt" => 50);
    }

    #[test]
    fn b_example() {
        test_solution!(b, "9_example.txt" => 24);
    }
}
