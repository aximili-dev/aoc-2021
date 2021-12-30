use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use crate::read_numbers_line;

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut crab_positions = read_numbers_line(input, ",")?;
    let even_crabs = crab_positions.len() % 2 == 0;
    let half_pos = crab_positions.len() / 2;

    crab_positions.sort();

    let median = if even_crabs {
        (crab_positions[half_pos] + crab_positions[half_pos - 1]) / 2
    } else {
        crab_positions[half_pos]
    };

    println!("Calculated median: {}", median);

    let mut distance_sum = 0;

    crab_positions
        .iter()
        .for_each(|d| distance_sum += (d - median).abs());

    Ok(distance_sum)
}

pub fn part_2(input: &mut dyn BufRead) -> io::Result<i32> {
    let crab_positions = read_numbers_line(input, ",")?;

    // let mode = amount_per_pos.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().0;
    let total = crab_positions.iter().sum::<i32>() as f64;
    let avg = total / crab_positions.len() as f64;
    let avg = avg.round() as i32;

    // println!("Calculated mode: {}", mode);
    println!("Calculated average: {}", avg);

    let mut distance_sum = 0;

    crab_positions.iter().for_each(|p| {
        let n = (p - avg).abs();
        distance_sum += (n * (n + 1)) / 2;
    });

    Ok(distance_sum)
}
