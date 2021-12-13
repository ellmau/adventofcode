use std::error::Error;

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
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let (page_desc, instructions) = Parser::parse(&input).unwrap().1;

    println!("{:?}", page_desc);
    let mut page = create_page(&page_desc);
    print_page(&page);
    println!("{:?}", instructions);

    // part 1
    let mut count: usize = 0;
    let newpage = fold_count(&page, &instructions[0], &mut count);
    print_page(&newpage);
    println!("{}", count);

    // part 2
    println!("dimensions: {} {}", page[0].len(), page.len());
    let newpage = instructions.iter().fold(page, |acc, elem| fold(&acc, elem));
    print_page(&newpage);
}

fn create_page(page: &[(usize, usize)]) -> Vec<Vec<bool>> {
    let max = page.iter().fold((0, 0), |acc, elem| {
        (usize::max(acc.0, elem.0), usize::max(acc.1, elem.1))
    });
    let mut result: Vec<Vec<bool>> = vec![vec![false; max.0 + 1]; max.1 + 1];
    page.iter().for_each(|(x, y)| {
        result[*y][*x] = true;
    });
    result
}

fn fold_count(page: &[Vec<bool>], fold: &Fold, count: &mut usize) -> Vec<Vec<bool>> {
    let mut result;
    match fold.axis {
        Axis::X => {
            result = Vec::new();
            page.iter().for_each(|line| {
                let mut new_line: Vec<bool> = Vec::new();
                for (e1, e2) in line[..fold.pos]
                    .iter()
                    .zip(line[fold.pos + 1..].iter().rev())
                {
                    new_line.push(*e1 || *e2);
                    if *e1 || *e2 {
                        *count += 1
                    }
                }
                result.push(new_line);
            })
        }
        Axis::Y => {
            result = Vec::new();

            for (v1, v2) in page[..fold.pos]
                .iter()
                .zip(page[fold.pos + 1..].iter().rev())
            {
                let mut line: Vec<bool> = Vec::new();
                for (e1, e2) in v1.iter().zip(v2.iter()) {
                    line.push(*e1 || *e2);
                    if *e1 || *e2 {
                        *count += 1
                    }
                }
                result.push(line)
            }
        }
    }
    result
}

fn fold(page: &[Vec<bool>], fold: &Fold) -> Vec<Vec<bool>> {
    let mut result;
    match fold.axis {
        Axis::X => {
            result = Vec::new();
            page.iter().for_each(|line| {
                let mut new_line: Vec<bool> = Vec::new();
                if fold.pos * 2 >= line.len() {}
                for (e1, e2) in line[..fold.pos]
                    .iter()
                    .zip(line[fold.pos + 1..].iter().rev())
                {
                    new_line.push(*e1 || *e2);
                }
                result.push(new_line);
            })
        }
        Axis::Y => {
            result = Vec::new();
            let is_even = page.len() % 2 == 0;

            if is_even || (page.len() - 1) / 2 != fold.pos {
                let mut iter_1 = page[..fold.pos].iter();
                let mut iter_2 = page[fold.pos + 1..].iter();

                loop {
                    let v1 = iter_1.next();
                    let v2 = iter_2.next();
                    let mut line: Vec<bool> = Vec::new();
                    if let (Some(v1), Some(v2)) = (v1, v2) {
                        for (e1, e2) in v1.iter().zip(v2.iter()) {
                            line.push(*e1 || *e2);
                        }
                        result.push(line);
                    } else if let Some(v1) = v1 {
                        for e in v1 {
                            line.push(*e);
                        }
                        result.push(line);
                    } else if let Some(v2) = v2 {
                        for e in v2 {
                            line.push(*e);
                        }
                        result.insert(0, line);
                    } else {
                        break;
                    }
                }
            } else {
                for (v1, v2) in page[..fold.pos]
                    .iter()
                    .zip(page[fold.pos + 1..].iter().rev())
                {
                    let mut line: Vec<bool> = Vec::new();
                    for (e1, e2) in v1.iter().zip(v2.iter()) {
                        line.push(*e1 || *e2);
                    }
                    result.push(line)
                }
            }
        }
    }
    println!("newsize {} {}", result[0].len(), result.len());
    result
}

fn print_page(page: &[Vec<bool>]) {
    for line in page {
        for elem in line {
            if *elem {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    pos: usize,
}

struct Parser {}
impl Parser {
    fn parse(input: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<Fold>)> {
        tuple((
            terminated(Parser::page_description, line_ending),
            many1(terminated(Parser::fold_operation, line_ending)),
        ))(input)
    }

    fn decimal_value(input: &str) -> IResult<&str, usize> {
        map_res(Parser::decimal, |val| val.parse::<usize>())(input)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(one_of("0123456789")))(input)
    }

    fn pair(input: &str) -> IResult<&str, (usize, usize)> {
        tuple((
            Parser::decimal_value,
            preceded(tag(","), Parser::decimal_value),
        ))(input)
    }

    fn page_description(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
        many1(terminated(Parser::pair, line_ending))(input)
    }

    fn fold_operation(input: &str) -> IResult<&str, Fold> {
        preceded(tag("fold along "), Parser::fold)(input)
    }

    fn fold(input: &str) -> IResult<&str, Fold> {
        map_res(
            tuple((alpha1, preceded(tag("="), Parser::decimal_value))),
            |(s, d)| -> Result<Fold, ()> {
                println!("{}", s);
                Ok(Fold {
                    pos: d,
                    axis: match s {
                        "y" => Axis::Y,
                        "x" => Axis::X,
                        _ => unreachable!(),
                    },
                })
            },
        )(input)
    }
}
