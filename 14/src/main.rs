use std::{collections::HashMap, io::BufRead};

fn main() {
    let file = std::fs::File::open(std::env::args().last().unwrap()).unwrap();
    let mut rules: HashMap<(char, char), char> = HashMap::new();
    let mut buffer = std::io::BufReader::new(file);

    let mut starting_line = String::new();
    buffer.read_line(&mut starting_line).unwrap();

    let mut input: Vec<char> = starting_line.chars().collect();
    input.remove(input.len() - 1);
    //let last_char = *input.last().unwrap();

    for line in buffer.lines() {
        let readline = line.unwrap();

        if readline.len() == 7 {
            rules.insert(
                (
                    readline.chars().next().unwrap(),
                    readline.chars().nth(1).unwrap(),
                ),
                readline.chars().nth(6).unwrap(),
            );
        }
    }

    println!("{:?}", input);
    println!("{:?}", rules);

    // part 1
    //let mut result: Vec<char> = input.clone();
    // for i in 1..11 {
    //     println!("step {}", i);
    //     result = result
    //         .iter()
    //         .zip(result[1..].iter())
    //         .fold(Vec::new(), |mut acc, (e1, e2)| {
    //             if let Some(answer) = rules.get(&(*e1, *e2)) {
    //                 acc.append(&mut vec![*e1, *answer]);
    //             }
    //             acc
    //         });
    //     result.push(last_char);
    // }

    //println!("{}", score(&result));
    part2(&input, &rules, 10);

    // for i in 11..26 {
    //     println!("step {}", i);
    //     result = result
    //         .iter()
    //         .zip(result[1..].iter())
    //         .fold(Vec::new(), |mut acc, (e1, e2)| {
    //             if let Some(answer) = rules.get(&(*e1, *e2)) {
    //                 acc.append(&mut vec![*e1, *answer]);
    //             }
    //             acc
    //         });
    //     result.push(last_char);
    // }

    part2(&input, &rules, 40);
}

fn score(list: &[char]) -> usize {
    let mut scorelist: HashMap<char, usize> = HashMap::new();
    for elem in list {
        let score = *scorelist.get(elem).unwrap_or(&0);
        scorelist.insert(*elem, score + 1);
    }
    let mut scores: Vec<usize> = scorelist.values().copied().collect();
    scores.sort_unstable();
    scores.last().unwrap() - scores.first().unwrap()
}

fn part2(inp: &[char], rules: &HashMap<(char, char), char>, steps: usize) {
    let mut input: HashMap<(char, char), usize> = HashMap::new();
    let mut last = inp[0];
    inp[1..].iter().for_each(|e| {
        *input.entry((last, *e)).or_insert(0) += 1;
        last = *e;
    });
    input.insert(('h', inp[0]), 1);
    input.insert((inp.last().copied().unwrap(), 'h'), 1);
    let result = (0..steps)
        .into_iter()
        .fold(input, |acc, _| growth_sim(acc, rules));

    let mut result_counts: HashMap<char, usize> = HashMap::new();
    result.into_iter().for_each(|((c1, c2), count)| {
        *result_counts.entry(c1).or_insert(0) += count;
    });

    result_counts.remove(&'h');
    println!(
        "{}",
        result_counts.values().max().copied().unwrap()
            - result_counts.values().min().copied().unwrap()
    );
}

fn growth_sim(
    input: HashMap<(char, char), usize>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut result = HashMap::new();
    input.into_iter().for_each(|((c1, c2), count)| {
        if let Some(mid) = rules.get(&(c1, c2)) {
            *result.entry((c1, *mid)).or_insert(0) += count;
            *result.entry((*mid, c2)).or_insert(0) += count;
        } else {
            *result.entry((c1, c2)).or_insert(0) += count;
        }
    });
    result
}
