pub fn a(input: &str) -> u64 {
    let mut problems: Vec<ProblemA> = vec![];
    for (i, line) in input.lines().enumerate() {
        for (j, cell) in line.split_ascii_whitespace().enumerate() {
            if cell == "+" || cell == "*" {
                problems[j].op = Some(cell.chars().next().unwrap());
                continue;
            }

            let num: u64 = cell.parse().unwrap();
            if i == 0 {
                problems.push(ProblemA {
                    nums: vec![num],
                    op: None,
                });
            } else {
                problems[j].nums.push(num);
            }
        }
    }

    let mut res = 0;
    for problem in problems {
        if problem.op == Some('+') {
            res += problem.nums.iter().sum::<u64>();
        } else if problem.op == Some('*') {
            res += problem.nums.iter().product::<u64>();
        }
    }

    res
}

struct ProblemA {
    nums: Vec<u64>,
    op: Option<char>,
}

pub fn b(input: &str) -> u64 {
    let ops_line = input.lines().last().unwrap();
    let mut problems = vec![];
    let mut op = 0u8;
    let mut width = 0usize;
    for c in ops_line.bytes() {
        if c == b'+' || c == b'*' {
            if op != 0 {
                problems.push(ProblemB {
                    op,
                    width,
                    rows: vec![],
                });
                width = 0;
            }
            op = c;
        } else {
            width += 1;
        }
    }
    problems.push(ProblemB {
        op,
        width: width + 1,
        rows: vec![],
    });

    for line in input.lines() {
        if line.starts_with('+') || line.starts_with('*') {
            break;
        }
        let mut remaining_chars = line;
        for problem in &mut problems {
            problem.rows.push(&remaining_chars[..problem.width]);
            if problem.width + 1 < remaining_chars.len() {
                remaining_chars = &remaining_chars[(problem.width + 1)..];
            }
        }
    }

    let mut res = 0;
    for problem in problems {
        let mut columns: Vec<String> = vec![];
        for (ri, row) in problem.rows.iter().enumerate() {
            for (i, c) in row.chars().enumerate() {
                if ri == 0 {
                    columns.push(c.into());
                } else {
                    columns[i].push(c);
                }
            }
        }

        let parsed_columns = columns.iter().map(|c| {
            c.split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap()
        });
        if problem.op == b'+' {
            res += parsed_columns.sum::<u64>();
        } else if problem.op == b'*' {
            res += parsed_columns.product::<u64>();
        }
    }

    res
}

#[derive(Debug)]
struct ProblemB<'a> {
    op: u8,
    width: usize,
    rows: Vec<&'a str>,
}

#[cfg(test)]
mod tests {
    use crate::test_solution;

    use super::*;

    #[test]
    fn a_example() {
        test_solution!(a, "6_example.txt" => 4277556);
    }

    #[test]
    fn b_example() {
        test_solution!(b, "6_example.txt" => 3263827);
    }
}
