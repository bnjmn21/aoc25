use std::fs;

pub fn a(input: &str) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in input.lines() {
        if !line.contains(|c: char| c.is_ascii_alphanumeric()) {
            continue;
        }
        let (direction, amount) = line.split_at(1);
        let parsed_amount: i32 = amount.parse().unwrap();
        let rotation = if direction == "L" {
            -parsed_amount
        } else {
            parsed_amount
        };
        dial = (dial + rotation).rem_euclid(100);
        if dial == 0 {
            zeros += 1;
        }
    }
    zeros
}

pub fn b(input: &str) -> i32 {
    let mut dial = 50;
    let mut zeros = 0;
    for line in input.lines() {
        if !line.contains(|c: char| c.is_ascii_alphanumeric()) {
            continue;
        }
        let (direction, amount) = line.split_at(1);
        let parsed_amount: i32 = amount.parse().unwrap();
        let rotation = if direction == "L" {
            -parsed_amount
        } else {
            parsed_amount
        };
        let old_pos = dial;
        let new_pos = dial + rotation;
        dial = new_pos.rem_euclid(100);
        if direction == "L" {
            zeros += (new_pos - 1).div_euclid(100).abs();
            if old_pos == 0 {
                zeros -= 1;
            }
        } else {
            zeros += new_pos.div_euclid(100).abs();
        }
    }
    zeros
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    #[test]
    fn d1a_example() {
        test_solution!(super::a, "1_example.txt" => 3);
    }

    #[test]
    fn d1b_example() {
        test_solution!(super::b, "1_example.txt" => 6);
    }
}
