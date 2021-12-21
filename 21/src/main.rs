fn main() {
    let input = std::fs::read_to_string("input").expect("IO Error occured");

    let mut split = input.split("\n");

    let p1 = split.next().unwrap();
    let p2 = split.next().unwrap();

    let p1 = p1.split_at(28).1.parse::<usize>().unwrap();
    let p2 = p2.split_at(28).1.parse::<usize>().unwrap();

    println!("game-score is {}", dirac_game(p1, p2, 10_000));
    let result = dirac_multidimension(p1, 0, p2, 0, 1, true);
    println!(
        "multideminsional result {} {} max: {}",
        result.0,
        result.1,
        result.0.max(result.1)
    );
}

fn dirac_game(p1: usize, p2: usize, wc: usize) -> usize {
    let (mut p1_score, mut p2_score) = (p1, p2);
    let (mut p1_last, mut p2_last) = (p1, p2);
    let mut player1_move = false;

    roll_seq()
        .enumerate()
        .try_fold((0, 0), |(p1_score, p2_score), (step, roll)| {
            player1_move = !player1_move;
            if player1_move {
                p1_last = step_score(p1_last, roll);
                let new_p1_score = p1_last + p1_score;

                if new_p1_score < 1_000 {
                    Ok((new_p1_score, p2_score))
                } else {
                    let result = ((step + 1) * 3) * (p2_score);
                    Err(result)
                }
            } else {
                p2_last = step_score(p2_last, roll);
                let new_p2_score = p2_score + p2_last;
                if new_p2_score < 1_000 {
                    Ok((p1_score, new_p2_score))
                } else {
                    let result = ((step + 1) * 3) * (p1_score);
                    Err(result)
                }
            }
        })
        .err()
        .expect("expected to fail")
}

fn dirac_multidimension(
    p1_pos: usize,
    p1_score: usize,
    p2_pos: usize,
    p2_score: usize,
    univ_count: usize,
    p1_plays: bool,
) -> (usize, usize) {
    let roll_variants = vec![
        (3usize, 1usize),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];
    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for (roll, worlds) in roll_variants {
        if p1_plays {
            let new_pos = p1_pos + roll;
            let new_pos = if new_pos > 9 { new_pos - 10 } else { new_pos };
            let new_pos = if new_pos == 0 { 10 } else { new_pos };
            let new_p1_score = p1_score + new_pos;
            if new_p1_score < 21 {
                let res = dirac_multidimension(
                    new_pos,
                    new_p1_score,
                    p2_pos,
                    p2_score,
                    univ_count * worlds,
                    !p1_plays,
                );
                p1_wins += res.0;
                p2_wins += res.1;
            } else {
                p1_wins += worlds * univ_count;
            }
        } else {
            let new_pos = p2_pos + roll;
            let new_pos = if new_pos > 9 { new_pos - 10 } else { new_pos };
            let new_pos = if new_pos == 0 { 10 } else { new_pos };
            let new_p2_score = p2_score + new_pos;
            if new_p2_score < 21 {
                let res = dirac_multidimension(
                    p1_pos,
                    p1_score,
                    new_pos,
                    new_p2_score,
                    univ_count * worlds,
                    !p1_plays,
                );
                p1_wins += res.0;
                p2_wins += res.1;
            } else {
                p2_wins += worlds * univ_count;
            }
        }
    }
    (p1_wins, p2_wins)
}

fn roll(roll: usize) -> usize {
    let rolled = roll * 3;
    (rolled * 3) - 3
}

fn step_score(from: usize, steps: usize) -> usize {
    let result = (from + (steps % 10)) % 10;
    if result == 0 {
        10
    } else {
        result
    }
}

fn roll_seq() -> impl Iterator<Item = usize> {
    (1..usize::MAX)
        .filter(|x| x % 3 == 0)
        .map(|x| ((if x % 100 == 0 { 100 } else { x % 100 }) * 3) - 3)
        .map(|x| match x {
            3 => 103,
            0 => 300,
            val => val,
        })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rolls() {
        assert_eq!(roll(1), 6);
        assert_eq!(roll(2), 15);
        assert_eq!(roll(10), 87);
        assert_eq!(roll(33), 294);

        let mut it = roll_seq();
        assert_eq!(it.next(), Some(6));
        assert_eq!(it.next(), Some(15));
        assert_eq!(roll_seq().nth(9), Some(87));
        assert_eq!(roll_seq().nth(32), Some(294));
        assert_eq!(roll_seq().nth(33), Some(103));
    }

    #[test]
    fn exmaple() {
        assert_eq!(dirac_game(4, 8, 10_000), 739785);
    }

    #[test]
    fn step_sc() {
        assert_eq!(step_score(4, 10), 4);
        assert_eq!(step_score(4, 6), 10);
        assert_eq!(step_score(4, 2), 6);
    }

    #[test]
    fn multidimensional() {
        assert_eq!(
            dirac_multidimension(4, 0, 8, 0, 1, true),
            (444_356_092_776_315, 341_960_390_180_808)
        )
    }
}
