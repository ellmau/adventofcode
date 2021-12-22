use std::iter::FromIterator;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
fn main() {
    env_logger::init();
    let mut input = Parser::parse(&std::fs::read_to_string("input").expect("file io issue"));
    let input = part_1(&mut input);

    log::info!(
        "There are {} Cubes active on the 50x50x50 field.",
        active_cubes(&create_cubefield(&input))
    );

    let input = Parser::parse(&std::fs::read_to_string("input").expect("file io issue"));

    log::info!(
        "There are {} Cubes active on the big field.",
        active_cubes(&create_cubefield(&input))
    );
}

fn create_cubefield(input: &[Cuboid]) -> Vec<Cuboid> {
    log::trace!("input size: {}", input.len());
    let mut result: Vec<Cuboid> = Vec::new();
    input.iter().for_each(|operation| {
        let mut new_result: Vec<Cuboid> = Vec::new();
        result.iter().for_each(|on_cube| {
            new_result.append(&mut on_cube.remove(operation));
        });
        result = new_result;
        if operation.on {
            result.push(*operation);
        }
    });
    result
}

fn part_1(input: &mut Vec<Cuboid>) -> Vec<Cuboid> {
    input
        .iter_mut()
        .filter_map(|elem| {
            elem.to_part1();
            if elem.valid() {
                Some(*elem)
            } else {
                None
            }
        })
        .collect()
}

fn active_cubes(input: &[Cuboid]) -> usize {
    input.iter().fold(0, |acc, elem| {
        log::trace!("{} + {} = {}", acc, elem.size(), acc + elem.size());
        acc + elem.size()
    })
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range(isize, isize);

impl Range {
    fn lower(&self) -> isize {
        self.0
    }

    fn upper(&self) -> isize {
        self.1
    }

    fn valid(&self) -> bool {
        self.0 < self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
    on: bool,
}

impl Cuboid {
    fn new(x: Range, y: Range, z: Range) -> Self {
        Cuboid { x, y, z, on: true }
    }

    fn on(&self) -> bool {
        self.on
    }

    fn to_part1(&mut self) {
        self.x.0 = self.x.0.max(-50);
        self.x.0 = self.x.0.min(51);
        self.x.1 = self.x.1.max(-50);
        self.x.1 = self.x.1.min(51);
        self.y.0 = self.y.0.max(-50);
        self.y.0 = self.y.0.min(51);
        self.y.1 = self.y.1.max(-50);
        self.y.1 = self.y.1.min(51);
        self.z.0 = self.z.0.max(-50);
        self.z.0 = self.z.0.min(51);
        self.z.1 = self.z.1.max(-50);
        self.z.1 = self.z.1.min(51);
    }
}

impl Cuboid {
    fn overlap(&self, other: &Self) -> bool {
        Self::overlap_range(&self.x, &other.x)
            && Self::overlap_range(&self.y, &other.y)
            && Self::overlap_range(&self.z, &other.z)
    }

    fn overlap_range(r1: &Range, r2: &Range) -> bool {
        r1.lower().max(r2.lower()) <= r1.upper().min(r2.upper())
    }

    fn valid(&self) -> bool {
        self.x.valid() && self.y.valid() && self.z.valid()
    }

    fn size(&self) -> usize {
        ((self.x.1 - self.x.0) * (self.y.1 - self.y.0) * (self.z.1 - self.z.0)) as usize
    }

    fn remove(&self, other: &Self) -> Vec<Self> {
        let mut result = Vec::new();
        if !(self.overlap(other)) {
            return vec![self.clone()];
        }
        (0..=2).for_each(|x| {
            (0..=2).for_each(|y| {
                (0..=2).for_each(|z| {
                    if !(x == 1 && y == 1 && z == 1) {
                        result.push(Cuboid::new(
                            Cuboid::get_slice(&self.x, &other.x, x),
                            Cuboid::get_slice(&self.y, &other.y, y),
                            Cuboid::get_slice(&self.z, &other.z, z),
                        ));
                    }
                });
            });
        });
        result.into_iter().filter(|elem| elem.valid()).collect()
    }

    fn get_slice(r1: &Range, r2: &Range, level: u8) -> Range {
        match level {
            0 => Range(r1.lower(), r1.lower().max(r2.lower())),
            1 => Range(r1.lower().max(r2.lower()), r1.upper().min(r2.upper())),
            2 => Range(r1.upper().min(r2.upper()), r1.upper()),
            _ => Range(0, 0),
        }
    }
}

struct Parser {}
impl Parser {
    fn decimal_value(input: &str) -> IResult<&str, isize> {
        map_res(Parser::decimal, |val| val.parse::<isize>())(input)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(one_of("-0123456789")))(input)
    }

    fn range(input: &str) -> IResult<&str, Range> {
        map_res(
            separated_pair(Parser::decimal_value, tag(".."), Parser::decimal_value),
            |(lower, upper)| -> Result<Range, nom::Err<nom::error::Error<&str>>> {
                Ok(Range(lower, upper + 1))
            },
        )(input)
    }

    fn on(input: &str) -> IResult<&str, bool> {
        let (inp, result) = alt((tag("on "), tag("off ")))(input)?;
        Ok((inp, result.eq("on ")))
    }

    fn cuboid(input: &str) -> IResult<&str, Cuboid> {
        let (inp, on) = Parser::on(input)?;
        let (inp, x) = terminated(preceded(tag("x="), Parser::range), tag(","))(inp)?;
        let (inp, y) = terminated(preceded(tag("y="), Parser::range), tag(","))(inp)?;
        let (inp, z) = terminated(preceded(tag("z="), Parser::range), line_ending)(inp)?;
        Ok((inp, Cuboid { on, x, y, z }))
    }

    fn parse(input: &str) -> Vec<Cuboid> {
        many1(Parser::cuboid)(input).expect("error while parsing").1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_log::test;

    fn testinput(num: u8) -> &'static str {
        match num {
            0 => indoc! {"
               on x=10..12,y=10..12,z=10..12
               on x=11..13,y=11..13,z=11..13
               off x=9..11,y=9..11,z=9..11
               on x=10..10,y=10..10,z=10..10
            "},
            1 => indoc! {"
               on x=-20..26,y=-36..17,z=-47..7
               on x=-20..33,y=-21..23,z=-26..28
               on x=-22..28,y=-29..23,z=-38..16
               on x=-46..7,y=-6..46,z=-50..-1
               on x=-49..1,y=-3..46,z=-24..28
               on x=2..47,y=-22..22,z=-23..27
               on x=-27..23,y=-28..26,z=-21..29
               on x=-39..5,y=-6..47,z=-3..44
               on x=-30..21,y=-8..43,z=-13..34
               on x=-22..26,y=-27..20,z=-29..19
               off x=-48..-32,y=26..41,z=-47..-37
               on x=-12..35,y=6..50,z=-50..-2
               off x=-48..-32,y=-32..-16,z=-15..-5
               on x=-18..26,y=-33..15,z=-7..46
               off x=-40..-22,y=-38..-28,z=23..41
               on x=-16..35,y=-41..10,z=-47..6
               off x=-32..-23,y=11..30,z=-14..3
               on x=-49..-5,y=-3..45,z=-29..18
               off x=18..30,y=-20..-8,z=-3..13
               on x=-41..9,y=-7..43,z=-33..15
               on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
               on x=967..23432,y=45373..81175,z=27513..53682
            "},
            _ => unreachable!("wrong testinput number"),
        }
    }

    #[test]
    fn parse() {
        let input = testinput(0);

        let res = Parser::parse(input);
        assert!(res[0].on);
        assert!(res[1].on);
        assert!(!res[2].on);
        assert!(res[3].on);
        assert_eq!(res[2].z, Range(9, 12));
    }

    #[test]
    fn overlap() {
        let res = Parser::parse(testinput(0));

        assert!(res[0].overlap(&res[1]));

        let res = Parser::parse(testinput(1));

        assert!(!res[18].overlap(&res[19]));
    }

    #[test]
    fn count() {
        let mut input = Parser::parse(testinput(0));

        assert_eq!(active_cubes(&[input[0]]), 27);
        assert_eq!(active_cubes(&[input[1]]), 27);
    }

    #[test]
    fn part1() {
        let mut input = Parser::parse(testinput(0));

        let p1_input = part_1(&mut input);
        assert_eq!(p1_input, input);

        let mut result = create_cubefield(&[p1_input[0], p1_input[1]]);
        let len = result.len();
        result.sort();
        result.dedup();
        assert_eq!(len, result.len());
        log::debug!("{:?}", result);
        result
            .iter()
            .for_each(|f| log::debug!("{:?} => {}", f, f.size()));
        assert_eq!(active_cubes(&result), 46);
        let result = create_cubefield(&p1_input);

        log::debug!("{:?}", result);
        assert_eq!(active_cubes(&result), 39);

        let mut input = Parser::parse(testinput(1));
        let p1_input = part_1(&mut input);
        log::trace!("p1_input: {:?}", p1_input);
        let result = create_cubefield(&p1_input);

        assert_eq!(active_cubes(&result), 590_784);
    }
}
