use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Dial {
    current_index: usize,
    num_of_zeros: usize,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left(usize),
    Right(usize),
}

impl Default for Dial {
    fn default() -> Self {
        Self {
            current_index: 50,
            num_of_zeros: 0,
        }
    }
}

impl Dial {
    pub fn rotate(&mut self, direction: Direction) {
        match direction {
            Direction::Left(subtraction) => self.subtract(subtraction),
            Direction::Right(addition) => self.add(addition),
        }
    }

    fn subtract(&mut self, mut subtraction: usize) {
        while subtraction != 0 {
            subtraction -= 1;
            if self.current_index == 0 {
                self.current_index = 99;
                continue;
            }
            self.current_index -= 1;

            if self.current_index == 0 {
                self.num_of_zeros += 1;
            }
        }
    }

    fn add(&mut self, mut addition: usize) {
        while addition != 0 {
            addition -= 1;
            if self.current_index == 99 {
                self.num_of_zeros += 1;
                self.current_index = 0;
                continue;
            }
            self.current_index += 1;
        }
    }

    pub fn get_current_index(&self) -> usize {
        self.current_index
    }
    pub fn get_num_of_zeros(&self) -> usize {
        self.num_of_zeros
    }
}

fn parse_input(input: String) -> Direction {
    let (first_character, rest) = input.split_at(1);

    match first_character {
        "L" => Direction::Left(rest.parse::<usize>().expect("Format should be L10")),
        "R" => Direction::Right(rest.parse::<usize>().expect("Format should be R10")),
        _ => unreachable!("Should not get anything else than a 'L' or a 'R'"),
    }
}

fn test() {
    let inputs = [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    let mut dial = Dial::default();
    for input in inputs {
        let input = parse_input(input.to_string());

        dial.rotate(input);
    }
    let count = dial.get_num_of_zeros();
    println!("Password = {count}");
}

fn part1() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut dial = Dial::default();
    let mut count = 0;
    for input in reader.lines() {
        let input = input.unwrap();
        let input = parse_input(input.to_string());

        dial.rotate(input);
        let current_index = dial.get_current_index();

        if current_index == 0 {
            count += 1;
        }
    }

    println!("Password = {count}");
}

fn part2() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut dial = Dial::default();
    for input in reader.lines() {
        let input = input.unwrap();
        let input = parse_input(input.to_string());

        dial.rotate(input);
        let count = dial.get_num_of_zeros();
    }
    let count = dial.get_num_of_zeros();
    println!("Password = {count}");
}

fn main() {
    part1();
    part2();
}
