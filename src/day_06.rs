use std::{
    collections::HashMap,
    io::{self, BufRead},
    ops::RangeBounds,
};

use crate::read_numbers_line;

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut fish = read_numbers_line(input, ",")?;

    for _ in 0..80 {
        let mut repr_this_gen = 0;

        fish.iter_mut().for_each(|f| *f -= 1);

        fish.iter()
            .filter(|f| **f < 0)
            .for_each(|_| repr_this_gen += 1);

        fish.iter_mut().filter(|f| **f < 0).for_each(|f| *f = 6);

        fish.append(&mut vec![8; repr_this_gen]);
    }

    Ok(fish.len() as i32)
}

pub fn part_2(input: &mut dyn BufRead) -> io::Result<u64> {
    let fish = read_numbers_line(input, ",")?;
    let mut fish_map: HashMap<i32, u64> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    println!("Read the following fish: {:?}", fish);

    fish.iter().for_each(|f| {
        if fish_map.contains_key(f) {
            fish_map.insert(*f, fish_map[f] + 1);
        } else {
            fish_map.insert(*f, 1);
        }
    });

    for gen in 0..256 {
        let mut new_map: HashMap<i32, u64> = HashMap::new();

        fish_map.iter().for_each(|(cur_life, amount)| {
            new_map.insert(cur_life - 1, *amount);
        });

        new_map.insert(
            6,
            new_map.get(&6).unwrap_or(&0) + new_map.get(&-1).unwrap_or(&0),
        );

        new_map.insert(8, *new_map.get(&-1).unwrap_or(&0));

        new_map.iter().for_each(|(cur_life, amount)| {
            fish_map.insert(cur_life.clone(), amount.clone());
        });

        fish_map.remove(&-1);

        println!("After {} days: {:?}", gen + 1, fish_map);
    }

    Ok(fish_map.values().sum::<u64>())
}
