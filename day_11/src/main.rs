use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct Device {
    uid: String,
    outputs: Vec<String>,
}

impl Device {
    pub fn new(uid: String, outputs: Vec<String>) -> Self {
        Self { uid, outputs }
    }
}

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if current == target {
        return 1;
    }

    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total_paths = 0;

    if let Some(neighbors) = graph.get(current) {
        for next in neighbors {
            total_paths += count_paths(graph, next, target, memo);
        }
    }

    memo.insert(current.to_string(), total_paths);
    total_paths
}

fn part1() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut devices = Vec::new();

    for input in reader.lines() {
        let input = input.unwrap();
        let mut uid_and_devices = input.splitn(2, ':');
        let uid = uid_and_devices.next().expect("should be valid format");
        let outputs = uid_and_devices
            .next()
            .expect("should be valid format")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        devices.push(Device::new(uid.to_string(), outputs));
    }

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for d in &devices {
        graph.insert(d.uid.clone(), d.outputs.clone());
    }

    let mut memo1 = HashMap::new();
    let paths = count_paths(&graph, "you", "out", &mut memo1);

    println!("A total of {} paths were found", paths);
}

fn part2() {
    let input_file = File::open("input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut devices = Vec::new();

    for input in reader.lines() {
        let input = input.unwrap();
        let mut uid_and_devices = input.splitn(2, ':');
        let uid = uid_and_devices.next().expect("should be valid format");
        let outputs = uid_and_devices
            .next()
            .expect("should be valid format")
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        devices.push(Device::new(uid.to_string(), outputs));
    }

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for d in &devices {
        graph.insert(d.uid.clone(), d.outputs.clone());
    }

    let mut memo1 = HashMap::new();
    let svr_to_dac = count_paths(&graph, "svr", "dac", &mut memo1);

    let mut memo2 = HashMap::new();
    let dac_to_fft = count_paths(&graph, "dac", "fft", &mut memo2);

    let mut memo3 = HashMap::new();
    let fft_to_out = count_paths(&graph, "fft", "out", &mut memo3);

    let mut memo4 = HashMap::new();
    let svr_to_fft = count_paths(&graph, "svr", "fft", &mut memo4);

    let mut memo5 = HashMap::new();
    let fft_to_dac = count_paths(&graph, "fft", "dac", &mut memo5);

    let mut memo6 = HashMap::new();
    let dac_to_out = count_paths(&graph, "dac", "out", &mut memo6);

    let answer = (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out);
    println!("A total of {} paths were found", answer);
}

fn main() {
    part1();
    part2();
}
