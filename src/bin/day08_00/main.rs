use std::io::Stdin;

fn main() {
    let mut lines = std::io::stdin().lines();

    let decisions_root = lines.next().unwrap().unwrap();

    let _ = lines.next().unwrap(); // Skip blank line

    let paths: Vec<_> = lines
        .map_while(Result::ok)
        .map_while(|s| ParsedLine::try_from_str(&s))
        .collect();
}

struct LocationID([u8; 3]);

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
