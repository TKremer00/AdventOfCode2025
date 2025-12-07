use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

pub struct TachyonManifold {
    input: Vec<Vec<char>>,
}

impl TachyonManifold {
    pub fn get_sum(&mut self) -> (usize, usize) {
        const SPLIT_CHARACTER: char = '^';
        let start_column = self.get_start_column().expect("should always have a start");
        let mut beam_positions = HashMap::<usize, usize>::new();
        beam_positions.insert(start_column, 1);

        let mut splits = 0;
        for columns in &self.input[1..] {
            let column_count = columns.iter().count();
            let mut next_positions = HashMap::default();

            for (beam_position, &unique_beams) in &beam_positions {
                let current_column_character = columns[*beam_position as usize];
                if current_column_character != SPLIT_CHARACTER {
                    *next_positions.entry(*beam_position).or_insert(0) += unique_beams;
                    continue;
                }

                splits += 1;

                let left_position = beam_position.wrapping_sub(1);
                if left_position < column_count {
                    *next_positions.entry(left_position).or_insert(0) += unique_beams;
                }

                let right_position = beam_position.wrapping_add(1);
                if right_position < column_count {
                    *next_positions.entry(right_position).or_insert(0) += unique_beams;
                }
            }
            beam_positions = next_positions;
        }

        let total_beams = beam_positions.values().sum();
        return (splits, total_beams);
    }

    fn get_start_column(&self) -> Option<usize> {
        const START_CHARACTER: char = 'S';
        let start_line = self
            .input
            .iter()
            .filter(|line| line.contains(&START_CHARACTER))
            .collect::<Vec<_>>();
        debug_assert_eq!(start_line.iter().count(), 1);
        start_line[0].iter().position(|c| *c == START_CHARACTER)
    }
}

fn part1() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let input = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut manifold = TachyonManifold { input };
    let sum = manifold.get_sum().0;
    println!("Sum = {sum}");
}

fn part2() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let input = reader
        .lines()
        .flatten()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut manifold = TachyonManifold { input };
    let result = manifold.get_sum();
    println!("Sum = {}, Beams = {}", result.0, result.1);
}

fn main() {
    part1();
    part2();
}
