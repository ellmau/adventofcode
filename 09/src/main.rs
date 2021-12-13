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

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let topography = parse(&input);
    println!("{}", score_part1(&topography));
    println!("{}", score_part2(&topography));
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn left(&self) -> Option<Point> {
        if self.x > 0 {
            Some(Point {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn right(&self, max: usize) -> Option<Point> {
        if self.x + 1 < max {
            Some(Point {
                x: self.x + 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    fn up(&self) -> Option<Point> {
        if self.y > 0 {
            Some(Point {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    fn down(&self, max: usize) -> Option<Point> {
        if self.y + 1 < max {
            Some(Point {
                x: self.x,
                y: self.y + 1,
            })
        } else {
            None
        }
    }

    fn get_basin_size(&self, matrix: &[Vec<usize>], visited: &mut HashSet<Point>) -> usize {
        let dirs: Vec<Option<Point>> = vec![
            self.left(),
            self.right(matrix[0].len()),
            self.up(),
            self.down(matrix.len()),
        ];
        // let dir_vals: Vec<Option<usize>> = dirs
        //     .iter()
        //     .map(|elem| elem.map(|val| val.get_val(matrix)))
        //     .collect();
        let mut result = 1;
        visited.insert(*self);
        dirs.iter().for_each(|elem| {
            if elem.is_some() {
                if elem.map(|val| val.get_val(matrix)).unwrap() < 9
                    && !visited.contains(&elem.unwrap())
                {
                    result += elem.unwrap().get_basin_size(matrix, visited);
                }
            }
        });
        result
    }

    fn get_val(&self, matrix: &[Vec<usize>]) -> usize {
        matrix[self.y][self.x]
    }
}

fn score_part2(matrix: &[Vec<usize>]) -> usize {
    let mut basins: Vec<usize> = naive_minima(matrix)
        .iter()
        .map(|val| {
            let mut visited: HashSet<Point> = HashSet::new();
            val.get_basin_size(matrix, &mut visited)
        })
        .collect();

    basins.sort_unstable();
    basins.reverse();
    basins[0] * basins[1] * basins[2]
}

fn score_part1(matrix: &[Vec<usize>]) -> usize {
    naive_minima(matrix)
        .iter()
        .fold(0, |acc, elem| acc + matrix[elem.y][elem.x] + 1)
}

fn naive_minima(matrix: &[Vec<usize>]) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        for (j, elem) in line.iter().enumerate() {
            let point = Point { x: j, y: i };
            if check_pos_min(&point, matrix) {
                result.push(point);
            }
        }
    }
    result
}

fn check_pos_min(pos: &Point, matrix: &[Vec<usize>]) -> bool {
    let elem = matrix[pos.y][pos.x];
    let mut result: bool = true;
    if pos.x > 0 {
        if matrix[pos.y][pos.x - 1] <= elem {
            result = false;
        }
    }
    if pos.x + 1 < matrix[pos.y].len() {
        if matrix[pos.y][pos.x + 1] <= elem {
            result = false;
        }
    }
    if pos.y > 0 {
        if matrix[pos.y - 1][pos.x] <= elem {
            result = false;
        }
    }
    if pos.y + 1 < matrix.len() {
        if matrix[pos.y + 1][pos.x] <= elem {
            result = false;
        }
    }
    result
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(
        one_of("0123456789"),
        many0(nom::character::complete::char('_')),
    )))(input)
}

fn decimal_value(input: &str) -> IResult<&str, usize> {
    map_res(decimal, |val: &str| val.parse::<usize>())(input)
}

fn digit_value(input: &str) -> IResult<&str, usize> {
    map_res(digit, |val| val.parse::<usize>())(input)
}

fn digit(input: &str) -> IResult<&str, &str> {
    recognize(one_of("0123456789"))(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<usize>> {
    terminated(many1(digit_value), line_ending)(input)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let (inp, result) = many1(parse_line)(input).unwrap();
    result
}
