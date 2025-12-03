pub fn a(input: &str) -> u32 {
    let mut res = 0u32;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let nums = line.chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        
        let (max_i, max) = nums[..(nums.len() - 1)].iter().enumerate()
            .reduce(|acc, e| if e.1 > acc.1 {e} else {acc})
            .unwrap();
            
        let second_max = nums[(max_i + 1)..].iter()
            .max().unwrap();
        
        res += (max * 10) + second_max;
    }
    res
}

pub fn b(input: &str) -> u64 {
    let mut res = 0u64;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let nums = line.chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect::<Vec<_>>();
        
        let mut start = 0usize;
        let mut end = nums.len() - 11;
        let mut digits = 0;
        
        for _ in 0..12 {
            let (max_i, max) = nums[start..end].iter().enumerate()
                .reduce(|acc, e| if e.1 > acc.1 {e} else {acc})
                .unwrap();
            start += max_i + 1;
            end += 1;
            digits = (digits * 10) + max;
        }
        res += digits;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "3_example.txt" => 357);
    }

    #[test]
    fn b_example() {
        test_solution!(a, "3_example.txt" => 3121910778619);
    }
}