use std::collections::HashSet;

pub fn a(input: &str) -> i64 {
    let mut ranges = vec![];
    for range_str in input.split(',') {
        let (start_str, end_str) = range_str.split_once('-').unwrap();
        let start: i64 = start_str.parse().unwrap();
        let end: i64 = end_str.parse().unwrap();
        let start_digits = start.ilog10() + 1;
        let end_digits = end.ilog10() + 1;
        for i in start_digits..=end_digits {
            if i % 2 == 0 {
                let start = if i == start_digits {
                    start
                } else {
                    10i64.pow(i - 1)
                };
                let end = if i == end_digits {
                    end
                } else {
                    10i64.pow(i) - 1
                };
                ranges.push((start, end));
            }
        }
    }

    let mut res = 0i64;
    for (start, end) in ranges {
        let digits = start.ilog10() + 1;
        let half_start = half(start, digits);
        let half_end = half(end, digits);
        if half_start == half_end {
            let num = double(half_start, digits);
            if (start..=end).contains(&num) {
                res += num;
            }
        } else {
            let first_num = double(half_start, digits);
            if start <= first_num {
                res += first_num;
            }
            let last_num = double(half_end, digits);
            if last_num <= end {
                res += last_num;
            }
            for i in (half_start + 1)..half_end {
                res += double(i, digits);
            }
        }
    }
    res
}

fn double(number: i64, digits: u32) -> i64 {
    number + (number * (10i64.pow(digits / 2)))
}

fn half(number: i64, digits: u32) -> i64 {
    number / (10i64.pow(digits / 2))
}

pub fn b(input: &str) -> i64 {
    let mut ranges = vec![];
    for range_str in input.split(',') {
        let (start_str, end_str) = range_str.split_once('-').unwrap();
        let start: i64 = start_str.parse().unwrap();
        let end: i64 = end_str.parse().unwrap();
        let start_digits = start.ilog10() + 1;
        let end_digits = end.ilog10() + 1;
        for digits in start_digits..=end_digits {
            if digits < 2 {
                continue;
            }
            let start = if digits == start_digits {
                start
            } else {
                10i64.pow(digits - 1)
            };
            let end = if digits == end_digits {
                end
            } else {
                10i64.pow(digits) - 1
            };
            ranges.push((start, end));
        }
    }

    let mut res = 0i64;
    for (start, end) in ranges {
        let mut found_ids = HashSet::new();
        let digits = start.ilog10() + 1;
        for len in 1..=(digits / 2) {
            if digits % len != 0 {
                continue;
            }
            let times = digits / len;
            let start_piece = piece(start, digits, len);
            let end_piece = piece(end, digits, len);
            if start_piece == end_piece {
                let num = repeat(start_piece, len, times);
                if (start..=end).contains(&num) && found_ids.insert(num) {
                    res += num;
                }
            } else {
                let first_num = repeat(start_piece, len, times);
                if start <= first_num && found_ids.insert(first_num) {
                    res += first_num;
                }
                let last_num = repeat(end_piece, len, times);
                if last_num <= end && found_ids.insert(last_num) {
                    res += last_num;
                }
                for i in (start_piece + 1)..end_piece {
                    let num = repeat(i, len, times);
                    if found_ids.insert(num) {
                        res += num;
                    }
                }
            }
        }
    }

    res
}

fn piece(number: i64, digits: u32, len: u32) -> i64 {
    number / (10i64.pow(digits - len))
}

fn repeat(piece: i64, len: u32, times: u32) -> i64 {
    let mut res = 0;
    let mut offset = 1;
    let exp = 10i64.pow(len);
    for _ in 0..times {
        res += piece * offset;
        offset *= exp;
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "2_example.txt" => 1227775554);
    }

    #[test]
    fn a_big_range() {
        assert_eq!(a("10-40"), 11 + 22 + 33);
    }

    #[test]
    fn a_many_orders_of_magnitude() {
        assert_eq!(
            a("10-1200"),
            11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99 + 1010 + 1111
        )
    }

    #[test]
    fn b_example() {
        test_solution!(b, "2_example.txt" => 4174379265);
    }
}
