use nom::{
    character::complete::{line_ending, one_of},
    combinator::recognize,
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("input").expect("file should exist");
    log::info!(
        "{} pixels are lit after 2 optimisation steps",
        part1(&input)
    );

    log::info!(
        "{} pixels are lit after 50 optimisation steps",
        part2(&input)
    );
}

fn part2(input: &str) -> usize {
    let (mapping, mut image) = Parser::parse(input);
    log::debug!("mapping len {}", mapping.len());
    apply_enhancer(&mut image, &mapping, 50);
    score(&image)
}

fn part1(input: &str) -> usize {
    let (mapping, mut image) = Parser::parse(input);
    log::debug!("mapping len {}", mapping.len());
    apply_enhancer(&mut image, &mapping, 2);
    score(&image)
}

fn score(input: &[Vec<u8>]) -> usize {
    input.iter().flatten().filter(|e| **e == 1).count()
}

fn apply_enhancer(input: &mut Vec<Vec<u8>>, mapping: &[u8], steps: usize) {
    let mut uval = 0u8;
    for i in 0..steps {
        if mapping[0] == 1 {
            if i % 2 == 1 {
                uval = 1u8;
            } else {
                uval = 0u8;
            }
        }
        let len = input[0].len();
        input.insert(0, vec![uval; len]);
        input.push(vec![uval; len]);

        for line in input.iter_mut() {
            line.insert(0, uval);
            line.push(uval);
        }
        let inp_cpy = input.clone();
        (0..input.len()).for_each(|y| {
            (0..input[0].len()).for_each(|x| {
                input[y][x] = mapping[window_to_usize(&window(&inp_cpy, (x, y), uval))];
            });
        });
    }
}

fn line_window(input: &[u8], idx: usize, uval: u8) -> Vec<u8> {
    vec![
        *input
            .get(if idx == 0 { usize::MAX } else { idx - 1 })
            .unwrap_or(&uval),
        *input.get(idx).unwrap_or(&uval),
        *input.get(idx + 1).unwrap_or(&uval),
    ]
}

fn window(input: &[Vec<u8>], coord: (usize, usize), uval: u8) -> Vec<Vec<u8>> {
    let (x, y) = coord;
    let len = 3;

    let line_w = |elem| line_window(elem, x, uval);

    vec![
        input
            .get(if y == 0 { usize::MAX } else { y - 1 })
            .map(|elem| line_w(elem))
            .unwrap_or_else(|| vec![uval; len]),
        input
            .get(y)
            .map(|elem| line_w(elem))
            .unwrap_or_else(|| vec![uval; len]),
        input
            .get(y + 1)
            .map(|elem| line_w(elem))
            .unwrap_or_else(|| vec![uval; len]),
    ]
}

fn window_to_usize(input: &[Vec<u8>]) -> usize {
    input
        .iter()
        .flatten()
        .rev()
        .fold((0, 0), |(exp, val), elem| {
            (exp + 1, (*elem as usize) * 2usize.pow(exp) + val)
        })
        .1
}

struct Parser {}

impl Parser {
    fn symbol(input: &str) -> IResult<&str, &str> {
        recognize(one_of("#."))(input)
    }

    fn symbol_value(input: &str) -> IResult<&str, u8> {
        let (inp, res) = Parser::symbol(input)?;

        let result = if res == "#" { 1u8 } else { 0u8 };
        Ok((inp, result))
    }

    fn mapping(input: &str) -> IResult<&str, Vec<u8>> {
        terminated(many1(Parser::symbol_value), line_ending)(input)
    }

    fn image(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
        many1(Parser::mapping)(input)
    }

    fn parse(input: &str) -> (Vec<u8>, Vec<Vec<u8>>) {
        let (inp, maps) = Parser::mapping(input).unwrap();
        (maps, preceded(line_ending, Parser::image)(inp).unwrap().1)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_log::test;

    fn input() -> &'static str {
        indoc! {"
            ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
            
            #..#.
            #....
            ##..#
            ..#..
            ..###
        "}
    }

    #[test]
    fn parse() {
        let input = input();
        let (mapping, image) = Parser::parse(input);
        assert_eq!(mapping.len(), 512);
        assert_eq!(image.len(), 5);
        assert_eq!(image[2], vec![1, 1, 0, 0, 1]);
    }

    #[test]
    fn window() {
        let input = input();
        let (mapping, image) = Parser::parse(input);

        assert_eq!(
            super::window(&image, (2, 2), 0),
            vec![vec![0, 0, 0], vec![1, 0, 0], vec![0, 1, 0u8]]
        );
        assert_eq!(
            super::window(&image, (0, 0), 0),
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 1, 0u8]]
        );
        assert_eq!(
            super::window(&image, (4, 4), 0),
            vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 0, 0u8]]
        );
    }

    #[test]
    fn position_in_map() {
        let input = input();
        let (mapping, image) = Parser::parse(input);

        assert_eq!(window_to_usize(&super::window(&image, (2, 2), 0)), 34);
        assert_eq!(
            mapping[window_to_usize(&super::window(&image, (2, 2), 0))],
            1
        );
        assert_eq!(mapping[70], 0);
    }

    #[test]
    fn apply_enh() {
        let input = input();
        let (mapping, mut image) = Parser::parse(input);

        apply_enhancer(&mut image, &mapping, 1);
        assert!(true);
    }

    #[test]
    fn part1() {
        let input = input();
        let (mapping, mut image) = Parser::parse(input);
        apply_enhancer(&mut image, &mapping, 2);
        assert_eq!(score(&image), 35);

        assert_eq!(super::part1(input), 35);
    }

    #[test]
    fn part2() {
        let input = input();
        let (mapping, mut image) = Parser::parse(input);
        apply_enhancer(&mut image, &mapping, 50);
        assert_eq!(score(&image), 3351);
        assert_eq!(super::part2(input), 3351);
    }
}
