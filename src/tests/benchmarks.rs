extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use test::Bencher;

use crate::day_01;

#[bench]
fn day_01_part_01(b: &mut Bencher) {
    let input = File::open("./inputs/day-01-input.txt").expect("Expected to open file");
    let mut buf = BufReader::new(input);

    let boxed_buf = test::black_box(&mut buf);

    b.bench(|_| {
        println!(
            "Number of increases: {}",
            day_01::part_1(boxed_buf as &mut dyn BufRead).unwrap()
        )
    });
}
