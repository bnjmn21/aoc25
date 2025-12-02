pub fn a(input: &str) -> i64 {
    let mut ranges = vec![]; 
    for range_str in input.split(',') {
        let (start_str, end_str) = range_str.split_once('-').unwrap();
        let start: i64 = start_str.parse().unwrap();
        let end: i64 = end_str.parse().unwrap();
        let start_digits = start.ilog10();
        let end_digits = end.ilog10();
        for i in start_digits..=end_digits {
            if i % 2 == 1 {
                let start = if i == start_digits {
                    start
                } else {
                    10i64.pow(start_digits + 1)
                };
                let end = if i == end_digits {
                    end
                } else {
                    10i64.pow(end_digits) - 1
                };
                ranges.push((start, end));
            }
        }
    }
    
    println!("{:?}", &ranges);
    
    let mut res = 0i64;
    for (start, end) in ranges {
        let digits = start.ilog10() + 1;
        let half_start = start / (10i64.pow(digits / 2));
        let half_end = end / (10i64.pow(digits / 2));
        if half_start == half_end {
            let num = doublify(half_start, digits);
            if (start..=end).contains(&num) {
                res += num
            }
        } else {
            let first_num = doublify(half_start, digits);
            if start <= first_num {
                res += first_num
            }
            let last_num = doublify(half_end, digits);
            if last_num <= end {
                res += last_num
            }
            for i in (half_start + 1)..half_end {
                res += doublify(i, digits);
            }
        }
    }
    res
}

fn doublify(number: i64, digits: u32) -> i64 {
    number + (number * (10i64.pow(digits / 2)))
}