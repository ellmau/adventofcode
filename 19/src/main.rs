use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, one_of},
    combinator::{map_res, recognize},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
    IResult,
};
fn main() {
    env_logger::init();
    let input = std::fs::read_to_string("input").expect("File IO issue");
    let input = Parser::parse(&input);

    let result = max_man_dist(&input);
    log::info!("beacons: {}, scanner-distance: {}", result.0, result.1);
}

fn fill_hashset(input: &[(isize, isize, isize)]) -> HashSet<(isize, isize, isize)> {
    let mut hashset = HashSet::new();
    input.iter().for_each(|e| {
        hashset.insert(*e);
    });
    hashset
}

fn max_man_dist(scanners: &[Vec<(isize, isize, isize)>]) -> (usize, isize) {
    let mut first_one = fill_hashset(&scanners[0]);
    let mut input = scanners[1..].to_vec();

    let mut distance: Vec<(isize, isize, isize)> = vec![(0, 0, 0)];
    while !input.is_empty() {
        (0..input.len()).rev().for_each(|idx| {
            if let Some(dist) = arrange_scanners(&mut first_one, &input[idx]) {
                distance.push(dist);
                input.remove(idx);
            }
        });
    }

    let max_dist = distance
        .iter()
        .cartesian_product(distance.iter())
        .map(|((x1, y1, z1), (x2, y2, z2))| ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()))
        .max();

    (first_one.len(), max_dist.expect("max val needs to exist"))
}

fn _arrange_all_scanners(
    scanners: &[Vec<(isize, isize, isize)>],
) -> HashSet<(isize, isize, isize)> {
    let mut first_one = fill_hashset(&scanners[0]);
    let mut input = scanners[1..].to_vec();

    while !input.is_empty() {
        (0..input.len()).rev().for_each(|idx| {
            if arrange_scanners(&mut first_one, &input[idx]).is_some() {
                input.remove(idx);
            }
        });
    }
    first_one
}

fn arrange_scanners(
    known_findings: &mut HashSet<(isize, isize, isize)>,
    scanner2: &[(isize, isize, isize)],
) -> Option<(isize, isize, isize)> {
    for rotate in 0..24 {
        log::trace!("try rotation {}", rotate);
        let rotated_scan = rotate_scanner(scanner2, rotate);
        let combinations = known_findings.iter().cartesian_product(rotated_scan.iter());
        let distances =
            combinations.map(|((x1, y1, z1), (x2, y2, z2))| (*x1 - *x2, *y1 - *y2, *z1 - *z2));

        for (dx, dy, dz) in distances {
            let possible_translation = rotated_scan
                .iter()
                .map(|(x, y, z)| (*x + dx, *y + dy, *z + dz));
            if possible_translation
                .clone()
                .filter(|coord| known_findings.contains(coord))
                .count()
                >= 12
            {
                known_findings.extend(possible_translation);
                return Some((dx, dy, dz)); // No .iter().for_each() possible, because of return command
            }
        }
    }
    None
}

fn rotate_scanner(scanner: &[(isize, isize, isize)], rotate: usize) -> Vec<(isize, isize, isize)> {
    scanner
        .iter()
        .map(|val| rotate_coord(*val, rotate))
        .collect()
}

fn rotate_coord(coord: (isize, isize, isize), rotate: usize) -> (isize, isize, isize) {
    let (x, y, z) = coord;
    match rotate {
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (y, x, -z),
        5 => (y, z, x),
        6 => (y, -x, z),
        7 => (y, -z, -x),
        8 => (z, x, y),
        9 => (z, y, -x),
        10 => (z, -x, -y),
        11 => (z, -y, x),
        12 => (-x, y, -z),
        13 => (-x, z, y),
        14 => (-x, -y, z),
        15 => (-x, -z, -y),
        16 => (-y, x, z),
        17 => (-y, z, -x),
        18 => (-y, -x, -z),
        19 => (-y, -z, x),
        20 => (-z, x, -y),
        21 => (-z, y, x),
        22 => (-z, -x, y),
        23 => (-z, -y, -x),
        _ => (x, y, z),
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

    fn read_coord(input: &str) -> IResult<&str, (isize, isize, isize)> {
        terminated(
            separated_list1(tag(","), Parser::decimal_value),
            line_ending,
        )(input)
        .map(|(inp, res)| (inp, (res[0], res[1], res[2])))
    }

    fn read_sensor(input: &str) -> IResult<&str, Vec<(isize, isize, isize)>> {
        let (inp, _) = delimited(
            tag("--- scanner "),
            Parser::decimal,
            terminated(tag(" ---"), line_ending),
        )(input)?;

        terminated(many1(Parser::read_coord), line_ending)(inp)
    }

    fn parse(input: &str) -> Vec<Vec<(isize, isize, isize)>> {
        many1(Parser::read_sensor)(input).expect("Parser failed").1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_log::test;

    #[test]
    fn parser() {
        let input = full_exa_input();

        assert_eq!(Parser::parse(input).len(), 5);
    }

    #[test]
    fn part_1() {
        let input = full_exa_input();
        let input = Parser::parse(input);

        assert_eq!(_arrange_all_scanners(&input).len(), 79);
    }

    #[test]
    fn part_2() {
        let input = full_exa_input();
        let input = Parser::parse(input);

        assert_eq!(max_man_dist(&input), (79, 3621));
    }

    fn full_exa_input() -> &'static str {
        indoc! {"
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401
            
            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390
            
            --- scanner 2 ---
            649,640,665
            682,-795,504
            -784,533,-524
            -644,584,-595
            -588,-843,648
            -30,6,44
            -674,560,763
            500,723,-460
            609,671,-379
            -555,-800,653
            -675,-892,-343
            697,-426,-610
            578,704,681
            493,664,-388
            -671,-858,530
            -667,343,800
            571,-461,-707
            -138,-166,112
            -889,563,-600
            646,-828,498
            640,759,510
            -630,509,768
            -681,-892,-333
            673,-379,-804
            -742,-814,-386
            577,-820,562
            
            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596
            
            --- scanner 4 ---
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14

	"}
    }

    fn easyinput() -> &'static str {
        indoc! {"
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401
            
            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390

	"}
    }

    #[test]
    fn translate() {
        let input = easyinput();

        let input = Parser::parse(input);
        let mut known_scans = fill_hashset(&input[0]);
        assert_eq!(
            arrange_scanners(&mut known_scans, &input[1]),
            Some((68, -1246, -43))
        );

        assert_eq!(
            arrange_scanners(&mut fill_hashset(&input[0]), &input[0]),
            Some((0, 0, 0))
        );
    }
}
