use std::collections::BTreeMap;
use advent_of_code_lmg_2023::*;

fn main() {
    let mut lines = std::io::stdin().lines();

    let decisions_root = lines.read_line().into_bytes();

    lines.skip_line();

    let paths: BTreeMap<LocationID, (LocationID, LocationID)> = lines
        .map_while(Result::ok)
        .map_while(|s| ParsedLine::try_from_str(&s))
        .map(|parsed_line| (parsed_line.location, (parsed_line.l_choice, parsed_line.r_choice)))
        .collect();

    let all_starting_locations: Vec<_> = paths
        .keys()
        .copied()
        .filter(|location| location.is_starting())
        .collect();

    let full_cyclic_path = all_starting_locations
        .iter()
        .map(|&starting_location| cyclic_path(&paths, starting_location, &decisions_root))
        .fold(1, num::integer::lcm);

    println!("{}", full_cyclic_path);
}

fn cyclic_path(
    paths: &BTreeMap<LocationID, (LocationID, LocationID)>,
    starting_location: LocationID,
    decisions_root: &[u8],
) -> i64 {
    let mut decision_root_it = decisions_root.iter();

    let mut current_location = starting_location;

    let mut steps: i64 = 0;

    while !current_location.is_ending() {
        let (l_choice, r_choice) = paths.get(&current_location).unwrap();

        let decision = match decision_root_it.next() {
            Some(choice) => choice,
            None => {
                decision_root_it = decisions_root.iter();
                decision_root_it.next().unwrap()
            }
        };

        match decision {
            b'L' => current_location = *l_choice,
            b'R' => current_location = *r_choice,
            _ => unreachable!(),
        }

        steps += 1;
    }

    num::integer::lcm(steps, decisions_root.len() as i64)
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct LocationID([u8; 3]);

impl LocationID {
    pub fn is_starting(&self) -> bool {
        self.0[2] == b'A'
    }

    pub fn is_ending(&self) -> bool {
        self.0[2] == b'Z'
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct ParsedLine{
    pub location: LocationID,
    pub l_choice: LocationID,
    pub r_choice: LocationID,
}

impl From<&str> for LocationID {
    fn from(value: &str) -> Self {
        Self(value.as_bytes().try_into().unwrap())
    }
}

impl ParsedLine {
    pub fn try_from_str(s: &str) -> Option<Self> {
        let (target, choices) = s.split_once(" = ")?;

        let target = LocationID::from(target.trim());

        let (l_choice, r_choice) = choices
            .strip_prefix('(')?
            .strip_suffix(')')?
            .split_once(", ")?;

        let l_choice = LocationID::from(l_choice);
        let r_choice = LocationID::from(r_choice);

        Some(Self { location: target, l_choice, r_choice })
    }
}
