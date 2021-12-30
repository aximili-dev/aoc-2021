#![feature(exclusive_range_pattern)]
#![feature(iter_zip)]
#![feature(test)]

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;

#[cfg(test)]
mod tests;

pub fn get_puzzle_file_name(day: i32, example: bool) -> String {
    if example {
        format!("./inputs/day-{:0>2}-example.txt", day)
    } else {
        format!("./inputs/day-{:0>2}-input.txt", day)
    }
}

pub fn get_puzzle_input(day: i32, example: bool) -> Box<dyn BufRead> {
    let name = get_puzzle_file_name(day, example);
    let file = File::open(name).expect("Expected to open file");
    let buf = BufReader::new(file);

    Box::new(buf)
}

pub fn read_numbers_line(input: &mut dyn BufRead, pat: &str) -> io::Result<Vec<i32>> {
    let mut line = String::new();

    input.read_line(&mut line)?;

    let numbers: Vec<i32> = line
        .trim()
        .split(pat)
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    Ok(numbers)
}
