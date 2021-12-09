use std::io::{self, BufRead};

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut cur_y = 0;
    let mut cur_x = 0;

    loop {
        let mut line = String::new();
        let len = input.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        let mut split = line.trim().split(" ");
        let cmd = split.next().expect("Expected a cmd");
        let amount = i32::from_str_radix(split.next().unwrap(), 10).unwrap_or(0);

        match cmd {
            "forward" => cur_x += amount,
            "up" => cur_y -= amount,
            "down" => cur_y += amount,
            _ => panic!("Didnt recognize command: {}", line.trim()),
        }
    }

    Ok(cur_x * cur_y)
}
pub fn part_2(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut cur_y = 0;
    let mut cur_x = 0;
    let mut cur_aim = 0;

    loop {
        let mut line = String::new();
        let len = input.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        let mut split = line.trim().split(" ");
        let cmd = split.next().expect("Expected a cmd");
        let amount = i32::from_str_radix(split.next().unwrap(), 10).unwrap_or(0);

        match cmd {
            "forward" => {
                cur_x += amount;
                cur_y += cur_aim * amount;
            }
            "up" => cur_aim -= amount,
            "down" => cur_aim += amount,
            _ => panic!("Didnt recognize command: {}", line.trim()),
        }
    }

    Ok(cur_x * cur_y)
}
