use nom::{
    character::complete::{line_ending, multispace0, one_of},
    combinator::{map_res, recognize},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    env_logger::init();
    let input = Parser::parse(&std::fs::read_to_string("input").expect("IO Error occurred"));

    log::info!("fixpoint confirmed at step {}", fp(&input));
}

fn fp(input: &[Vec<u8>]) -> usize {
    let mut counter = 0;
    let mut cur = input.to_vec();
    loop {
        counter += 1;
        if let Some(new) = mv(&cur) {
            cur = new;
        } else {
            break;
        }
    }
    counter
}

fn mv(cur: &[Vec<u8>]) -> Option<Vec<Vec<u8>>> {
    let mut new = vec![vec![0u8; cur[0].len()]; cur.len()];
    let mut change = false;
    cur.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, elem)| match *elem {
            0 => {}
            1 => {
                let next_x = right(cur, x);
                if cur[y][next_x] == 0 {
                    change = true;
                    new[y][next_x] = 1;
                } else {
                    new[y][x] = 1;
                }
            }
            2 => {
                let next_y = down(cur, y);
                if (cur[next_y][x] == 0 && cur[next_y][left(cur, x)] != 1)
                    || (cur[next_y][x] == 1 && cur[next_y][right(cur, x)] == 0)
                {
                    change = true;
                    new[next_y][x] = 2;
                } else {
                    new[y][x] = 2;
                }
            }
            _ => unreachable!(),
        });
    });
    if change {
        Some(new)
    } else {
        None
    }
}

fn right(cur: &[Vec<u8>], x: usize) -> usize {
    if x == cur[0].len() - 1 {
        0
    } else {
        x + 1
    }
}

fn left(cur: &[Vec<u8>], x: usize) -> usize {
    if x == 0 {
        cur[0].len() - 1
    } else {
        x - 1
    }
}

fn down(cur: &[Vec<u8>], y: usize) -> usize {
    if y == cur.len() - 1 {
        0
    } else {
        y + 1
    }
}

struct Parser {}
impl Parser {
    fn symbol(input: &str) -> IResult<&str, u8> {
        map_res(
            recognize(one_of(".>v")),
            |res| -> Result<u8, nom::Err<nom::error::Error<&str>>> {
                match res {
                    "." => Ok(0u8),
                    ">" => Ok(1u8),
                    "v" => Ok(2u8),
                    _ => unreachable!(),
                }
            },
        )(input)
    }

    fn line(input: &str) -> IResult<&str, Vec<u8>> {
        preceded(multispace0, terminated(many1(Parser::symbol), line_ending))(input)
    }

    fn parse(input: &str) -> Vec<Vec<u8>> {
        many1(Parser::line)(input).unwrap().1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    fn input() -> &'static str {
        indoc::indoc! {"
            ...>...
            .......
            ......>
            v.....>
            ......>
            .......
            ..vvv..
        "}
    }
    #[test]
    fn parse() {
        let input = input();

        let input = Parser::parse(input);
        assert_eq!(
            input,
            vec![
                vec![0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 2, 2, 2, 0, 0]
            ]
        );
    }
    #[test]
    fn mv() {
        let input = input();
        let input = Parser::parse(input);

        let after_move = super::mv(&input);
        log::debug!("{:?}", after_move);
        let should_result_in = vec![
            vec![0, 0, 2, 2, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0],
            vec![2, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0, 0],
        ];

        assert_eq!(after_move, Some(should_result_in));
    }

    #[test]
    fn exa1() {
        let input = indoc::indoc! {"
	    v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
	"};

        let input = Parser::parse(input);
        assert_eq!(fp(&input), 58);
    }
}
