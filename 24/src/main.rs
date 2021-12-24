use std::{cell::RefCell, collections::HashMap};

use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{line_ending, multispace0, multispace1, one_of},
    combinator::{map_res, recognize},
    multi::many1,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

fn main() {
    env_logger::init();
    let program = Parser::parse(&std::fs::read_to_string("input").expect("IO issue"));

    let (min, max) = find_model(&program);
    log::info!("maximal valid id: {}", max);
    log::info!("minimal valid id: {}", min);

    // let mut optimizer = NumberOptimizer::new();
    // let mut result = optimizer.find_model_number_opt(&program, &Alu::new(), 14);
    // let number: Vec<usize> = result.unwrap().into_iter().rev().collect();
    // log::info!("maximal valid model number {:?}", number);
    // log::info!("maximal valid model number {}", find_model_number(&program));
}

fn find_model_number(program: &[Com]) -> usize {
    let mut curval: usize = 100_000_000_000_000;
    let mut alu = Alu::new();
    loop {
        alu.reset();
        curval -= 1;
        log::debug!("number {}", curval);
        let input = number_to_vec(curval);
        if input.is_some() && alu.run_program(program, &input.unwrap()) && alu.register[3] == 0 {
            return curval;
        }
        if curval == 10_000_000_000_000 {
            return 0;
        }
    }
}

fn find_model(program: &[Com]) -> (usize, usize) {
    let mut alus = vec![(Alu::new(), (0usize, 0usize))];
    program.iter().for_each(|prog| match prog {
        Com::Inp(value) => {
            let mut alu_set: Vec<(Alu, (usize, usize))> = Vec::new();
            let mut alu_cache: HashMap<Alu, usize> = HashMap::new();
            alus.iter().for_each(|(alu, (min, max))| {
                (1..10usize).for_each(|digit| {
                    let mut working_alu = alu.clone();
                    let new_min = min * 10 + digit;
                    let new_max = max * 10 + digit;
                    working_alu.command(&Com::Inp(*value), &[digit]);
                    if let Some(idx) = alu_cache.get(&working_alu) {
                        alu_set[*idx].1 .0 = std::cmp::min(alu_set[*idx].1 .0, new_min);
                        alu_set[*idx].1 .1 = std::cmp::max(alu_set[*idx].1 .1, new_max);
                    } else {
                        alu_cache.insert(working_alu.clone(), alu_set.len());
                        alu_set.push((working_alu, (new_min, new_max)));
                    }
                });
            });
            alus = alu_set;
        }
        com => alus.iter_mut().for_each(|(alu, (min, max))| {
            alu.command(com, &[]);
        }),
    });
    let mut min = usize::MAX;
    let mut max = 0;

    alus.iter().for_each(|(alu, (amin, amax))| {
        if alu.register[3] == 0 {
            min = std::cmp::min(min, *amin);
            max = std::cmp::max(max, *amax);
        }
    });

    (min, max)
}

struct NumberOptimizer {
    hash: HashMap<(Vec<Com>, Alu, usize), Option<Vec<usize>>>,
}

impl NumberOptimizer {
    fn new() -> Self {
        Self {
            hash: HashMap::new(),
        }
    }
    fn find_model_number_opt(
        &mut self,
        program: &[Com],
        alu: &Alu,
        digits: usize,
    ) -> Option<Vec<usize>> {
        let inputs = vec![9, 8, 7, 6, 5, 4, 3, 2, 1usize];
        let mut working_alu;
        if digits == 1 {
            for digit in inputs {
                working_alu = alu.clone();
                if working_alu.run_one_inp(program, &[digit]).is_some()
                    && working_alu.register[3] == 0
                {
                    return Some(vec![digit]);
                }
            }
        } else {
            for digit in inputs {
                working_alu = alu.clone();

                let mut query_result = self.hash.get(&(program.to_vec(), alu.clone(), digit));
                let mut serialresult;
                if let Some(result) = query_result {
                    println!("found result: {:?};{}", result, self.hash.len());
                    serialresult = result.clone();
                } else {
                    serialresult = match working_alu.run_one_inp(program, &[digit]) {
                        Some(remain) => {
                            self.find_model_number_opt(remain, &working_alu, digits - 1)
                        }
                        None => None,
                    };
                    self.hash
                        .insert((program.to_vec(), alu.clone(), digit), serialresult.clone());
                }

                if let Some(mut serial) = serialresult {
                    serial.push(digit);
                    return Some(serial.clone());
                }
            }
        }
        None
    }
}

fn number_to_vec(n: usize) -> Option<Vec<usize>> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        let digit = n % 10;
        if n == 0 {
            return None;
        }
        digits.push(digit);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    Some(digits)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Alu {
    register: Vec<isize>,
}

impl Alu {
    fn new() -> Self {
        Self {
            register: vec![0; 4],
        }
    }

    fn reset(&mut self) {
        self.register = vec![0; 4];
    }

    fn command<'a>(&mut self, command: &Com, inp: &'a [usize]) -> Option<&'a [usize]> {
        match command {
            Com::Inp(reg) => {
                self.register[reg.value()] = inp[0] as isize;
                return Some(&inp[1..]);
            }
            Com::Add(reg, val) => self.register[reg.value()] += val.value(&self.register),
            Com::Mul(reg, val) => self.register[reg.value()] *= val.value(&self.register),
            Com::Div(reg, val) => {
                let val_value = val.value(&self.register);
                if val_value == 0 {
                    return None;
                }
                self.register[reg.value()] /= val_value;
            }
            Com::Mod(reg, val) => {
                let reg_value = self.register[reg.value()];
                let val_value = val.value(&self.register);
                if reg_value < 0 || val_value <= 0 {
                    return None;
                }
                self.register[reg.value()] %= val.value(&self.register)
            }
            Com::Eql(reg, val) => {
                self.register[reg.value()] =
                    if self.register[reg.value()] == val.value(&self.register) {
                        1
                    } else {
                        0
                    }
            }
        }
        Some(inp)
    }

    fn run_program(&mut self, program: &[Com], inp: &[usize]) -> bool {
        program
            .iter()
            .try_fold(inp, |acc, elem| self.command(elem, acc))
            .is_some()
    }

    fn run_one_inp<'a>(&mut self, program: &'a [Com], inp: &[usize]) -> Option<&'a [Com]> {
        let mut inp_allowed = true;
        for (idx, com) in program.iter().enumerate() {
            if com.is_inp() {
                if !inp_allowed {
                    return Some(&program[idx..]);
                }
                inp_allowed = false;
            }
            self.command(com, inp)?;
        }
        Some(&[])
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ValOrVar {
    Val(isize),
    Var(usize),
}

impl ValOrVar {
    fn value(&self, varlist: &[isize]) -> isize {
        match self {
            ValOrVar::Val(val) => *val,
            ValOrVar::Var(idx) => varlist[*idx],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Var(usize);

impl From<usize> for Var {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl Into<usize> for Var {
    fn into(self) -> usize {
        self.0
    }
}

impl Var {
    fn value(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Com {
    Inp(Var),
    Add(Var, ValOrVar),
    Mul(Var, ValOrVar),
    Div(Var, ValOrVar),
    Mod(Var, ValOrVar),
    Eql(Var, ValOrVar),
}

impl Com {
    fn is_inp(&self) -> bool {
        matches!(self, Com::Inp(_))
    }
}

struct Parser {}
impl Parser {
    fn parse(input: &str) -> Vec<Com> {
        many1(terminated(Parser::command, line_ending))(input)
            .expect("Parser failed")
            .1
    }
    fn command(input: &str) -> IResult<&str, Com> {
        let (inp, com) = preceded(multispace0, take(4usize))(input)?;
        match com {
            "inp " => {
                let (inp, var) = Parser::var(inp)?;
                Ok((inp, Com::Inp(var)))
            }
            "add " => {
                let (inp, (var, val)) = Parser::digit_pair(inp)?;
                Ok((inp, Com::Add(var, val)))
            }
            "mul " => {
                let (inp, (var, val)) = Parser::digit_pair(inp)?;
                Ok((inp, Com::Mul(var, val)))
            }
            "div " => {
                let (inp, (var, val)) = Parser::digit_pair(inp)?;
                Ok((inp, Com::Div(var, val)))
            }
            "mod " => {
                let (inp, (var, val)) = Parser::digit_pair(inp)?;
                Ok((inp, Com::Mod(var, val)))
            }
            "eql " => {
                let (inp, (var, val)) = Parser::digit_pair(inp)?;
                Ok((inp, Com::Eql(var, val)))
            }
            _ => {
                log::debug!("input {:?} com  {:?}", input, com);
                unreachable!()
            }
        }
    }

    fn digit_pair(input: &str) -> IResult<&str, (Var, ValOrVar)> {
        separated_pair(Parser::var, multispace1, Parser::val_var)(input)
    }

    fn var(input: &str) -> IResult<&str, Var> {
        map_res(
            recognize(one_of("wxyz")),
            |val| -> Result<Var, nom::Err<nom::error::Error<&str>>> {
                match val {
                    "w" => Ok(Var(0)),
                    "x" => Ok(Var(1)),
                    "y" => Ok(Var(2)),
                    "z" => Ok(Var(3)),
                    _ => unreachable!(),
                }
            },
        )(input)
    }

    fn val_var(input: &str) -> IResult<&str, ValOrVar> {
        alt((
            map_res(
                recognize(one_of("wxyz")),
                |val| -> Result<ValOrVar, nom::Err<nom::error::Error<&str>>> {
                    match val {
                        "w" => Ok(ValOrVar::Var(0)),
                        "x" => Ok(ValOrVar::Var(1)),
                        "y" => Ok(ValOrVar::Var(2)),
                        "z" => Ok(ValOrVar::Var(3)),
                        _ => unreachable!(),
                    }
                },
            ),
            map_res(
                Parser::decimal_value,
                |val| -> Result<ValOrVar, nom::Err<nom::error::Error<&str>>> {
                    Ok(ValOrVar::Val(val))
                },
            ),
        ))(input)
    }

    fn digit_value(input: &str) -> IResult<&str, usize> {
        map_res(Parser::digit, |val| val.parse::<usize>())(input)
    }

    fn digit(input: &str) -> IResult<&str, &str> {
        recognize(one_of("0123456789"))(input)
    }

    fn decimal_value(input: &str) -> IResult<&str, isize> {
        map_res(Parser::decimal, |val| val.parse::<isize>())(input)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(one_of("-0123456789")))(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_log::test;

    fn input(idx: usize) -> &'static str {
        match idx {
            0 => indoc! {"
                inp x
                mul x -1
            "},

            1 => indoc! {"
		inp z
                inp x
                mul z 3
                eql z x
	    "},
            2 => indoc! {"
		inp w
                add z w
                mod z 2
                div w 2
                add y w
                mod y 2
                div w 2
                add x w
                mod x 2
                div w 2
                mod w 2
	    "},
            _ => unreachable!(),
        }
    }
    #[test]
    fn command() {
        let mut alu = Alu::new();
        log::debug!("{:?}", input(0));
        let program = Parser::parse(input(0));

        assert!(alu.command(&program[0], &[22]).is_some());
        assert!(alu.command(&program[1], &Vec::new()).is_some());
        assert_eq!(alu.register[1], -22);
        assert!(alu.command(&program[1], &Vec::new()).is_some());
        assert_eq!(alu.register[1], 22);
        assert!(alu.command(&program[1], &Vec::new()).is_some());
        assert_eq!(alu.register[1], -22);
    }

    #[test]
    fn run_program() {
        let mut alu = Alu::new();
        let program = Parser::parse(input(0));
        assert!(alu.run_program(&program, &[22]));
        assert_eq!(alu.register[1], -22);

        alu.reset();
        let program = Parser::parse(input(1));
        assert!(alu.run_program(&program, &[1, 3]));
        assert_eq!(alu.register[3], 1);

        alu.reset();
        let program = Parser::parse(input(1));
        assert!(alu.run_program(&program, &[2, 4]));
        assert_eq!(alu.register[3], 0);
        alu.reset();
        let program = Parser::parse(input(1));
        assert!(alu.run_program(&program, &[5, 15]));
        assert_eq!(alu.register[3], 1);
    }
}
