use std::{borrow::Borrow, io::BufRead};

fn main() {
    let input = read_file();

    let mut dist: usize = 0;
    let mut depth: usize = 0;
    for cmd in &input {
        match cmd {
            Command::Forward(val) => {
                dist += val;
            }
            Command::Up(val) => {
                depth -= val;
            }
            Command::Down(val) => {
                depth += val;
            }
        }
    }
    println!("dist: {} depth: {} goal: {}", dist, depth, dist * depth);

    let mut dist: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    for cmd in &input {
        match cmd {
            Command::Forward(val) => {
                dist += val;
                depth += val * aim;
            }
            Command::Up(val) => {
                aim -= val;
            }
            Command::Down(val) => {
                aim += val;
            }
        }
    }
    println!("dist: {} depth: {} goal: {}", dist, depth, dist * depth);
}

#[derive(Copy, Clone)]
enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

fn read_file() -> Vec<Command> {
    let inputfile = std::env::args().last().unwrap();

    let buf_reader =
        std::io::BufReader::new(std::fs::File::open(std::path::Path::new(&inputfile)).unwrap());
    let mut result: Vec<Command> = Vec::new();

    for line in buf_reader.lines().flatten() {
        if line.starts_with("forward ") {
            result.push(Command::Forward(line[8..].parse::<usize>().unwrap()));
        } else if line.starts_with("up ") {
            result.push(Command::Up(line[3..].parse::<usize>().unwrap()));
        } else if line.starts_with("down ") {
            result.push(Command::Down(line[5..].parse::<usize>().unwrap()));
        }
    }
    result
}
