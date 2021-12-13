use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list0},
    sequence::terminated,
    IResult,
};

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let mut parser = Parser::default();
    parser.parse(&input);
    let list = parser.list.clone();
    let hist = parser.position_hist();
    let mut results: Vec<usize> = hist.iter().map(|elem| score(&hist, *elem)).collect();

    let mut minval = usize::MAX;
    let mut minval_g = usize::MAX;
    for i in 0..(parser.max + 1) {
        minval = std::cmp::min(minval, naive_score(&list, i));
        minval_g = std::cmp::min(minval_g, naive_gaussian_score(&list, i));
    }

    println!("{}", minval);
    println!("{}", minval_g);
}

fn score(hist: &[usize], pos: usize) -> usize {
    let mut score: usize = 0;
    for (cur_pos, elem) in hist.iter().enumerate() {
        score += *elem * abs_diff(cur_pos, pos);
    }
    score
}

fn abs_diff(val1: usize, val2: usize) -> usize {
    if val1 > val2 {
        val1 - val2
    } else {
        val2 - val1
    }
}

fn gaussian(val1: usize) -> usize {
    ((val1 * val1) + val1) / 2
}

fn naive_score(list: &[usize], goal: usize) -> usize {
    list.iter().fold(0, |acc, elem| acc + abs_diff(*elem, goal))
}

fn naive_gaussian_score(list: &[usize], goal: usize) -> usize {
    list.iter()
        .fold(0, |acc, elem| acc + gaussian(abs_diff(*elem, goal)))
}

struct Parser {
    max: usize,
    list: Vec<usize>,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            max: 0,
            list: Vec::new(),
        }
    }
}

impl<'a> Parser {
    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(
            one_of("0123456789"),
            many0(nom::character::complete::char('_')),
        )))(input)
    }

    fn decimal_value(input: &str) -> IResult<&str, usize> {
        map_res(Self::decimal, |val: &str| val.parse::<usize>())(input)
    }

    fn position(&'a mut self) -> impl FnMut(&'a str) -> IResult<&str, ()> {
        |input| {
            let (input, val) = Self::decimal_value(input).unwrap();
            self.max = std::cmp::max(self.max, val);
            self.list.push(val);
            Ok((input, {}))
        }
    }

    fn parse(&'a mut self, input: &str) {
        let _ = separated_list0(tag(","), self.position())(input);
    }
}

impl Parser {
    fn position_hist(&self) -> Vec<usize> {
        let mut res: Vec<usize> = vec![0; self.max + 1];

        self.list.iter().for_each(|elem| {
            res[*elem] += 1;
        });
        res
    }
}
