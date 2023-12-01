const PATTERNS: &[&str]= &[ "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8", "eight", "9", "nine" ];

use aho_corasick::{AhoCorasick, PatternID};

#[inline(always)]
fn pattern_to_value(pattern: PatternID) -> i32 {
    pattern.as_i32() / 2 + 1
}

#[inline(always)]
fn tuning_param(finder: &AhoCorasick, s: String) -> i32 {
    let mut iter = finder.find_overlapping_iter(&s);

    let     first_digit = pattern_to_value(iter.next().unwrap().pattern());
    let mut last_digit = first_digit;

    for m in iter {
        last_digit = pattern_to_value(m.pattern());
    }

    first_digit * 10 + last_digit
}

fn main() {
    let stdin = std::io::stdin();

    let finder = AhoCorasick::new(PATTERNS).unwrap();

    let sum: i32 = stdin
        .lines()
        .map_while(Result::ok)
        .map(|s| tuning_param(&finder, s))
        .sum();

    println!("{}", sum);
}