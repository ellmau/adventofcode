use std::{ops::Deref, usize};

use nom::{
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let (_, mut fishlist) = fish_list(&input).unwrap();

    let mut fishdays: Vec<usize> = vec![0; 9];

    for fish in &fishlist {
        fishdays[usize::from(fish.0)] += 1;
    }

    for _ in 0..80 {
        let mut newfish: usize = 0;
        for fish in fishlist.iter_mut() {
            newfish += fish.day_passed();
        }
        fishlist.append(&mut vec![Fish(8); newfish]);
    }
    println!("{}", fishlist.len());
    println!("{}", optimised_generations(&mut fishdays.clone(), 80));
    println!("{}", optimised_generations(&mut fishdays.clone(), 256));
}

fn optimised_generations(fishdays: &mut [usize], days: usize) -> usize {
    for _ in 0..days {
        let newfish = fishdays[0];
        for i in 0..6 {
            fishdays[i] = fishdays[i + 1];
        }
        fishdays[6] = fishdays[7] + newfish;
        fishdays[7] = fishdays[8];
        fishdays[8] = newfish;
    }
    fishdays.iter().fold(0, |acc, elem| acc + *elem)
}

#[derive(Clone)]
struct Fish(u8);
impl Deref for Fish {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Fish {
    /**
    tells the fish that a day passed
    Returns how many new fish are produced
    */
    fn day_passed(&mut self) -> usize {
        if self.0 > 0 {
            self.0 -= 1;
            0
        } else {
            self.0 = 6;
            1
        }
    }
}

fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(
        one_of("0123456789"),
        many0(nom::character::complete::char('_')),
    )))(input)
}

fn decimal_value_u8(input: &str) -> IResult<&str, u8> {
    map_res(decimal, |val: &str| val.parse::<u8>())(input)
}

fn fish(input: &str) -> IResult<&str, Fish> {
    let (input, fish) = decimal_value_u8(input)?;
    Ok((input, Fish(fish)))
}

fn fish_list(input: &str) -> IResult<&str, Vec<Fish>> {
    separated_list1(tag(","), fish)(input)
}
