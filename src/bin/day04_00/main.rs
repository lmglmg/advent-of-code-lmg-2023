use std::collections::BTreeSet;

fn to_set(s: &str) -> BTreeSet<i32> {
    s.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn winning_score(s: String) -> Option<i32> {
    let (_, interesting_part) = s.split_once(':')?;

    let (my_numbers, winning_numbers) = interesting_part.split_once('|')?;

    let my_numbers     : BTreeSet<i32> = to_set(my_numbers);
    let winning_numbers: BTreeSet<i32> = to_set(winning_numbers);

    let intersection = my_numbers.intersection(&winning_numbers);

    match intersection.count() {
        0 => None,
        c => Some(1 << ( c - 1 ))
    }
}

fn main () {
    let sum: i32 = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter_map(winning_score)
        .sum();

    println!("{}", sum);
}