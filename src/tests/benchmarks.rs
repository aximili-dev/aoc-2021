extern crate test;

use test::Bencher;

use crate::{day_01, day_02, day_03, day_04, get_puzzle_input};

#[bench]
fn day_01_part_01(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(1, false));

    b.bench(|_| {
        day_01::part_1(&mut boxed_buf);
    });
}

#[bench]
fn day_01_part_02(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(1, false));

    b.bench(|_| {
        day_01::part_2(&mut boxed_buf);
    });
}

#[bench]
fn day_02_part_01(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(2, false));

    b.bench(|_| {
        day_02::part_1(&mut boxed_buf);
    });
}

#[bench]
fn day_02_part_02(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(2, false));

    b.bench(|_| {
        day_02::part_2(&mut boxed_buf);
    });
}

#[bench]
fn day_03_part_01(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(3, false));

    b.bench(|_| {
        day_03::part_1(&mut boxed_buf);
    });
}

#[bench]
fn day_04_part_01(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(4, false));

    b.bench(|_| {
        day_04::part_1(&mut boxed_buf);
    });
}

#[bench]
fn day_04_part_02(b: &mut Bencher) {
    let mut boxed_buf = test::black_box(get_puzzle_input(4, false));

    b.bench(|_| {
        day_04::part_2(&mut boxed_buf);
    });
}
