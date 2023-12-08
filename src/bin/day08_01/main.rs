use std::collections::BTreeMap;
use num::integer::lcm;
use advent_of_code_lmg_2023::*;

fn main() {
    let mut lines = std::io::stdin().lines();

    let choices_base = lines.read_line().into_bytes();

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
        .map(|&starting_location| cyclic_path(&paths, starting_location, &choices_base))
        .fold(1, lcm);

    println!("{}", full_cyclic_path);
}

fn cyclic_path(
    paths: &BTreeMap<LocationID, (LocationID, LocationID)>,
    starting_location: LocationID,
    choices_base: &[u8],
) -> i64 {
    let mut choices_base_it = choices_base.iter();

    let mut current_location = starting_location;

    let mut steps: i64 = 0;

    while !current_location.is_ending() {
        let (left_location, right_location) = paths.get(&current_location).unwrap();

        let decision = match choices_base_it.next() {
            Some(choice) => choice,
            None => {
                // Reset path
                choices_base_it = choices_base.iter();
                choices_base_it.next().unwrap()
            }
        };

        current_location = match decision {
            b'L' => *left_location,
            b'R' => *right_location,
            _ => unreachable!(),
        };

        steps += 1;
    }

    lcm(steps, choices_base.len() as i64)
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

        Some(Self{location: target, l_choice: l_choice.into(), r_choice: r_choice.into()})
    }
}
