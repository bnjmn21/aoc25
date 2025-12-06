use std::mem::replace;

pub fn a(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let height = input.iter().filter(|c| **c == b'\n').count() + 1;
    let mut data = Vec::with_capacity(height);
    let mut row = Vec::with_capacity(width);
    for c in input {
        if *c == b'\n' {
            data.push(replace(&mut row, Vec::with_capacity(width)));
        } else if *c == b'@' {
            row.push(true);
        } else if *c == b'.' {
            row.push(false);
        }
    }
    data.push(row);

    let mut res = 0;
    for (y, row) in data.iter().enumerate() {
        for (x, bit) in row.iter().enumerate() {
            if !bit {
                continue;
            }
            let mut count = 0u32;
            let x = x as isize;
            let y = y as isize;
            count += get_bit(&data, x - 1, y - 1) as u32;
            count += get_bit(&data, x - 1, y) as u32;
            count += get_bit(&data, x - 1, y + 1) as u32;
            count += get_bit(&data, x, y - 1) as u32;
            count += get_bit(&data, x, y + 1) as u32;
            count += get_bit(&data, x + 1, y - 1) as u32;
            count += get_bit(&data, x + 1, y) as u32;
            count += get_bit(&data, x + 1, y + 1) as u32;
            if count < 4 {
                res += 1;
            }
        }
    }
    res
}

pub fn b(input: &str) -> u32 {
    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let height = input.iter().filter(|c| **c == b'\n').count() + 1;
    let mut data = Vec::with_capacity(height);
    let mut row = Vec::with_capacity(width);
    for c in input {
        if *c == b'\n' {
            data.push(replace(&mut row, Vec::with_capacity(width)));
        } else if *c == b'@' {
            row.push(true);
        } else if *c == b'.' {
            row.push(false);
        }
    }
    data.push(row);

    let mut res = 0;
    loop {
        let mut changes = vec![];
        for y in 0..(height as isize) {
            for x in 0..(width as isize) {
                if !data[y as usize][x as usize] {
                    continue;
                }
                let mut count = 0u32;
                count += get_bit(&data, x - 1, y - 1) as u32;
                count += get_bit(&data, x - 1, y) as u32;
                count += get_bit(&data, x - 1, y + 1) as u32;
                count += get_bit(&data, x, y - 1) as u32;
                count += get_bit(&data, x, y + 1) as u32;
                count += get_bit(&data, x + 1, y - 1) as u32;
                count += get_bit(&data, x + 1, y) as u32;
                count += get_bit(&data, x + 1, y + 1) as u32;
                if count < 4 {
                    res += 1;
                    changes.push((x as usize, y as usize));
                }
            }
        }
        if changes.is_empty() {
            break;
        } else {
            for (x, y) in changes {
                data[y][x] = false;
            }
        }
    }

    res
}

fn get_bit(data: &Vec<Vec<bool>>, x: isize, y: isize) -> bool {
    if x < 0 || y < 0 {
        return false;
    }

    data.get(y as usize)
        .and_then(|row| row.get(x as usize))
        .cloned()
        .unwrap_or(false)
}
