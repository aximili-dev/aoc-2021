#[macro_use]
extern crate clap;

use std::{fs::File, io::BufReader};

use aoc_2021::day_01;

fn main() {
    let matches = clap_app!(
        aoc =>
            (version: "1.0")
            (author: "Daniel Litvak")
            (about: "Advent of Code - 2021")
            (@arg DAY: -d +required +takes_value "Specifies which day's puzzles to run")
            (@arg PART: -p +required +takes_value "Specifies which part of a day to run (requires -d)")
    )
    .get_matches();

    let day = matches.value_of("DAY").unwrap();
    let part = matches.value_of("PART").unwrap();

    let day = i32::from_str_radix(day, 10).unwrap();
    let part = i32::from_str_radix(part, 10).unwrap();

    match (day, part) {
        (1, _) => {
            let input = File::open("./inputs/day-01-input.txt").expect("Expected to open file");
            let mut buf = BufReader::new(input);

            let r = if part == 1 {
                day_01::part_1(&mut buf)
            } else {
                day_01::part_2(&mut buf)
            };

            println!("Number of increases: {}", r.unwrap())
        }
        _ => panic!(
            "Ho! Ho! Ho! Day {} part {} hasn't been implemented yet!",
            day, part
        ),
    }
}
