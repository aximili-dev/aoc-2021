use std::{
    fmt::Display,
    io::{self, BufRead},
};

struct Board {
    numbers: Vec<i32>,
    marked: Vec<bool>,
}

impl Board {
    fn new() -> Board {
        Board {
            numbers: vec![0; 25],
            marked: vec![false; 25],
        }
    }

    fn numbers(&self) -> &Vec<i32> {
        &self.numbers
    }

    fn set(&mut self, x: usize, y: usize, val: i32) {
        self.numbers[y * 5 + x] = val;
    }

    fn called_num(&mut self, num: i32) {
        let found = self.numbers.iter().position(|x| *x == num);

        if let Some(index) = found {
            self.marked[index] = true;
        }
    }

    fn has_won(&self) -> bool {
        let any_rows = self
            .marked
            .chunks(5)
            .any(|chunk| chunk.iter().all(|marked| *marked));

        if any_rows {
            return any_rows;
        }

        for x in 0..5 {
            let mut done = true;

            for y in 0..5 {
                if self.marked[y * 5 + x] == false {
                    done = false;
                }
            }

            if done {
                return true;
            }
        }

        false
    }

    fn score(&self) -> i32 {
        let mut score = 0;

        for y in 0..5 {
            for x in 0..5 {
                let i = y * 5 + x;

                if !self.marked[i] {
                    score += self.numbers[i]
                }
            }
        }

        score
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numbers = self.numbers();

        for y in 0..5 {
            for x in 0..5 {
                write!(f, "{}", numbers[y * 5 + x]);

                if x != 4 {
                    write!(f, " ");
                }
            }

            writeln!(f);
        }

        Ok(())
    }
}

fn read_numbers(input: &mut dyn BufRead) -> io::Result<Vec<i32>> {
    let mut line = String::new();

    // Read numbers line
    input.read_line(&mut line)?;

    let numbers: Vec<i32> = line
        .trim()
        .split(",")
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    Ok(numbers)
}

fn read_board(input: &mut dyn BufRead) -> io::Result<Board> {
    let mut board = Board::new();

    for y in 0..5 {
        let mut line = String::new();
        input.read_line(&mut line)?;

        line.trim()
            .split_whitespace()
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .enumerate()
            .for_each(|(x, val)| board.set(x, y, val));
    }

    Ok(board)
}

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let numbers = read_numbers(input)?;
    let mut boards: Vec<Board> = Vec::new();

    println!("Read numbers: {:?}", numbers);

    loop {
        let mut line = String::new();
        let len = input.read_line(&mut line)?;

        if len == 0 {
            break;
        }

        let board = read_board(input)?;

        println!("Read board:\n {}", &board);

        boards.push(board);
    }

    for n in numbers.iter() {
        boards.iter_mut().for_each(|board| board.called_num(*n));

        if let Some(board) = boards.iter().find(|b| b.has_won()) {
            return Ok(*n * board.score());
        }
    }

    Ok(0)
}
