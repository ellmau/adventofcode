use std::{fs::File, io::BufRead, path::Path};

fn main() {
    let args = std::env::args();
    let filename = args.last().unwrap();
    let file = File::open(Path::new(&filename)).unwrap();
    let buf_reader = std::io::BufReader::new(file);

    let mut last: Option<usize> = None;
    let mut counter: usize = 0;
    for line in buf_reader.lines() {
        let current = line.unwrap().parse::<usize>().unwrap();
        match last {
            Some(val) => {
                if current > val {
                    counter += 1;
                }
            }
            None => {}
        }
        last = Some(current);
    }

    println!("{} increasements", counter);

    let file = File::open(Path::new(&filename)).unwrap();
    let buf_reader = std::io::BufReader::new(file);
    let numbers: Vec<usize> = buf_reader
        .lines()
        .map(|val| val.unwrap().parse::<usize>().unwrap())
        .collect();

    last = None;
    counter = 0;
    for i in 0..numbers.len() - 2 {
        let current = numbers[i] + numbers[i + 1] + numbers[i + 2];
        match last {
            Some(val) => {
                if current > val {
                    counter += 1;
                }
            }
            None => {}
        }
        last = Some(current);
    }

    println!("{} sliding window increasements", counter);
}
