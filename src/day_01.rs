use std::io::{self, BufRead};

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut increases = 0;
    let mut prev = i32::MAX;

    loop {
        let mut line = String::new();
        let len = input.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        let depth = i32::from_str_radix(&line.trim(), 10).unwrap_or(i32::MIN);

        if depth > prev {
            increases += 1;
        }

        prev = depth
    }

    Ok(increases)
}

pub fn part_2(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut increases = 0;
    let mut prev_window = i32::MAX;

    let mut readings: Vec<i32> = vec![];

    loop {
        let mut line = String::new();
        let len = input.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        let depth = i32::from_str_radix(line.trim(), 10).unwrap_or(i32::MIN);

        readings.insert(0, depth);

        if readings.len() >= 3 {
            let window = readings[0..3].iter().sum();

            if window > prev_window {
                increases += 1;
            }

            prev_window = window;
        }
    }

    Ok(increases)
}
