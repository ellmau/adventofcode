use std::io::BufRead;

fn main() {
    let file = std::fs::File::open(std::env::args().last().unwrap()).unwrap();
    let buffer = std::io::BufReader::new(file);

    let result = buffer.lines().fold(0, |acc: usize, elem| {
        let result = parse_line(elem.unwrap().as_str())
            .value_invalid()
            .unwrap_or(0);
        acc + result
    });

    println!("{}", result);

    let file = std::fs::File::open(std::env::args().last().unwrap()).unwrap();
    let buffer = std::io::BufReader::new(file);

    let mut resultlist: Vec<usize> = buffer
        .lines()
        .filter_map(|elem| parse_line(elem.unwrap().as_str()).value_incomplete())
        .collect();
    resultlist.sort_unstable();

    let index = (resultlist.len() - 1) / 2;
    let result = resultlist[index];

    println!("{}", result);
}

enum Result {
    Invalid(usize),
    Incomplete(usize),
    Ok,
}

impl Result {
    fn value(&self) -> Option<usize> {
        match self {
            Result::Invalid(val) | Result::Incomplete(val) => Some(*val),
            Result::Ok => None,
        }
    }

    fn value_invalid(&self) -> Option<usize> {
        match self {
            Result::Invalid(val) => Some(*val),
            Result::Incomplete(_) => None,
            Result::Ok => None,
        }
    }

    fn value_incomplete(&self) -> Option<usize> {
        match self {
            Result::Invalid(_) => None,
            Result::Incomplete(v) => Some(*v),
            Result::Ok => None,
        }
    }
}

fn parse_line(input: &str) -> Result {
    let mut list: Vec<char> = Vec::with_capacity(input.len());
    for i in input.chars() {
        match i {
            '(' | '[' | '<' | '{' => {
                list.push(i);
            }
            ')' | ']' | '>' | '}' => match list.pop() {
                Some(last) => {
                    if !compparens(last, i) {
                        return Result::Invalid(penalty(i));
                    }
                }
                None => {
                    return Result::Invalid(penalty(i));
                }
            },
            _ => unreachable!(),
        }
    }
    if list.is_empty() {
        Result::Ok
    } else {
        list.reverse();
        Result::Incomplete(autocomplete_penalty(&list))
    }
}

fn compparens(l: char, r: char) -> bool {
    (l == '(' && r == ')')
        || (l == '[' && r == ']')
        || (l == '{' && r == '}')
        || (l == '<' && r == '>')
}

fn penalty(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn complete_penalty(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

fn autocomplete_penalty(list: &[char]) -> usize {
    list.iter()
        .fold(0, |acc, elem| (acc * 5) + complete_penalty(*elem))
}
