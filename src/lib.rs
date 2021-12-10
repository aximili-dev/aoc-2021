#![feature(iter_zip)]
#![feature(test)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;

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
