#[macro_use]
extern crate clap;

use aoc_2021::{day_01, day_02, day_03, day_04, day_05, day_06, day_07, get_puzzle_input};

fn main() {
    let matches = clap_app!(
        aoc =>
            (version: "1.0")
            (author: "Daniel Litvak")
            (about: "Advent of Code - 2021")
            (@arg DAY: -d +required +takes_value "Specifies which day's puzzles to run")
            (@arg PART: -p +required +takes_value "Specifies which part of a day to run (requires -d)")
            (@arg EXAMPLE: -e "Uses example input")
    )
    .get_matches();

    let day = matches.value_of("DAY").unwrap();
    let part = matches.value_of("PART").unwrap();
    let use_example = matches.is_present("EXAMPLE");

    let day = i32::from_str_radix(day, 10).unwrap();
    let part = i32::from_str_radix(part, 10).unwrap();

    let mut buf = get_puzzle_input(day, use_example);

    let result = match (day, part) {
        (1, 1) => day_01::part_1(&mut buf),
        (1, 2) => day_01::part_2(&mut buf),
        (2, 1) => day_02::part_1(&mut buf),
        (2, 2) => day_02::part_2(&mut buf),
        (3, 1) => day_03::part_1(&mut buf),
        (4, 1) => day_04::part_1(&mut buf),
        (4, 2) => day_04::part_2(&mut buf),
        (5, 1) => day_05::part_1(&mut buf),
        (5, 2) => day_05::part_2(&mut buf),
        (6, 1) => day_06::part_1(&mut buf),
        (6, 2) => {
            let result = day_06::part_2(&mut buf).unwrap();
            println!("Day 6 part 2 result: {}", result);
            return;
        }
        (7, 1) => day_07::part_1(&mut buf),
        (7, 2) => day_07::part_2(&mut buf),
        _ => panic!(
            "Ho! Ho! Ho! Day {} part {} hasn't been implemented yet!",
            day, part
        ),
    };

    println!("Day {} part {} result: {}", day, part, result.unwrap());
}
