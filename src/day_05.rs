use std::{
    collections::HashMap,
    fmt::Display,
    io::{self, BufRead},
    iter::zip,
    num::ParseIntError,
    str::FromStr,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    fn new(start: Coord, end: Coord) -> Line {
        Line { start, end }
    }

    fn get_coords(&self) -> Vec<Coord> {
        if self.start.x == self.end.x {
            let y_1 = self.start.y.min(self.end.y);
            let y_2 = self.start.y.max(self.end.y) + 1;

            (y_1..y_2).map(|y| Coord::new(self.start.x, y)).collect()
        } else if self.start.y == self.end.y {
            let x_1 = self.start.x.min(self.end.x);
            let x_2 = self.start.x.max(self.end.x) + 1;

            (x_1..x_2).map(|x| Coord::new(x, self.start.y)).collect()
        } else {
            let x_1 = self.start.x.min(self.end.x);
            let x_2 = self.start.x.max(self.end.x) + 1;
            let x_range = x_1..x_2;

            let y_1 = self.start.y.min(self.end.y);
            let y_2 = self.start.y.max(self.end.y) + 1;
            let y_range = y_1..y_2;

            if x_1 == self.start.x && y_1 == self.start.y {
                zip(x_range, y_range)
                    .map(|(x, y)| Coord::new(x, y))
                    .collect()
            } else if x_1 == self.start.x && y_1 == self.end.y {
                zip(x_range, y_range.rev())
                    .map(|(x, y)| Coord::new(x, y))
                    .collect()
            } else if x_1 == self.end.x && y_1 == self.start.y {
                zip(x_range, y_range.rev())
                    .map(|(x, y)| Coord::new(x, y))
                    .collect()
            } else if x_1 == self.end.x && y_1 == self.end.y {
                zip(x_range, y_range)
                    .map(|(x, y)| Coord::new(x, y))
                    .collect()
            } else {
                panic!("No more possibilities")
            }
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)?;

        Ok(())
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start, self.end)?;

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Coords(Vec<Coord>);

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        self.0.iter().for_each(|coord| {
            write!(f, "{}, ", coord).unwrap();
        });

        write!(f, "]")?;

        Ok(())
    }
}

impl Eq for Coord {}

impl Eq for Line {}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(",");

        let x = i32::from_str_radix(split.next().unwrap(), 10)?;
        let y = i32::from_str_radix(split.next().unwrap(), 10)?;

        Ok(Coord::new(x, y))
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.trim().split(" -> ");

        let start = Coord::from_str(split.next().unwrap())?;
        let end = Coord::from_str(split.next().unwrap())?;

        Ok(Line::new(start, end))
    }
}

pub fn part_1(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut visited: HashMap<Coord, i32> = HashMap::new();
    let lines: Vec<Line> = input
        .lines()
        .map(|s| Line::from_str(&s.unwrap()).unwrap())
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect();

    for line in lines {
        let coords = line.get_coords();

        println!("Line {} has coords {}", line, Coords(coords.clone()));

        coords.iter().for_each(|coord| {
            if visited.contains_key(&coord) {
                println!("Found an overlap: {}", coord);
                visited.insert(*coord, visited[coord] + 1);
            } else {
                visited.insert(*coord, 1);
            }
        })
    }

    let overlaps = visited.iter().filter(|(_, v)| **v > 1).count();

    Ok(overlaps as i32)
}

pub fn part_2(input: &mut dyn BufRead) -> io::Result<i32> {
    let mut visited: HashMap<Coord, i32> = HashMap::new();
    let lines: Vec<Line> = input
        .lines()
        .map(|s| Line::from_str(&s.unwrap()).unwrap())
        .collect();

    for line in lines {
        let coords = line.get_coords();

        println!("Line {} has coords {}", line, Coords(coords.clone()));

        coords.iter().for_each(|coord| {
            if visited.contains_key(&coord) {
                println!("Found an overlap: {:?}", coord);
                visited.insert(*coord, visited[coord] + 1);
            } else {
                visited.insert(*coord, 1);
            }
        })
    }

    let overlaps = visited.iter().filter(|(_, v)| **v > 1).count();

    Ok(overlaps as i32)
}
