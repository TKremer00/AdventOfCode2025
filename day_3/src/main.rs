use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_part2_max(line: &str, digit_length: usize, current: &str) -> Option<String> {
    if digit_length > line.len() {
        return None;
    }

    if digit_length == 0 {
        return Some(current.to_string());
    }

    if digit_length == 1 {
        if let Some(max_ch) = line.chars().max() {
            return Some(format!("{}{}", current, max_ch));
        } else {
            return None;
        }
    }

    for c in "987654321".chars() {
        if let Some(index) = line.find(c) {
            let substr = &line[index + 1..];

            if let Some(result) =
                get_part2_max(substr, digit_length - 1, &format!("{}{}", current, c))
            {
                return Some(result);
            }
        }
    }

    unreachable!();
}

fn get_part1_max(line: &str) -> usize {
    let chars = line[..line.len() - 1].chars().collect::<Vec<_>>();

    let mut highest_digit = char::MIN;
    let mut seccond_highest = char::MIN;

    for (i, digit) in chars.iter().enumerate() {
        if highest_digit >= *digit {
            continue;
        }

        highest_digit = *digit;
        seccond_highest = char::MIN;

        let remaining_line = &line[i + 1..];
        let remaining = remaining_line.chars();
        for other_digit in remaining {
            if other_digit < seccond_highest {
                continue;
            }
            seccond_highest = other_digit;
        }
    }

    let result = format!("{highest_digit}{seccond_highest}");
    return result.parse().unwrap();
}

fn part1() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for input in reader.lines() {
        let input = input.unwrap();
        sum += get_part1_max(&input);
    }

    println!("Result {sum}");
}

fn part2() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut sum = 0;
    for input in reader.lines() {
        let input = input.unwrap();
        let result = get_part2_max(&input, 12, "").unwrap();
        sum += result.parse::<usize>().unwrap();
    }

    println!("Result {sum}");
}

fn main() {
    part1();
    part2();
}
