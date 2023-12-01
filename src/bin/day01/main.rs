#[inline(always)]
fn to_digit_value(c: u8) -> i32 {
    match c {
        b'0'..=b'9' => (c - b'0') as i32,
        _ => panic!("Invalid digit"),
    }
}

fn tuning_param(s: String) -> i32 {
    let s = s.as_bytes(); // No need for utf8 handling

    let first_digit = s.iter().find(|c| c.is_ascii_digit()).unwrap();
    let last_digit = s.iter().rev().find(|c| c.is_ascii_digit()).unwrap();

    let first_digit = to_digit_value(*first_digit);
    let last_digit  = to_digit_value(*last_digit);

    first_digit * 10 + last_digit
}

fn main() {
    let stdin = std::io::stdin();

    let sum: i32 = stdin
        .lines()
        .filter_map(Result::ok)
        .map(tuning_param)
        .sum();

    println!("{}", sum);
}