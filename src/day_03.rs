use std::io::{self, BufRead};

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut counts: Vec<i32> = vec![];

    let mut line = String::new();
    let len = input.read_line(&mut line)?;

    line.trim().as_bytes().iter().for_each(|bit| match bit {
        0x31 => counts.push(1),  // Push positive for '1'
        0x30 => counts.push(-1), // Push negative for '0'
        _ => panic!("Unrecognized bit in first line!"),
    });

    loop {
        let mut line = String::new();
        let len = input.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        line.trim()
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, bit)| match bit {
                0x31 => counts[i] += 1, // Push positive for '1'
                0x30 => counts[i] -= 1, // Push negative for '0'
                _ => panic!("Unrecognized bit in first line!"),
            });
    }

    let gamma: Vec<u8> = counts
        .iter()
        .map(|count| if count > &0 { 0x31 } else { 0x30 })
        .collect();

    let epsilon: Vec<u8> = counts
        .iter()
        .map(|count| if count > &0 { 0x30 } else { 0x31 })
        .collect();

    println!("Gamma: {:?}", gamma);
    println!("Epsilon: {:?}", epsilon);

    let gamma = i32::from_str_radix(&String::from_utf8(gamma).unwrap(), 2);
    let epsilon = i32::from_str_radix(&String::from_utf8(epsilon).unwrap(), 2);

    println!("Gamma: {:?}", gamma);
    println!("Epsilon: {:?}", epsilon);

    Ok(gamma.unwrap() * epsilon.unwrap())
}
