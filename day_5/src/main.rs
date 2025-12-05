use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug)]
struct IdRange(usize, usize);

#[derive(Default)]
struct Database {
    ids: Vec<IdRange>,
}

impl Database {
    pub fn add_id_range(&mut self, from: usize, to: usize) {
        self.ids.push(IdRange(from, to));
        self.ids.sort_by(|a, b| a.0.cmp(&b.0));

        let mut merged: Vec<IdRange> = Vec::new();

        for r in self.ids.drain(..) {
            if let Some(last) = merged.last_mut() {
                if r.0 <= last.1 {
                    last.1 = last.1.max(r.1);
                } else {
                    merged.push(r);
                }
            } else {
                merged.push(r);
            }
        }

        self.ids = merged;
    }

    pub fn is_fresh(&self, id: usize) -> bool {
        self.find_range(id).is_some()
    }

    pub fn find_range(&self, id: usize) -> Option<IdRange> {
        for ids in self.ids.iter() {
            if id < ids.0 {
                continue;
            }
            if id > ids.1 {
                continue;
            }
            return Some(*ids);
        }
        None
    }

    pub fn fresh_ranges(&mut self) -> usize {
        let mut counter = 0;
        self.ids.sort_by(|a, b| a.0.cmp(&b.0));

        for ids in self.ids.iter() {
            let count = ids.0.abs_diff(ids.1) + 1;
            counter += count;
        }

        return counter;
    }
}

fn parse_range_and_add(database: &mut Database, line: &str) {
    let ids = line
        .split('-')
        .into_iter()
        .map(|id| usize::from_str_radix(id, 10))
        .flatten()
        .collect::<Vec<_>>();

    debug_assert_eq!(ids.iter().count(), 2);

    database.add_id_range(ids[0], ids[1]);
}

fn is_fresh(database: &Database, line: &str) -> bool {
    let id = usize::from_str_radix(line, 10).expect("should just be a number");
    database.is_fresh(id)
}

fn part1() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut database = Database::default();

    let mut should_insert = true;
    let mut fresh_counter = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            should_insert = false;
            continue;
        }

        if should_insert {
            parse_range_and_add(&mut database, &line);
            continue;
        }

        if is_fresh(&database, &line) {
            fresh_counter += 1;
        }
    }

    println!("Fresh ingredients {fresh_counter}");
}

fn part2() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);
    let mut database = Database::default();

    let mut should_insert = true;
    for line in reader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            should_insert = false;
            continue;
        }

        if should_insert {
            parse_range_and_add(&mut database, &line);
            continue;
        }
        break;
    }

    println!("Fresh ingredient ranges {}", database.fresh_ranges());
}

fn main() {
    part1();
    part2();
}
