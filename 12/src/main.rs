use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};

use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::{
        complete::{alpha1, char, line_ending, one_of},
        is_digit,
    },
    combinator::{map_res, recognize},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

fn main() {
    let input = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();

    let (_, input_construct) = Parser::parse(&input).unwrap();

    let mut graph = Graph::default();

    Parser::parse(&input).unwrap().1.iter().for_each(|pair| {
        graph.add_node(pair.0);
        graph.add_node(pair.1);
        graph.link_nodes(pair.0, pair.1);
    });

    let result = graph.paths();
    println!("{}", result.len());

    let mut result2 = graph.paths2();
    result2.sort_unstable();
    result2.dedup();

    println!("{}", result2.len());
}

struct Node {
    nmbr: usize,
    name: String,
    neighbors: Vec<Weak<RefCell<Node>>>,
    is_start: bool,
}

impl Node {
    fn new(input: &str) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Node {
            nmbr: COUNTER.fetch_add(1, Ordering::Relaxed),
            name: String::from(input),
            neighbors: Vec::new(),
            is_start: input.eq("start"),
        }
    }

    fn connect_to(&mut self, other_node: &Rc<RefCell<Node>>) {
        self.neighbors.push(Rc::downgrade(other_node));
    }

    fn is_big(&self) -> bool {
        self.name == self.name.to_uppercase()
    }

    fn paths(&self, visited: &[usize]) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        if self.name == "end" {
            result.push([visited, &[self.nmbr]].concat());
            return result;
        }
        if self.is_big() || !visited.contains(&self.nmbr) {
            self.neighbors.iter().for_each(|elem| {
                let node = elem.upgrade().unwrap();
                let mut inner_result = node.borrow().paths(&[visited, &[self.nmbr]].concat());
                result.append(&mut inner_result);
            });
        }
        result
    }

    fn paths2(&self, visited: &[usize], double_visit: bool) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        if self.name == "end" {
            result.push([visited, &[self.nmbr]].concat());
            return result;
        }
        let cont = visited.contains(&self.nmbr);
        if self.is_big() || !cont || double_visit {
            self.neighbors.iter().for_each(|elem| {
                let node = elem.upgrade().unwrap();
                if self.is_big() || !cont {
                    let mut inner_result = node
                        .borrow()
                        .paths2(&[visited, &[self.nmbr]].concat(), double_visit);
                    result.append(&mut inner_result);
                }
                if double_visit && !self.is_big() && cont && !self.is_start {
                    let mut inner_result = node.borrow().paths(&[visited, &[self.nmbr]].concat());
                    result.append(&mut inner_result);
                }
            });
        }
        result
    }
}

struct Graph {
    store: HashMap<String, Rc<RefCell<Node>>>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

impl Graph {
    fn add_node(&mut self, name: &str) {
        self.store
            .entry(String::from(name))
            .or_insert_with(|| Rc::new(RefCell::new(Node::new(name))));
    }

    fn link_nodes(&mut self, name1: &str, name2: &str) {
        if let Some(x) = self.store.get(&String::from(name1)) {
            if let Some(y) = self.store.get(&String::from(name2)) {
                x.borrow_mut().connect_to(y);
                y.borrow_mut().connect_to(x);
            }
        }
    }

    fn paths(&self) -> Vec<Vec<usize>> {
        self.store
            .get("start")
            .map(|node| node.borrow().paths(&[]))
            .unwrap()
    }

    fn paths2(&self) -> Vec<Vec<usize>> {
        self.store
            .get("start")
            .map(|node| node.borrow().paths2(&[], true))
            .unwrap()
    }
}

struct Parser {}
impl Parser {
    fn name(input: &str) -> IResult<&str, &str> {
        alpha1(input)
    }

    fn connection(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(
            Parser::name,
            nom::character::complete::char('-'),
            Parser::name,
        )(input)
    }

    fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
        many1(terminated(Parser::connection, line_ending))(input)
    }
}
