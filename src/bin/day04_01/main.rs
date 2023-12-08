use std::collections::BTreeSet;

fn to_set(s: &str) -> BTreeSet<i32> {
    s.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn winning_tickets(s: String) -> Option<usize> {
    let (_, interesting_part) = s.split_once(':')?;

    let (my_numbers, winning_numbers) = interesting_part.split_once('|')?;

    let my_numbers     : BTreeSet<i32> = to_set(my_numbers);
    let winning_numbers: BTreeSet<i32> = to_set(winning_numbers);

    let intersection = my_numbers.intersection(&winning_numbers);

    Some(intersection.count())
}

fn main () {
    let ticket_scores: Vec<usize> = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter_map(winning_tickets)
        .collect();

    let mut ticket_counts = vec![1usize; ticket_scores.len()];

    for i in (0..ticket_scores.len()).rev() {
        let additional_tickets = ticket_scores[i];
        for j in (i+1)..(i+1+additional_tickets) {
            ticket_counts[i] += ticket_counts[j];
        }
    }

    let total_tickets: usize = ticket_counts.iter().sum();

    println!("{}", total_tickets);
}