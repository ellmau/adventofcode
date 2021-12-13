use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{map_res, recognize},
    multi::{many0, many1},
    sequence::terminated,
    IResult,
};

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let (_, edges) = Parser::read_input(&input).unwrap();
    let mut field: Vec<Vec<u8>> = Vec::new();
    init_field(&mut field, &edges);
    fill_field_01(&mut field, &edges);
    //print_field(&field);
    println!(
        "{}",
        field.iter().flatten().fold(0, |acc, elem| -> i32 {
            if *elem > 1 {
                acc + 1
            } else {
                acc
            }
        })
    );

    field = Vec::new();
    init_field(&mut field, &edges);
    fill_field_02(&mut field, &edges);
    //print_field(&field);
    println!(
        "{}",
        field.iter().flatten().fold(0, |acc, elem| -> i32 {
            if *elem > 1 {
                acc + 1
            } else {
                acc
            }
        })
    );
}

fn init_field(field: &mut Vec<Vec<u8>>, edges: &[Edge]) {
    let mut x: usize = 0;
    let mut y: usize = 0;
    for edge in edges {
        x = std::cmp::max(x, edge.max_x());
        y = std::cmp::max(y, edge.max_y());
    }
    field.append(&mut vec![vec![0u8; x + 1]; y + 1]);
}

fn fill_field_01(field: &mut Vec<Vec<u8>>, edges: &[Edge]) {
    for edge in edges {
        if edge.from().x() == edge.to().x() {
            for y in edge.min_y()..edge.max_y() + 1 {
                field[y][edge.from().x()] = std::cmp::min(5, field[y][edge.from().x()] + 1);
            }
        } else if edge.from().y() == edge.to().y() {
            for x in edge.min_x()..edge.max_x() + 1 {
                field[edge.from().y()][x] = std::cmp::min(5, field[edge.from().y()][x] + 1);
            }
        }
    }
}

fn print_field(field: &[Vec<u8>]) {
    for line in field {
        for elem in line {
            if *elem > 0 {
                print!("{}", elem);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fill_field_02(field: &mut Vec<Vec<u8>>, edges: &[Edge]) {
    for edge in edges {
        if edge.from().x() == edge.to().x() {
            for y in edge.min_y()..edge.max_y() + 1 {
                field[y][edge.from().x()] = std::cmp::min(5, field[y][edge.from().x()] + 1);
            }
        } else if edge.from().y() == edge.to().y() {
            for x in edge.min_x()..edge.max_x() + 1 {
                field[edge.from().y()][x] = std::cmp::min(5, field[edge.from().y()][x] + 1);
            }
        } else {
            // diagonal
            for point in edge.get_diag() {
                field[point.y()][point.x()] = std::cmp::min(5, field[point.y()][point.x()] + 1);
            }
        }
    }
}

struct Parser {}

impl Parser {
    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(terminated(
            one_of("0123456789"),
            many0(nom::character::complete::char('_')),
        )))(input)
    }

    fn decimal_value(input: &str) -> IResult<&str, usize> {
        map_res(Self::decimal, |val: &str| val.parse::<usize>())(input)
    }

    fn point(input: &str) -> IResult<&str, Point> {
        let (input, x) = terminated(Self::decimal_value, tag(","))(input)?;
        let (input, y) = Self::decimal_value(input)?;
        Ok((input, Point::new(x, y)))
    }

    fn edge(input: &str) -> IResult<&str, Edge> {
        let (input, from) = terminated(Self::point, tag(" -> "))(input)?;
        let (input, to) = terminated(Self::point, line_ending)(input)?;
        Ok((input, Edge::new(from, to)))
    }

    fn read_input(input: &str) -> IResult<&str, Vec<Edge>> {
        many1(Self::edge)(input)
    }
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn x(self) -> usize {
        self.x
    }

    fn y(self) -> usize {
        self.y
    }
}

#[derive(Debug)]
struct Edge {
    from: Point,
    to: Point,
}

impl Edge {
    fn new(from: Point, to: Point) -> Self {
        Self { from, to }
    }

    fn from(&self) -> Point {
        self.from
    }

    fn to(&self) -> Point {
        self.to
    }

    fn max_x(&self) -> usize {
        std::cmp::max(self.from.x(), self.to.x())
    }

    fn max_y(&self) -> usize {
        std::cmp::max(self.from.y(), self.to.y())
    }

    fn min_x(&self) -> usize {
        std::cmp::min(self.from.x(), self.to.x())
    }

    fn min_y(&self) -> usize {
        std::cmp::min(self.from.y(), self.to.y())
    }

    fn get_diag(&self) -> Vec<Point> {
        let y_shift = self.from.y == self.min_y();
        let x_shift = self.from.x == self.min_x();
        let mut result: Vec<Point> = Vec::new();

        let (mut x, mut y) = (self.from.x, self.from.y);
        while x != self.to.x {
            result.push(Point::new(x, y));
            if x_shift {
                x += 1;
            } else {
                x -= 1;
            }
            if y_shift {
                y += 1;
            } else {
                y -= 1;
            }
        }
        result.push(Point::new(x, y));
        result
    }
}
