use std::io::Stdin;
use std::collections::BTreeMap;

fn main() {
    let mut lines = std::io::stdin().lines();

    let decisions_root = lines.next().unwrap().unwrap().into_bytes();

    let _ = lines.next().unwrap(); // Skip blank line

    let paths: BTreeMap<LocationID, (LocationID, LocationID)> = lines
        .map_while(Result::ok)
        .map_while(|s| ParsedLine::try_from_str(&s))
        .map(|parsed_line| (parsed_line.location, (parsed_line.l_choice, parsed_line.r_choice)))
        .collect();

    let mut steps: i64 = 0;

    let mut decision_root_it = decisions_root.iter();

    let mut current_location = LocationID([b'A'; 3]);

    while current_location != LocationID([b'Z'; 3]) {
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

    let full_step_count = num::integer::lcm(steps, decisions_root.len() as i64);

    println!("{}", full_step_count);

}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
struct LocationID([u8; 3]);

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
