use std::fmt::Display;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, map_res, recognize},
    multi::many1,
    sequence::{delimited, separated_pair, terminated},
    IResult,
};

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("input").expect("IO error");

    let mut input = Parser::parse(&input);
    log::info!(
        "result of the magnitude test is {}",
        compute_sum(&mut input.clone()).magnitude()
    );

    log::info!(
        "result of the max magnitude is {}",
        max_magnitude(&mut input)
    );
}

struct Parser {}

impl Parser {
    fn decimal_value(input: &str) -> IResult<&str, usize> {
        map_res(Parser::decimal, |val| val.parse::<usize>())(input)
    }

    fn decimal(input: &str) -> IResult<&str, &str> {
        recognize(many1(one_of("0123456789")))(input)
    }

    fn num(input: &str) -> IResult<&str, Num> {
        Parser::decimal_value(input).map(|(inp, val)| (inp, Num::Value(val)))
    }

    fn nested(input: &str) -> IResult<&str, Num> {
        Parser::pair(input).map(|(inp, val)| (inp, Num::Number(Box::new(val))))
    }

    fn pair(input: &str) -> IResult<&str, Number> {
        let (inp, pair) = delimited(
            tag("["),
            separated_pair(
                alt((Parser::num, Parser::nested)),
                tag(","),
                alt((Parser::num, Parser::nested)),
            ),
            tag("]"),
        )(input)?;
        Ok((inp, Number { pair }))
    }

    fn parse(input: &str) -> Vec<Number> {
        let (_, result) =
            all_consuming(many1(terminated(Parser::pair, line_ending)))(input).unwrap();
        result
    }
}

fn compute_sum(input: &mut [Number]) -> Number {
    let mut first = input[0].clone();
    input[1..].iter_mut().fold(first, |acc, elem| {
        let mut result = acc.add(elem);
        log::trace!("sum: {}", result);
        result.reduce_closure();
        //result.reduce_new();
        log::debug!("\n {}\n+{}\n={}", acc, elem, result);
        result
    })
}

fn max_magnitude(input: &mut Vec<Number>) -> usize {
    let mut inp2 = input.clone();
    input
        .iter()
        .flat_map(|y| {
            inp2.iter().map(|x| {
                let mut inp = vec![y.clone(), x.clone()];
                compute_sum(&mut inp).magnitude()
            })
        })
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Number {
    pair: (Num, Num),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left(), self.right())
    }
}

impl Number {
    fn left(&self) -> &Num {
        &self.pair.0
    }

    fn right(&self) -> &Num {
        &self.pair.1
    }

    fn left_mut(&mut self) -> &mut Num {
        &mut self.pair.0
    }

    fn right_mut(&mut self) -> &mut Num {
        &mut self.pair.1
    }

    fn add(&self, other: &Number) -> Number {
        Number {
            pair: (
                Num::Number(Box::new(self.clone())),
                Num::Number(Box::new(other.clone())),
            ),
        }
    }

    fn reduce(&mut self) {
        if self.reduce_explode(0) == Action::None {
            self.reduce_split();
        }
    }

    fn reduce_closure(&mut self) {
        loop {
            let mut result = self.reduce_explode(0);
            if result != Action::None {
                log::debug!("{}", self);
            }
            if result == Action::None {
                //let result = self.reduce_split();
                //log::debug!("{}", self);
                if !self.split() {
                    break;
                }
            }
        }
    }

    fn reduce_new(&mut self) {
        loop {
            if !self.explode() {
                if !self.split() {
                    break;
                }
            }
        }
    }

    fn explode(&mut self) -> bool {
        if let Some((_left, right)) = self.left_mut().explode(1) {
            self.right_mut().add_val_l(right);
            return true;
        }
        if let Some((left, _right)) = self.right_mut().explode(1) {
            self.left_mut().add_val_r(left);
            return true;
        }
        false
    }

    fn reduce_explode(&mut self, depth: usize) -> Action {
        let mut result: Action = Action::None;
        if depth > 3 {
            // explode
            if self.left().is_number() || self.right().is_number() {
                panic!("should not happen");
            } else {
                log::trace!("explosion at {}", self);
                result =
                    Action::ExplodeB(self.left().value().unwrap(), self.right().value().unwrap());
            }
        } else {
            if self.left().is_number() {
                result = self
                    .left_mut()
                    .number()
                    .expect("should work")
                    .reduce_explode(depth + 1);
                if result.is_expl_b() {
                    *self.left_mut() = Num::Value(0);
                }
                if result.expl_included(Action::ExplodeR(0)) {
                    if let Some(val) = result.expl_r_val() {
                        log::trace!("propagating right into {} with val {}", self.right(), val);
                        self.right_mut().add_val_l(val);
                        result = result.remove_explosion(Action::ExplodeR(0));
                    }
                }
            }
            if result == Action::None && self.right().is_number() {
                result = self
                    .right_mut()
                    .number()
                    .expect("should work")
                    .reduce_explode(depth + 1);
                if result.is_expl_b() {
                    *self.right_mut() = Num::Value(0);
                }
                if result.expl_included(Action::ExplodeL(0)) {
                    if let Some(val) = result.expl_l_val() {
                        log::trace!("propagating left into {} val {}", self.left(), val);
                        self.left_mut().add_val_r(val);
                        result = result.remove_explosion(Action::ExplodeL(0));
                    }
                }
            }
        }
        result
    }

    fn reduce_split(&mut self) -> Action {
        let mut result: Action = Action::None;
        if self.pair.0.split() {
            self.pair.0 = Num::num_pair_from_pair(self.pair.0.div_pair().expect("should work"));
            Action::Split
        } else if self.pair.1.split() {
            self.pair.1 = Num::num_pair_from_pair(self.pair.1.div_pair().expect("should work"));
            Action::Split
        } else {
            if self.pair.0.is_number() {
                result = self.pair.0.number().expect("should work").reduce_split();
            }
            if result == Action::None && self.pair.1.is_number() {
                result = self.pair.1.number().expect("should work").reduce_split();
            }
            result
        }
    }

    fn split(&mut self) -> bool {
        self.left_mut().split_new() || self.right_mut().split_new()
    }

    fn magnitude(&self) -> usize {
        (3 * self.pair.0.magnitude()) + (2 * self.pair.1.magnitude())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Explode,
    ExplodeL(usize),
    ExplodeR(usize),
    ExplodeB(usize, usize),
    Split,
    None,
}

impl Action {
    fn is_none(&self) -> bool {
        match self {
            Action::None => true,
            _ => false,
        }
    }

    fn is_expl(&self) -> bool {
        match self {
            Action::Explode => true,
            Action::ExplodeL(_) => true,
            Action::ExplodeR(_) => true,
            Action::ExplodeB(_, _) => true,
            Action::Split => false,
            Action::None => false,
        }
    }

    fn expl_included(&self, other: Action) -> bool {
        match self {
            Action::Explode => false,
            Action::ExplodeL(_) => match other {
                Action::ExplodeL(_) => true,
                Action::ExplodeB(_, _) => true,
                _ => false,
            },
            Action::ExplodeR(_) => match other {
                Action::ExplodeR(_) => true,
                Action::ExplodeB(_, _) => true,
                _ => false,
            },
            Action::ExplodeB(_, _) => true,
            Action::Split => false,
            Action::None => false,
        }
    }

    fn is_expl_r(&self) -> bool {
        match self {
            Action::ExplodeR(_) => true,
            _ => false,
        }
    }

    fn expl_r_val(&self) -> Option<usize> {
        match self {
            Action::ExplodeR(val) => Some(*val),
            Action::ExplodeB(l, r) => Some(*r),
            _ => None,
        }
    }

    fn is_expl_l(&self) -> bool {
        match self {
            Action::ExplodeL(_) => true,
            _ => false,
        }
    }

    fn expl_l_val(&self) -> Option<usize> {
        match self {
            Action::ExplodeL(val) => Some(*val),
            Action::ExplodeB(l, r) => Some(*l),
            _ => None,
        }
    }

    fn is_expl_b(&self) -> bool {
        match self {
            Action::ExplodeB(_, _) => true,
            _ => false,
        }
    }

    fn remove_explosion(self, other: Action) -> Action {
        match self {
            Action::Explode => self,
            Action::ExplodeL(_) => {
                if other.is_expl_l() {
                    Action::Explode
                } else {
                    self
                }
            }
            Action::ExplodeR(_) => {
                if other.is_expl_r() {
                    Action::Explode
                } else {
                    self
                }
            }
            Action::ExplodeB(l, r) => match other {
                Action::ExplodeL(_) => Action::ExplodeR(r),
                Action::ExplodeR(_) => Action::ExplodeL(l),
                _ => self,
            },
            Action::Split => self,
            Action::None => self,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Num {
    Value(usize),
    Number(Box<Number>),
}

impl Num {
    fn explode(&mut self, depth: usize) -> Option<(usize, usize)> {
        log::trace!("{} at depth {}", self, depth);
        match self {
            Num::Value(_) => None,
            Num::Number(value) => {
                if depth > 3 {
                    let (left, right) = (
                        value.left().value().unwrap(),
                        value.right().value().unwrap(),
                    );
                    *self = Num::Value(0);
                    log::trace!("reducing [{},{}] to 0", left, right);
                    Some((left, right))
                } else if let Some((left, right)) = value.left_mut().explode(depth + 1) {
                    value.right_mut().add_val_l(right);
                    Some((left, 0))
                } else if let Some((left, right)) = value.right_mut().explode(depth + 1) {
                    value.left_mut().add_val_r(left);
                    Some((0, right))
                } else {
                    None
                }
            }
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            Num::Value(x) => *x,
            Num::Number(num) => num.magnitude(),
        }
    }

    fn split_new(&mut self) -> bool {
        match self {
            Num::Value(val) => {
                if *val > 9 {
                    *self = Num::Number(Box::new(Number {
                        pair: (Num::Value(*val / 2), Num::Value((*val + 1) / 2)),
                    }));

                    true
                } else {
                    false
                }
            }
            Num::Number(num) => num.split(),
        }
    }

    fn split(&self) -> bool {
        match self {
            Num::Value(val) => *val > 9,
            Num::Number(_) => false,
        }
    }

    fn div_pair(&self) -> Option<(usize, usize)> {
        if let Num::Value(x) = self {
            Some((x / 2, (x + 1) / 2))
        } else {
            None
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Num::Value(_) => false,
            Num::Number(_) => true,
        }
    }

    fn number(&mut self) -> Option<&mut Number> {
        if let Num::Number(x) = self {
            Some(x)
        } else {
            None
        }
    }

    fn value(&self) -> Option<usize> {
        if let Num::Value(x) = self {
            Some(*x)
        } else {
            None
        }
    }

    fn create_trivial_num_pair(a: usize, b: usize) -> Num {
        Num::Number(Box::new(Number {
            pair: (Num::Value(a), Num::Value(b)),
        }))
    }

    fn num_pair_from_pair(p: (usize, usize)) -> Num {
        Num::create_trivial_num_pair(p.0, p.1)
    }

    fn add_val_l(&mut self, val: usize) {
        if val > 0 {
            match self {
                Num::Value(value) => {
                    *value += val;
                }
                Num::Number(num) => {
                    num.left_mut().add_val_l(val);
                }
            }
        }
    }

    fn add_val_r(&mut self, val: usize) {
        if val > 0 {
            match self {
                Num::Value(value) => {
                    *value += val;
                }
                Num::Number(num) => {
                    num.right_mut().add_val_r(val);
                }
            }
        }
    }
}

impl Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Num::Value(val) => {
                write!(f, "{}", *val)
            }
            Num::Number(boxed) => write!(f, "{}", boxed.as_ref()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_log::test;

    #[test]
    fn parse() {
        let input = indoc! {"
           [1,2]
           [[1,2],3]
           [9,[8,7]]
           [[1,9],[8,5]]
           [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
           [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
           [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
	"};
        log::debug!("{:?}", Parser::parse(input));
        for num in Parser::parse(input) {
            log::debug!("{}", num);
        }
    }
    #[test]
    fn add() {
        let test_vec = Parser::parse("[1,2]\n[[3,4],5]\n[[1,2],[[3,4],5]]\n");
        assert_eq!(test_vec[0].add(&test_vec[1]), test_vec[2]);
    }

    #[test]
    fn reduce() {
        let input = indoc! {"
           [[[[[9,8],1],2],3],4]
           [7,[6,[5,[4,[3,2]]]]]
           [[6,[5,[4,[3,2]]]],1]
           [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]
           [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
        "};

        let results = indoc! {"
           [[[[0,9],2],3],4]
           [7,[6,[5,[7,0]]]]
           [[6,[5,[7,0]]],3]
           [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]
           [[3,[2,[8,0]]],[9,[5,[7,0]]]]
         "};

        let mut test_vec = Parser::parse(input);
        let res_vec = Parser::parse(results);
        test_vec
            .iter_mut()
            .zip(res_vec.iter())
            .for_each(|(inp, out)| {
                log::debug!("{}", inp);
                inp.reduce_explode(0);
                log::debug!("{}", inp);
                log::debug!("{}", out);
                assert_eq!(inp, out);
            })
    }

    #[test]
    fn final_sum() {
        let input = indoc! {"
            [1,1]
            [2,2]
            [3,3]
            [4,4]
            [5,5]
            [6,6]
        "};

        let result = "[[[[5,0],[7,4]],[5,5]],[6,6]]\n";

        let mut input = Parser::parse(input);
        let result = Parser::parse(result);

        let computed_result = compute_sum(&mut input);
        log::debug!("{}", computed_result);
        log::debug!("{}", result[0]);
        assert_eq!(computed_result, result[0]);
        log::debug!("\n============================= exa 1 done =============================\n");

        let input = "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]\n";
        let result = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]\n";

        let mut input = Parser::parse(input);
        let result = Parser::parse(result);

        let computed_result = compute_sum(&mut input);
        log::debug!("{}", computed_result);
        log::debug!("{}", result[0]);
        assert_eq!(computed_result, result[0]);

        log::debug!("\n============================= exa 2 done =============================\n");

        let input = indoc! {"
            [[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
            [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
            [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
            [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
            [7,[5,[[3,8],[1,4]]]]
            [[2,[2,2]],[8,[8,1]]]
            [2,9]
            [1,[[[9,3],9],[[9,0],[0,7]]]]
            [[[5,[7,4]],7],1]
            [[[[4,2],2],6],[8,7]]
        "};

        let result = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]\n";

        let mut input = Parser::parse(input);
        let result = Parser::parse(result);

        let computed_result = compute_sum(&mut input);
        log::debug!("{}", computed_result);
        log::debug!("{}", result[0]);
        assert_eq!(computed_result, result[0]);
    }

    #[test]
    fn magnitude() {
        let input = indoc! {"
            [1,9]
            [[9,1],[1,9]]
            [[1,2],[[3,4],5]]
            [[[[0,7],4],[[7,8],[6,0]]],[8,1]]
            [[[[1,1],[2,2]],[3,3]],[4,4]]
            [[[[3,0],[5,3]],[4,4]],[5,5]]
            [[[[5,0],[7,4]],[5,5]],[6,6]]
            [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]
         "};

        let result = vec![21, 129, 143, 1384, 445, 791, 1137, 3488usize];

        let input = Parser::parse(input);
        input.iter().zip(result.iter()).for_each(|(inp, out)| {
            assert_eq!(inp.magnitude(), *out);
        })
    }

    #[test]
    fn exa() {
        let input = indoc! {"
            [[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
            [[[5,[2,8]],4],[5,[[9,9],0]]]
            [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
            [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
            [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
            [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
            [[[[5,4],[7,7]],8],[[8,3],8]]
            [[9,3],[[9,9],[6,[4,9]]]]
            [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
            [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
        "};

        let mut input = Parser::parse(input);
        let mut input2 = input.clone();
        let result =
            Parser::parse("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]\n");
        let magnitude = 4140usize;

        let computed_result = compute_sum(&mut input);
        assert_eq!(computed_result, result[0]);
        assert_eq!(computed_result.magnitude(), magnitude);

        assert_eq!(max_magnitude(&mut input2), 3993);
    }

    #[test]
    fn crashing() {
        let input = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]\n[7,[5,[[3,8],[1,4]]]]\n";
        let result = "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]\n";
        let mut input = Parser::parse(input);
        let result = Parser::parse(result);

        let output = compute_sum(&mut input);
        log::trace!("\n ex {}", result[0]);
        assert_eq!(output, result[0]);
    }
}
