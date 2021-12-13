use std::io::BufRead;

fn main() {
    let mut bit_field = to_bits(&read_input());
    let mut result: Vec<isize> = Vec::new();

    for _ in 0..bit_field[0].len() {
        result.push(0);
    }

    for line in bit_field.clone() {
        for (pos, elem) in line.iter().enumerate() {
            if *elem {
                result[pos] += 1;
            } else {
                result[pos] -= 1;
            }
        }
    }

    let (resg, rese, gamma, epsilon) = get_int(&result);
    println!("{}; {}; {}", resg, rese, resg * rese);

    let mut oxygen = bit_field.clone();
    let mut co2 = bit_field.clone();
    let mut oxycan: Vec<Vec<bool>> = Vec::new();
    let mut co2can: Vec<Vec<bool>> = Vec::new();

    for i in 0..gamma.len() {
        let mut resultoxy: Vec<isize> = Vec::new();
        let mut resultco2: Vec<isize> = Vec::new();
        for _ in 0..gamma.len() {
            resultoxy.push(0);
            resultco2.push(0);
        }

        for line in oxygen.clone() {
            for (pos, elem) in line.iter().enumerate() {
                if *elem {
                    resultoxy[pos] += 1;
                } else {
                    resultoxy[pos] -= 1;
                }
            }
        }
        for line in co2.clone() {
            for (pos, elem) in line.iter().enumerate() {
                if *elem {
                    resultco2[pos] += 1;
                } else {
                    resultco2[pos] -= 1;
                }
            }
        }
        let (_, _, gamma, _) = get_int(&resultoxy);
        let (_, _, _, epsilon) = get_int(&resultco2);
        let mut newoxy: Vec<Vec<bool>> = Vec::new();
        if oxygen.len() > 1 {
            for elem in oxygen {
                if elem[i] == to_bool(&gamma, i) {
                    newoxy.push(elem.clone());
                }
            }
            oxygen = newoxy;
        }
        if co2.len() > 1 {
            let mut newco2: Vec<Vec<bool>> = Vec::new();
            for elem in co2 {
                if elem[i] == to_bool(&epsilon, i) {
                    newco2.push(elem.clone());
                }
            }
            co2 = newco2;
        }
    }

    let oxyval = from_boolvec_to_usize(&oxygen[0]);
    let co2val = from_boolvec_to_usize(&co2[0]);

    println!("oxy {}, co2 {}, val {}", oxyval, co2val, oxyval * co2val);
}

fn from_boolvec_to_usize(input: &[bool]) -> usize {
    let mut result: Vec<char> = Vec::new();
    for b in input {
        if *b {
            result.push('1');
        } else {
            result.push('0');
        }
    }
    let res: String = result.iter().collect();
    usize::from_str_radix(&res, 2).unwrap()
}

fn get_diff_idx(list: &[Vec<bool>], to_remove: &[Vec<bool>]) -> usize {
    for i in 0..list.len() {
        if list.get(i).unwrap() != to_remove.get(i).unwrap() {
            return i;
        }
    }

    list.len()
}

fn to_bool(st: &str, pos: usize) -> bool {
    matches!(st.chars().nth(pos), Some('1'))
}

fn read_input() -> Vec<String> {
    let inputfile = std::env::args().last().unwrap();

    let buf_reader =
        std::io::BufReader::new(std::fs::File::open(std::path::Path::new(&inputfile)).unwrap());

    buf_reader.lines().map(|val| val.unwrap()).collect()
}

fn to_bits(str: &[String]) -> Vec<Vec<bool>> {
    let mut result: Vec<Vec<bool>> = Vec::new();
    for line in str {
        let mut inner: Vec<bool> = Vec::new();
        for elem in line.chars() {
            match elem {
                '1' => inner.push(true),
                '0' => inner.push(false),
                _ => unreachable!(),
            }
        }
        result.push(inner);
    }
    result
}

fn get_int(input: &[isize]) -> (usize, usize, String, String) {
    let mut charay_pos: Vec<char> = Vec::new();
    let mut charay_neg: Vec<char> = Vec::new();

    for elem in input {
        if *elem >= 0 {
            charay_pos.push('1');
            charay_neg.push('0');
        } else {
            charay_pos.push('0');
            charay_neg.push('1');
        }
    }

    let gamma: String = charay_pos.iter().collect();
    let epsilon: String = charay_neg.iter().collect();

    (
        usize::from_str_radix(&gamma, 2).unwrap(),
        usize::from_str_radix(&epsilon, 2).unwrap(),
        gamma,
        epsilon,
    )
}
