use itertools::Itertools;

fn main() {
    env_logger::init();
    let input = parse(&std::fs::read_to_string("input").expect("IO error"));
    let input2 = input.gen_part_2();
    log::info!("minimal costs are {}", part1(&input));

    log::info!("minimal costs are {}", part1(&input2));
}

fn part1(start: &State) -> usize {
    let mut states = vec![start.clone()];
    let mut finished_states: Vec<State> = Vec::new();
    let mut min = usize::MAX;
    loop {
        states.iter().filter(|st| st.goal_reached()).for_each(|st| {
            finished_states.push(st.clone());
        });
        if states
            .iter()
            .filter(|st| !st.goal_reached() && !st.possible_moves().is_empty())
            .count()
            == 0
        {
            break;
        }
        let mut new_states = Vec::new();
        states.iter().for_each(|st| {
            st.possible_moves().iter().for_each(|mv| {
                let mut new_state = st.clone();
                new_state.do_move(*mv);
                if new_state.costs <= min {
                    if new_state.goal_reached() {
                        log::debug!("reached goal with cost of {}", new_state.costs);
                        min = std::cmp::min(min, new_state.costs);
                    }
                    new_states.push(new_state);
                }
            });
        });
        if min < usize::MAX {
            log::info!("current min: {}", min);
        }
        new_states.sort_unstable();
        new_states.dedup();
        states = new_states;
    }
    min
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
struct State {
    corridor: Vec<Option<u8>>,
    rooms: Vec<Vec<Option<u8>>>,
    costs: usize,
    size: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            corridor: vec![None; 11],
            rooms: vec![vec![None; 2]; 4],
            costs: 0,
            size: 2,
        }
    }
}

impl State {
    fn gen_part_2(&self) -> Self {
        let mut result = self.clone();
        result.rooms[0].insert(1, Some(3));
        result.rooms[0].insert(1, Some(3));
        result.rooms[1].insert(1, Some(2));
        result.rooms[1].insert(1, Some(1));
        result.rooms[2].insert(1, Some(1));
        result.rooms[2].insert(1, Some(0));
        result.rooms[3].insert(1, Some(0));
        result.rooms[3].insert(1, Some(2));
        result.size = 4;
        result
    }
    fn new() -> Self {
        Self::default()
    }

    fn possible_moves(&self) -> Vec<(usize, usize)> {
        (0..11usize)
            .cartesian_product(0..11usize)
            .filter(|(from, to)| self.valid_move(*from, *to))
            .collect()
    }

    fn do_move(&mut self, mv: (usize, usize)) -> usize {
        let (from, to) = mv;
        let amphi = self.which_amphi(from);
        let mut steps = 0usize;
        if let Some(from_room) = Self::corridor_to_room(from) {
            let (base_step, room_value) = self.rooms[from_room]
                .iter_mut()
                .rev()
                .enumerate()
                .find(|(_idx, elem)| elem.is_some())
                .expect("some check before was wrong");
            steps += base_step + 1;
            *room_value = None;
        } else {
            self.corridor[from] = None;
        }
        if let Some(to_room) = Self::corridor_to_room(to) {
            let (base_step, room_value) = self.rooms[to_room]
                .iter_mut()
                .rev()
                .enumerate()
                .rev()
                .find(|(_idx, elem)| elem.is_none())
                .expect("some check before was wrong");
            steps += base_step + 1;
            *room_value = amphi;
        } else {
            self.corridor[to] = amphi
        };
        steps += std::cmp::max(from, to) - std::cmp::min(from, to);
        self.costs += steps * 10usize.pow(amphi.unwrap() as u32);
        steps * 10usize.pow(amphi.unwrap() as u32)
    }

    /// only used for the part 1 parsing action
    fn to_room(&mut self, amph: u8, pos: usize) -> bool {
        match Self::room_vaccancy(&self.rooms[pos]) {
            // free room
            0 => {
                self.rooms[pos][0] = Some(amph);
                true
            }
            // top spot free
            1 => {
                self.rooms[pos][1] = Some(amph);
                true
            }
            // room is full
            _ => false,
        }
    }

    fn room_to_corridor(room: usize) -> usize {
        (room + 1) * 2
    }

    fn corridor_to_room(corridor: usize) -> Option<usize> {
        if vec![2, 4, 6, 8].contains(&corridor) {
            Some((corridor / 2) - 1)
        } else {
            None
        }
    }

    fn valid_move(&self, from: usize, to: usize) -> bool {
        if self.goal_reached() {
            return false;
        }
        log::trace!("from: {}, to: {}", from, to);
        if from == to {
            log::trace!("from == to -> false");
            return false;
        }
        if let Some(from_room) = Self::corridor_to_room(from) {
            if self.is_goal(from_room) {
                log::trace!("pos {} is already in goal state -> false", from_room);
                return false;
            }
            if let Some(amphi) = self.which_amphi(from) {
                if amphi as usize == from_room
                    && self.rooms[from_room]
                        .iter()
                        .filter(|x| x.is_some())
                        .all(|x| *x == Some(amphi))
                {
                    log::trace!("amphi already at right position -> false");
                    return false;
                }
            }
        } else {
            if Self::corridor_to_room(to).is_none() {
                log::trace!("amphi moving on the corridor -> false");
                return false;
            }
        }
        if self.which_amphi(from).is_none() {
            return false;
        }
        let lower = std::cmp::min(from, to);
        let upper = std::cmp::max(from, to);
        log::trace!(
            "corridor: {:?}, filtercount: {}",
            &self.corridor[lower..=upper],
            self.corridor[lower..=upper]
                .iter()
                .filter(|pos| pos.is_some())
                .count()
        );
        match self.corridor[lower..=upper]
            .iter()
            .filter(|pos| pos.is_some())
            .count()
        {
            0 => {
                if let Some(to_room) = Self::corridor_to_room(to) {
                    self.free_rooms()
                        .filter(|r| {
                            *r == (self.which_amphi(from).unwrap_or(u8::MAX) as usize)
                                && *r == to_room
                        })
                        .count()
                        == 1
                } else {
                    true
                }
            }
            1 => {
                Self::corridor_to_room(from).is_none()
                    && if let Some(to_room) = Self::corridor_to_room(to) {
                        self.free_rooms()
                            .filter(|r| {
                                *r == (self.which_amphi(from).unwrap_or(u8::MAX) as usize)
                                    && *r == to_room
                            })
                            .count()
                            == 1
                    } else {
                        false
                    }
            }
            _ => false,
        }
    }

    fn which_amphi(&self, pos: usize) -> Option<u8> {
        if let Some(room) = Self::corridor_to_room(pos) {
            match self.rooms[room].iter().rev().find(|x| x.is_some()) {
                Some(x) => *x,
                None => None,
            }
        } else {
            self.corridor[pos]
        }
    }

    fn room_vaccancy(room: &[Option<u8>]) -> usize {
        room.iter().filter(|x| x.is_some()).count()
    }

    fn free_rooms(&self) -> impl Iterator<Item = usize> + '_ {
        self.rooms
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(idx, room)| {
                Self::room_vaccancy(room) < self.size
                    && room
                        .iter()
                        .filter(|r| {
                            if let Some(val) = r {
                                *val as usize != *idx
                            } else {
                                false
                            }
                        })
                        .count()
                        == 0
            })
            .map(|(idx, _room)| idx)
    }

    fn is_goal(&self, pos: usize) -> bool {
        self.rooms[pos]
            .iter()
            .filter(|x| {
                if let Some(val) = x {
                    *val as usize == pos
                } else {
                    false
                }
            })
            .count()
            == self.size
    }

    fn goal_reached(&self) -> bool {
        (0..4).all(|i| self.is_goal(i))
    }
}

fn parse(input: &str) -> State {
    let mut lineit = input.split("\n");
    let mut result = State::default();
    if let Some(line) = lineit.nth(2) {
        if let Some(line2) = lineit.next() {
            let mut substr = &line[2..];
            let mut substr2 = &line2[2..];
            (0..4).for_each(|pos| {
                result.to_room(
                    value_of(substr2.chars().nth(1).expect("amphipod id expected")),
                    pos,
                );
                result.to_room(
                    value_of(substr.chars().nth(1).expect("amphipod id expected")),
                    pos,
                );
                substr = &substr[2..];
                substr2 = &substr2[2..];
            });
        }
    }
    result
}

fn value_of(input: char) -> u8 {
    match input {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use test_log::test;

    fn input() -> &'static str {
        indoc! {"
            #############
            #...........#
            ###B#C#B#D###
              #A#D#C#A#
              #########
            
	"}
    }

    #[test]
    fn parse() {
        let result = super::parse(input());
        log::debug!("{:?}", result);
        assert_eq!(
            result,
            State {
                corridor: vec![None; 11],
                rooms: vec![
                    vec![Some(0), Some(1)],
                    vec![Some(3), Some(2)],
                    vec![Some(2), Some(1)],
                    vec![Some(0), Some(3)]
                ],
                costs: 0,
                size: 2,
            }
        );
    }

    #[test]
    fn isgoal() {
        let result = super::parse(input());

        (0..3).for_each(|i| {
            assert!(!result.is_goal(i));
        });

        let test = State {
            corridor: vec![None; 11],
            rooms: vec![
                vec![Some(0), Some(1)],
                vec![Some(3), Some(1)],
                vec![Some(2), Some(2)],
                vec![Some(0), Some(3)],
            ],
            costs: 0,
            size: 2,
        };

        assert!(!test.is_goal(0));
        assert!(!test.is_goal(1));
        assert!(test.is_goal(2));
        assert!(!test.is_goal(3));

        let test = State {
            corridor: vec![None; 11],
            rooms: vec![
                vec![Some(0), Some(0)],
                vec![Some(1), Some(1)],
                vec![Some(2), Some(2)],
                vec![Some(3), Some(3)],
            ],
            costs: 1,
            size: 2,
        };

        assert!(test.goal_reached());
    }

    #[test]
    fn possible_moves() {
        let result = super::parse(input());
        log::debug!("{:?}", result.possible_moves());
        assert_eq!(result.possible_moves().len(), 28);
    }

    #[test]
    fn do_move() {
        let mut result = super::parse(input());
        assert_eq!(result.do_move((6, 3)), 40);
        assert_eq!(result.do_move((4, 6)), 400);
        assert_eq!(
            result.possible_moves(),
            vec![
                (2, 0),
                (2, 1),
                (4, 5),
                (4, 7),
                (4, 9),
                (4, 10),
                (8, 5),
                (8, 7),
                (8, 9),
                (8, 10)
            ]
        );
        assert_eq!(result.do_move((4, 5)), 3_000);
        assert_eq!(
            result.possible_moves(),
            vec![(2, 0), (2, 1), (3, 4), (8, 7), (8, 9), (8, 10)]
        );
    }

    #[test]
    fn part1() {
        let input = super::parse(input());
        assert_eq!(super::part1(&input), 12_521);
    }
}
