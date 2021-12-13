use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_till, take_until, take_while},
    character::complete::{digit0, line_ending, multispace1, one_of, space1},
    combinator::{map_res, recognize},
    complete::take,
    multi::{many0, many1, many_m_n, separated_list0},
    number,
    sequence::{self, preceded, terminated},
    IResult,
};

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let (x, (mut input, mut boards)) = parse_input(&input).unwrap();
    let mut last: Option<u64> = None;
    let mut first: Option<u64> = None;

    for elem in input {
        let mut brem: Option<usize> = None;
        for i in 0..boards.len() {
            boards[i].mark_number(elem)
        }
        for (pos, board) in boards.iter_mut().enumerate() {
            match board.get_score(elem) {
                Some(val) => {
                    if first.is_none() {
                        first = Some(val);
                    }
                    brem = Some(pos);
                    if val > 0 {
                        last = Some(val);
                    }
                }
                None => {}
            }
        }
        if brem.is_some() {
            match brem {
                Some(idx) => {
                    boards.remove(idx);
                }
                _ => unreachable!(),
            }
        }
    }
    println!("\n{:?}\n{:?}", first, last);
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(
        one_of("0123456789"),
        many0(nom::character::complete::char('_')),
    )))(input)
}

fn decimal_value(input: &str) -> IResult<&str, u64> {
    map_res(decimal, |val: &str| val.parse::<u64>())(input)
}

fn input_sequence(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(tag(","), decimal_value)(input)
}

fn bingo_line(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list0(many1(space1), decimal_value)(input)
}

fn bingo_line_term(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(many0(space1), terminated(bingo_line, line_ending))(input)
}

fn bingo_field(input: &str) -> IResult<&str, Vec<Vec<u64>>> {
    many_m_n(5, 5, bingo_line_term)(input)
}

fn bingo_board(input: &str) -> IResult<&str, BingoBoard> {
    let (inp, field) = preceded(nom::branch::alt((tag("\n\n"), tag("\n"))), bingo_field)(input)?;
    Ok((inp, BingoBoard::new(field)))
}

fn parse_input(input: &str) -> IResult<&str, (Vec<u64>, Vec<BingoBoard>)> {
    let (i, res1) = input_sequence(input)?;
    let (stuff, boards) = many0(bingo_board)(i)?;
    Ok((stuff, (res1, boards)))
}

#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<u64>>,
    marks: Vec<Vec<bool>>,
    bingo: bool,
    scored: bool,
}

impl BingoBoard {
    fn new(board: Vec<Vec<u64>>) -> Self {
        let mut marks: Vec<Vec<bool>> = vec![vec![false; 5]; 5];
        Self {
            board,
            marks,
            bingo: false,
            scored: false,
        }
    }
    fn mark_number(&mut self, number: u64) {
        for (ypos, elem) in self.board.iter().enumerate() {
            for (xpos, elem) in elem.iter().enumerate() {
                if *elem == number {
                    self.marks[ypos][xpos] = true;
                    let mut xmarks = 0;
                    let mut ymarks = 0;
                    for i in 0..self.marks.len() {
                        if self.marks[ypos][i] {
                            ymarks += 1;
                        }
                        if self.marks[i][xpos] {
                            xmarks += 1;
                        }
                        if xmarks == self.marks.len() || ymarks == self.marks.len() {
                            self.bingo = true;
                        }
                    }
                }
            }
        }
    }

    fn get_score(&mut self, last: u64) -> Option<u64> {
        if self.bingo && !self.scored {
            self.scored = true;
            Some(
                self.marks
                    .clone()
                    .into_iter()
                    .flatten()
                    .zip(self.board.clone().into_iter().flatten())
                    .fold(0, |acc, (mark, val)| if !mark { val + acc } else { acc })
                    * last,
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn testinput() {
        assert_eq!(
            input_sequence("12,11,23"),
            Ok(("", vec![12u64, 11u64, 23u64]))
        );
    }

    #[test]
    fn test_file() {
        let input = std::fs::read_to_string("./test2.txt").unwrap();
        println!("{}", input);
        println!("{:?}", input_sequence(&input));
        let (a, b) = input_sequence(&input).unwrap();
        println!("{}", &a[2..]);
        println!("\n\n\n{:?}", parse_input(&input));
    }
}
