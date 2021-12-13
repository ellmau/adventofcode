use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of, space0, space1},
    combinator::{map_res, recognize},
    complete::take,
    multi::{many0, many1, separated_list1},
    sequence::{self, preceded, terminated},
    IResult,
};

use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let mut sequences = parse(&input);

    println!(
        "{}",
        sequences
            .iter()
            .fold(0, |acc, elem| { acc + elem.naive_number_count() })
    );
    println!(
        "{}",
        sequences.iter_mut().fold(0, |acc, elem| {
            elem.fill_naive_numbers();
            elem.find_based_on_naive();
            elem.output() + acc
        })
    );
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Segment {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
}

impl Segment {
    fn as_num(&self) -> usize {
        *self as usize
    }

    fn from_num(num: usize) -> Self {
        match num {
            0 => Segment::A,
            1 => Segment::B,
            2 => Segment::C,
            3 => Segment::D,
            4 => Segment::E,
            5 => Segment::F,
            6 => Segment::G,
            _ => unreachable!(),
        }
    }

    fn new_segment(input: &str) -> Self {
        match input {
            "a" => Segment::A,
            "b" => Segment::B,
            "c" => Segment::C,
            "d" => Segment::D,
            "e" => Segment::E,
            "f" => Segment::F,
            "g" => Segment::G,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Digit {
    segments: Vec<Segment>,
}

impl Digit {
    fn new(segments: Vec<Segment>) -> Self {
        let mut result = Self { segments };
        result.segments.sort();
        result
    }

    fn naive_number(&self) -> Option<usize> {
        match self.segments.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn compareSegs(&self, other: &Self) -> usize {
        let left: HashSet<Segment> = HashSet::from_iter(self.segments.iter().cloned());
        let right: HashSet<Segment> = HashSet::from_iter(other.segments.iter().cloned());

        left.intersection(&right).count()
    }

    fn len(&self) -> usize {
        self.segments.len()
    }
}

#[derive(Debug)]
struct InputSequence {
    sequence: Vec<Digit>,
    outputindex: usize,
    digit_to_nbr: Vec<Option<usize>>,
    nbr_to_digit: Vec<Option<usize>>,
}

impl InputSequence {
    fn new(warmup: Vec<Digit>, output: Vec<Digit>) -> Self {
        let outputindex = warmup.len();
        let sequence: Vec<Digit> = warmup
            .iter()
            .cloned()
            .chain(output.iter().cloned())
            .collect();
        let len = sequence.len();
        Self {
            sequence,
            outputindex,
            digit_to_nbr: vec![None; len],
            nbr_to_digit: vec![None; len],
        }
    }

    fn naive_number_count(&self) -> usize {
        self.sequence[self.outputindex..]
            .iter()
            .fold(0, |acc, elem| match elem.naive_number() {
                Some(_) => acc + 1,
                None => acc,
            })
    }

    fn output(&self) -> usize {
        let mut result = String::new();

        self.digit_to_nbr[self.outputindex..]
            .iter()
            .for_each(|elem| {
                result = format!("{}{}", result, elem.unwrap());
            });
        result.parse().unwrap()
    }

    fn fill_naive_numbers(&mut self) {
        for i in 0..self.sequence.len() {
            let naive_nmbr = self.sequence[i].naive_number();
            self.digit_to_nbr[i] = naive_nmbr;
            if naive_nmbr.is_some() {
                self.nbr_to_digit[naive_nmbr.unwrap()] = Some(i);
            }
        }
    }

    fn find_based_on_naive(&mut self) {
        for i in 0..self.sequence.len() {
            match self.sequence[i].len() {
                5 => {
                    if self.sequence[i].compareSegs(&self.sequence[self.nbr_to_digit[7].unwrap()])
                        == 3
                    {
                        Some(3)
                    } else if self.sequence[i]
                        .compareSegs(&self.sequence[self.nbr_to_digit[4].unwrap()])
                        == 3
                    {
                        Some(5)
                    } else {
                        Some(2)
                    }
                }
                6 => {
                    if self.sequence[i].compareSegs(&self.sequence[self.nbr_to_digit[1].unwrap()])
                        == 1
                    {
                        Some(6)
                    } else if self.sequence[i]
                        .compareSegs(&self.sequence[self.nbr_to_digit[4].unwrap()])
                        == 4
                    {
                        Some(9)
                    } else {
                        Some(0)
                    }
                }
                _ => None,
            }
            .and_then(|value| {
                self.digit_to_nbr[i] = Some(value);
                self.nbr_to_digit[value] = Some(i);
                Some(value)
            });
        }
    }
}

fn segment(input: &str) -> IResult<&str, Segment> {
    let (input, symbol) = nom::branch::alt((
        tag("a"),
        tag("b"),
        tag("c"),
        tag("d"),
        tag("e"),
        tag("f"),
        tag("g"),
    ))(input)?;
    Ok((input, Segment::new_segment(symbol)))
}

fn digit(input: &str) -> IResult<&str, Digit> {
    let (input, digit) = terminated(many1(segment), space0)(input)?;
    Ok((input, Digit::new(digit)))
}

fn input_sequence(input: &str) -> IResult<&str, InputSequence> {
    let (input, warmup) = many1(digit)(input)?;
    let (input, output) = preceded(tag("| "), terminated(many1(digit), line_ending))(input)?;
    Ok((input, InputSequence::new(warmup, output)))
}

fn parse(input: &str) -> Vec<InputSequence> {
    many1(input_sequence)(input).unwrap().1
}
#[cfg(test)]
mod test {
    use super::*;
    use nom::IResult;

    #[test]
    fn test() {
        let segments = vec![Segment::A, Segment::B, Segment::D];

        assert_eq!(Digit { segments }, digit("abd").unwrap().1);
    }
}
