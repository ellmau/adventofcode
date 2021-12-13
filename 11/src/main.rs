use std::collections::HashSet;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::{
        complete::{char, line_ending, one_of},
        is_digit,
    },
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::terminated,
    IResult,
};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let mut field = octo_array(&input);
    init_pos(&mut field);
    println!(
        "{}",
        (1..101)
            .into_iter()
            .fold(0, |acc, s| { step(s, &mut field) + acc })
    );

    let mut field = octo_array(&input);
    init_pos(&mut field);
    let mut i: usize = 0;
    loop {
        i += 1;
        if step(i, &mut field) == 100 {
            break;
        }
    }
    println!("{}", i);
}

fn init_pos(field: &mut Vec<Vec<Octopus>>) {
    for (yidx, line) in field.iter_mut().enumerate() {
        for (xidx, val) in line.iter_mut().enumerate() {
            val.set_pos(Point::new(xidx, yidx));
        }
    }
}

/// returns how many flares there have been in this step
fn step(s: usize, field: &mut Vec<Vec<Octopus>>) -> usize {
    let mut to_flare: HashSet<Point> = HashSet::new();
    field.iter_mut().for_each(|line| {
        line.iter_mut().for_each(|octo| {
            if octo.increase() {
                to_flare.insert(octo.pos.clone());
            }
        })
    });
    let mut flared: bool = !to_flare.is_empty();
    while flared {
        let mut increase: Vec<Point> = Vec::new();
        to_flare
            .into_iter()
            .for_each(|pt| increase.append(&mut field[pt.y][pt.x].flare_at(s)));
        to_flare = HashSet::new();
        increase.iter_mut().for_each(|octo| {
            if field[octo.y][octo.x].increase() {
                to_flare.insert(octo.clone());
            }
        });
        flared = !to_flare.is_empty();
    }
    field.iter_mut().fold(0, |acc, line| {
        line.iter_mut().fold(0, |acc_inner, octo| {
            acc_inner + octo.update_level().unwrap_or(0)
        }) + acc
    })
}

fn print_field(field: &[Vec<Octopus>]) {
    for line in field {
        for octo in line {
            print!("{}", octo);
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Octopus {
    level: usize,
    pos: Point,
    last_flare: usize,
}

impl std::fmt::Display for Octopus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.level)
    }
}

impl Octopus {
    fn new(level: usize) -> Self {
        Self {
            level,
            pos: Point::default(),
            last_flare: 0,
        }
    }

    fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    fn increase(&mut self) -> bool {
        self.level += 1;
        self.level == 10
    }

    fn flare_at(&mut self, s: usize) -> Vec<Point> {
        let mut result = Vec::new();
        if self.last_flare != s {
            self.last_flare = s;
            result.append(&mut self.pos.neighbors());
        }
        result
    }

    fn update_level(&mut self) -> Option<usize> {
        if self.level > 9 {
            self.level = 0;
            Some(1)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    neighbor_cache: Option<Vec<Point>>,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            neighbor_cache: None,
        }
    }
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            neighbor_cache: None,
        }
    }

    fn neighbors(&self) -> Vec<Point> {
        if self.neighbor_cache.is_some() {
            return self.neighbor_cache.clone().unwrap();
        }
        let mut result: Vec<Point> = Vec::new();
        let mut xvals: Vec<usize> = Vec::new();
        let mut yvals: Vec<usize> = Vec::new();
        xvals.push(self.x);
        yvals.push(self.y);
        if self.x < 9 {
            xvals.push(self.x + 1);
        }
        if self.x > 0 {
            xvals.push(self.x - 1);
        }
        if self.y < 9 {
            yvals.push(self.y + 1);
        }
        if self.y > 0 {
            yvals.push(self.y - 1);
        }
        xvals
            .iter()
            .cartesian_product(yvals.iter())
            .for_each(|elem| {
                result.push(Point::new(*elem.0, *elem.1));
            });
        result
    }
}

fn digit_value(input: &str) -> IResult<&str, usize> {
    map_res(digit, |val| val.parse::<usize>())(input)
}

fn digit(input: &str) -> IResult<&str, &str> {
    recognize(one_of("0123456789"))(input)
}

fn octopus(input: &str) -> IResult<&str, Octopus> {
    map_res(
        digit_value,
        |val| -> Result<Octopus, nom::error::ErrorKind> { Ok(Octopus::new(val)) },
    )(input)
}

fn octo_line(input: &str) -> IResult<&str, Vec<Octopus>> {
    many1(octopus)(input)
}

fn octo_array(input: &str) -> Vec<Vec<Octopus>> {
    many1(terminated(octo_line, line_ending))(input).unwrap().1
}
