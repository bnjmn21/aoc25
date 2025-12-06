use std::ops::RangeInclusive;

pub fn a(input: &str) -> u32 {
    let mut ranges = vec![];
    let mut done_parsing_ranges = false;
    let mut res = 0;

    for line in input.lines() {
        if line.is_empty() {
            done_parsing_ranges = true;
        } else if !done_parsing_ranges {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            ranges.push(start..=end);
        } else {
            let value: u64 = line.parse().unwrap();
            res += ranges.iter().any(|range| range.contains(&value)) as u32;
        }
    }

    res
}

pub fn b(input: &str) -> u64 {
    let mut ranges: Vec<RangeInclusive<u64>> = vec![];
    let mut done_parsing_ranges = false;

    for line in input.lines() {
        if line.is_empty() {
            done_parsing_ranges = true;
        } else if !done_parsing_ranges {
            let (start_str, end_str) = line.split_once('-').unwrap();
            let start: u64 = start_str.parse().unwrap();
            let end: u64 = end_str.parse().unwrap();
            ranges.retain(|range| {
                !((start..=end).contains(range.start()) && (start..=end).contains(range.end()))
            });
            let start_range_pos = ranges.iter().position(|range| range.contains(&start));
            let end_range_pos = ranges.iter().position(|range| range.contains(&end));
            if let Some(mut srp) = start_range_pos
                && let Some(erp) = end_range_pos
            {
                if srp != erp {
                    let end_range = ranges.swap_remove(erp);
                    if srp == ranges.len() {
                        // fix msrp after swap_remove
                        srp = erp;
                    }
                    ranges[srp] = (*ranges[srp].start())..=(*end_range.end());
                }
            } else if let Some(srp) = start_range_pos {
                ranges[srp] = (*ranges[srp].start())..=end;
            } else if let Some(erp) = end_range_pos {
                ranges[erp] = start..=(*ranges[erp].end());
            } else {
                ranges.push(start..=end);
            }
        }
    }

    let mut res = 0;
    for range in ranges {
        res += range.end() - range.start() + 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "5_example.txt" => 3);
    }

    #[test]
    fn b_example() {
        test_solution!(b, "5_example.txt" => 14);
    }
}
