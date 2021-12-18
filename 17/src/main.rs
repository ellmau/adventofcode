use std::ops::RangeInclusive;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{digit1, one_of},
    combinator::{all_consuming, map_res, opt, recognize},
    error::ParseError,
    multi::{many1, many_m_n, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated},
    Finish, IResult,
};
fn main() {
    env_logger::init();

    let target = Target::parse(&std::fs::read_to_string("input").unwrap());

    log::info!("stylish shot has value {}", target.most_value_shot());
    log::info!("Number of shots {}", target.count_shots());
}

#[derive(Debug, Eq, PartialEq)]
struct Shot {
    vertical: isize,
    horizontal: isize,
}

impl Shot {
    fn new(x: isize, y: isize) -> Self {
        Shot {
            vertical: y,
            horizontal: x,
        }
    }

    fn style_value(&self) -> usize {
        let mut result = 0;
        for i in 1..=self.vertical {
            result += i as usize;
        }
        result
    }

    fn hits(
        &self,
        target_x: &RangeInclusive<isize>,
        target_y: &RangeInclusive<isize>,
    ) -> Option<&Self> {
        let mut pos_x = 0;
        let mut pos_y = 0;
        let mut vel_vert = self.vertical;
        let mut vel_hori = self.horizontal;

        loop {
            if pos_x > *target_x.end() || pos_y < *target_y.start() {
                return None;
            }
            pos_x += vel_hori;
            pos_y += vel_vert;

            if vel_hori > 0 {
                vel_hori -= 1;
            } else if vel_hori < 0 {
                vel_hori += 1;
            }

            vel_vert -= 1;

            if target_x.contains(&pos_x) && target_y.contains(&pos_y) {
                return Some(self);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Target {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
}

impl Default for Target {
    fn default() -> Self {
        Self {
            x: RangeInclusive::new(0, 0),
            y: RangeInclusive::new(0, 0),
        }
    }
}

impl Target {
    fn generate_shots(&self) -> impl Iterator<Item = Shot> + '_ {
        let min_x: isize = 1;
        let min_y = *self.y.start();

        let max_x = *self.x.end() + 1;
        let max_y = -*self.y.start();

        (min_y..max_y)
            .into_iter()
            .flat_map(move |y| (min_x..max_x).into_iter().map(move |x| Shot::new(x, y)))
            .filter(|x| x.hits(self.x(), self.y()).is_some())
    }

    fn count_shots(&self) -> usize {
        self.generate_shots().count()
    }

    fn most_value_shot(&self) -> usize {
        self.generate_shots()
            .map(|shot| shot.style_value())
            .max()
            .unwrap()
    }

    fn x(&self) -> &RangeInclusive<isize> {
        &self.x
    }

    fn y(&self) -> &RangeInclusive<isize> {
        &self.y
    }

    fn decimal_value(input: &str) -> IResult<&str, isize> {
        map_res(Target::decimal, |val| val.parse::<isize>())(input)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(pair(opt(tag("-")), many1(one_of("0123456789"))))(input)
    }

    fn range(input: &str) -> IResult<&str, (isize, isize)> {
        separated_pair(Target::decimal_value, tag(".."), Target::decimal_value)(input)
    }

    fn parse(input: &str) -> Self {
        let (_, (x, y)) = preceded(
            tag("target area: "),
            separated_pair(
                preceded(tag("x="), Target::range),
                tag(", "),
                preceded(tag("y="), Target::range),
            ),
        )(input)
        .unwrap();

        Self {
            x: RangeInclusive::new(x.0, x.1),
            y: RangeInclusive::new(y.0, y.1),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use test_log::test;

    #[test]
    fn parse() {
        assert_eq!(
            Target::parse("target area: x=20..30, y=-10..-5"),
            Target {
                x: RangeInclusive::new(20, 30),
                y: RangeInclusive::new(-10, -5)
            }
        );
    }

    #[test]
    fn hitting_shots() {
        let target = Target::parse("target area: x=20..30, y=-10..-5");
        let shot = Shot::new(7, 2);
        assert_eq!(shot.hits(target.x(), target.y()), Some(&shot));
        let shot = Shot::new(6, 3);
        assert_eq!(shot.hits(target.x(), target.y()), Some(&shot));
        let shot = Shot::new(9, 0);
        assert_eq!(shot.hits(target.x(), target.y()), Some(&shot));
        let shot = Shot::new(14, -7);
        assert_eq!(shot.hits(target.x(), target.y()), None);

        target.generate_shots().for_each(|s| {
            log::debug!("{:?}", s);
        });
        assert_eq!(target.most_value_shot(), 45);
        assert_eq!(target.count_shots(), 112);
    }
}
