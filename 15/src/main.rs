use std::{collections::HashSet, iter::FromIterator};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::{
        complete::{alpha1, char, line_ending, multispace0, one_of},
        is_digit,
    },
    combinator::{map_res, recognize},
    error::ErrorKind,
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let file_input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let input = Parser::parse(&file_input);
    // dijkstra(&input, (input[0].len() - 1, input.len() - 1));

    // part 2:
    let mut new_input: Vec<Vec<u8>> = new_inp(&input, 0);
    for i in 1..5u8 {
        let mut int_new_input = new_inp(&input, i);
        int_new_input
            .iter()
            .for_each(|line| new_input.push(line.clone()));
    }

    //    printvec(&new_input);

    dijkstra(&new_input, (new_input[0].len() - 1, new_input.len() - 1));
}

fn new_inp(input: &[Vec<u8>], offset: u8) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    for line in input {
        let mut line_res: Vec<u8> = line
            .clone()
            .iter()
            .map(|e| {
                if *e + offset > 9 {
                    (*e + 1 + offset) % 10
                } else {
                    *e + offset
                }
            })
            .collect();
        for i in 1..5u8 {
            line_res.append(
                &mut line
                    .clone()
                    .iter()
                    .map(|elem| {
                        let res = *elem + i + offset;
                        if res > 9 {
                            (res + 1) % 10
                        } else {
                            res
                        }
                    })
                    .collect(),
            );
        }

        result.push(line_res);
    }
    result
}

fn printvec(vec: &[Vec<u8>]) {
    for l in vec {
        for e in l {
            print!("{}", e);
        }
        println!();
    }
}

fn dijkstra(input: &[Vec<u8>], goal: (usize, usize)) {
    let mut visited: HashSet<Pt> = HashSet::new();
    let mut result: Vec<Vec<Node>> = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _element)| Node(usize::MAX, Pt(x, y), None))
                .collect()
        })
        .collect();
    result[0][0].0 = 0;
    while !visited.contains(&Pt(goal.0, goal.1)) {
        let mut updates: Vec<Node> = Vec::new();
        {
            let cur = result
                .iter()
                .flatten()
                .filter(|elem| !visited.contains(&elem.1))
                .min()
                .unwrap();
            for (x, y) in cur.get_surround(goal) {
                if cur.0 + (input[y][x] as usize) < result[y][x].0 {
                    updates.push(Node(
                        cur.0 + (input[y][x] as usize),
                        result[y][x].1.clone(),
                        Some(cur.1.clone()),
                    ));
                }
            }
            visited.insert(cur.1.clone());
        }
        for upd in updates {
            let y = upd.1 .1;
            let x = upd.1 .0;
            result[y][x] = upd;
        }
    }
    println!("{}", result[goal.1][goal.0].0);
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pt(usize, usize);
#[derive(Clone, Debug)]
struct Node(usize, Pt, Option<Pt>);
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl Eq for Node {}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl Node {
    fn get_surround(&self, max: (usize, usize)) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = Vec::new();
        if self.1 .0 > 0 {
            result.push((self.1 .0 - 1, self.1 .1));
        }
        if self.1 .1 > 0 {
            result.push((self.1 .0, self.1 .1 - 1));
        }
        if self.1 .0 < max.0 {
            result.push((self.1 .0 + 1, self.1 .1));
        }
        if self.1 .1 < max.1 {
            result.push((self.1 .0, self.1 .1 + 1));
        }
        result
    }
}

impl FromIterator<Node> for Node {
    fn from_iter<T: IntoIterator<Item = Node>>(iter: T) -> Self {
        todo!()
    }
}
struct Parser {}

impl Parser {
    fn digit_value(input: &str) -> IResult<&str, u8> {
        map_res(Parser::digit, |val| val.parse::<u8>())(input)
    }

    fn digit(input: &str) -> IResult<&str, &str> {
        recognize(one_of("0123456789"))(input)
    }

    fn parse(input: &str) -> Vec<Vec<u8>> {
        many1(terminated(many1(Parser::digit_value), line_ending))(input)
            .unwrap()
            .1
    }
}
