use nom::{
    bytes::complete::{tag, take},
    character::complete::one_of,
    combinator::{all_consuming, opt},
    error::ParseError,
    multi::{many1, many_m_n, separated_list1},
    sequence::{pair, terminated},
    Finish, IResult,
};
use std::{cmp::Reverse, fmt::Binary, fs::File, io::Read, ops::Index};

fn hex_input(input: &str) -> Vec<u8> {
    input
        .chars()
        .into_iter()
        .map(|elem| elem.to_digit(16))
        .map(|v| match v {
            Some(0) => vec![0, 0, 0, 0],
            Some(1) => vec![0, 0, 0, 1],
            Some(2) => vec![0, 0, 1, 0],
            Some(3) => vec![0, 0, 1, 1],
            Some(4) => vec![0, 1, 0, 0],
            Some(5) => vec![0, 1, 0, 1],
            Some(6) => vec![0, 1, 1, 0],
            Some(7) => vec![0, 1, 1, 1],
            Some(8) => vec![1, 0, 0, 0],
            Some(9) => vec![1, 0, 0, 1],
            Some(10) => vec![1, 0, 1, 0],
            Some(11) => vec![1, 0, 1, 1],
            Some(12) => vec![1, 1, 0, 0],
            Some(13) => vec![1, 1, 0, 1],
            Some(14) => vec![1, 1, 1, 0],
            Some(15) => vec![1, 1, 1, 1],
            _ => Vec::new(),
        })
        .flatten()
        .collect()
}

fn package(input: &[u8]) -> (&[u8], Package) {
    let (content, version, kind) = package_preamble(input);
    match kind {
        4 => {
            let (remain, value) = literal_block(content);
            (
                remain,
                Package {
                    version,
                    kind: Type::Literal(get_usize(&value)),
                },
            )
        }
        x => {
            let (remain, value) = operator_content(content);
            (
                remain,
                Package {
                    version,
                    kind: Type::Operator(x, value),
                },
            )
        }
    }
}

fn package_preamble(input: &[u8]) -> (&[u8], usize, usize) {
    (&input[6..], get_usize(&input[..3]), get_usize(&input[3..6]))
}

fn literal_block(input: &[u8]) -> (&[u8], Vec<u8>) {
    match input[0] {
        1 => {
            let (remain, result) = literal_block(&input[5..]);
            (
                &remain,
                input[1..5]
                    .iter()
                    .cloned()
                    .chain(result.iter().cloned())
                    .collect(),
            )
        }
        _ => (&input[5..], input[1..5].to_vec()),
    }
}
fn operator_content(input: &[u8]) -> (&[u8], Vec<Package>) {
    // lengthid
    if input[0] == 0 {
        let mut bits = get_usize(&input[1..=15]);
        let mut packageslice = &input[16..][..bits];
        let remain = &input[16..][bits..];
        let mut result: Vec<Package> = Vec::new();
        while bits > 1 {
            let (rem, res) = package(packageslice);
            result.push(res);
            packageslice = rem;
            bits = rem.len();
        }
        (remain, result)
    } else {
        let mut packages = get_usize(&input[1..=11]);
        let mut result: Vec<Package> = Vec::new();
        let mut remaining = &input[12..];
        (0..packages).into_iter().for_each(|_x| {
            let (rem, res) = package(remaining);
            result.push(res);
            remaining = rem;
        });
        (remaining, result)
    }
}

fn get_usize(input: &[u8]) -> usize {
    let mut pot = 0;

    let result = input.iter().rev().fold(0, |acc, elem| {
        let result = acc + ((*elem as usize) * (2usize.pow(pot)));
        pot += 1;
        result
    });
    result
}

fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("input").unwrap();
    let package = package(&hex_input(&input)).1;
    log::info!("Version-sum: {}", package.version());
    log::info!("Evaluated expression: {}", package.value());
}
#[derive(PartialEq, Eq, Debug)]
struct Package {
    version: usize,
    kind: Type,
}

impl Package {
    fn version(&self) -> usize {
        self.version + self.kind.version()
    }

    fn value(&self) -> usize {
        self.kind.value()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Type {
    Literal(usize),
    Operator(usize, Vec<Package>),
}

impl Type {
    fn version(&self) -> usize {
        match self {
            Type::Literal(_) => 0,
            Type::Operator(x, nested) => nested.iter().fold(0, |acc, elem| acc + elem.version()),
        }
    }

    fn value(&self) -> usize {
        match self {
            Type::Literal(val) => *val,
            Type::Operator(op, packets) => match op {
                0 => packets.iter().fold(0, |acc, elem| acc + elem.value()),
                1 => packets.iter().fold(1, |acc, elem| acc * elem.value()),
                2 => packets
                    .iter()
                    .fold(usize::MAX, |acc, elem| acc.min(elem.value())),
                3 => packets
                    .iter()
                    .fold(usize::MIN, |acc, elem| acc.max(elem.value())),
                5 => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use test_log::test;

    #[test]
    fn hex_conversion() {
        let hex_res = hex_input("2D");
        assert_eq!(hex_res, vec![0u8, 0, 1, 0, 1, 1, 0, 1]);
    }

    #[test]
    fn to_usize() {
        assert_eq!(get_usize(&hex_input("0")), 0);
        assert_eq!(get_usize(&hex_input("1")), 1);
        assert_eq!(get_usize(&hex_input("F")), 15);
        assert_eq!(get_usize(&hex_input("2DE")), 734);
    }

    #[test]
    fn examples() {
        let exa = vec![
            "8A004A801A8002F478",
            "620080001611562C8802118E34",
            "C0015000016115A2E0802F182340",
            "A0016C880162017C3686B18A3D4780",
        ];
        let res = vec![16, 12, 23, 31usize];

        exa.iter().zip(res.iter()).for_each(|(inp, out)| {
            log::debug!("{:?}", package(&hex_input(inp)));
            assert_eq!(package(&hex_input(inp)).1.version(), *out);
        });

        let exa = vec![
            "C200B40A82",
            "04005AC33890",
            "880086C3E88112",
            "CE00C43D881120",
            "D8005AC2A8F0",
            "F600BC2D8F",
            "9C005AC2F8F0",
            "9C0141080250320F1802104A08",
        ];

        let res: Vec<usize> = vec![3, 54, 7, 9, 1, 0, 0, 1];

        exa.iter().zip(res.iter()).for_each(|(inp, out)| {
            assert_eq!(package(&hex_input(inp)).1.value(), *out);
        });
    }

    #[test]
    fn part_examples() {
        let vec = vec![0u8, 0, 0];
        assert_eq!(
            package(&hex_input("D2FE28")),
            (
                &vec[0..],
                Package {
                    version: 6,
                    kind: Type::Literal(2021)
                }
            )
        );
        assert_eq!(
            package(&hex_input("38006F45291200")).1,
            Package {
                version: 1,
                kind: Type::Operator(
                    6,
                    vec![
                        Package {
                            version: 6,
                            kind: Type::Literal(10)
                        },
                        Package {
                            version: 2,
                            kind: Type::Literal(20)
                        }
                    ]
                )
            }
        );

        assert_eq!(
            package(&hex_input("EE00D40C823060")).1,
            Package {
                version: 7,
                kind: Type::Operator(
                    3,
                    vec![
                        Package {
                            version: 2,
                            kind: Type::Literal(1)
                        },
                        Package {
                            version: 4,
                            kind: Type::Literal(2)
                        },
                        Package {
                            version: 1,
                            kind: Type::Literal(3)
                        }
                    ]
                )
            }
        );
    }
}
