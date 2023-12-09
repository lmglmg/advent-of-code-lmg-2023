fn main() {
    let lines = std::io::stdin().lines();

    let total: i64 = lines
        .map_while(Result::ok)
        .map(parse_line)
        .map(get_prediction)
        .sum();

    println!("{}", total);
}

fn parse_line(s: String) -> Vec<i64> {
    s.split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn get_prediction(numbers: Vec<i64>) -> i64 {
    let mut sz = numbers.len();

    let mut numbers = numbers;

    while sz > 0 {
        let mid = numbers.len() - sz;

        if numbers[mid-sz..].iter().all(|&x| x == 0) {
            return numbers[0..mid]
                .iter()
                .enumerate()
                .map(|(i, &x)| if i % 2 == 0 { x } else { -x })
                .sum()
        }

        for i in (mid..numbers.len()-1).rev()  {
            numbers[i+1] = numbers[i+1] - numbers[i];
        }

        sz -= 1;
    }

    unreachable!()
}