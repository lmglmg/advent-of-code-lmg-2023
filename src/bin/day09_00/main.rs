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
        if numbers[0..sz].iter().all(|&x| x == 0) {
            return numbers[sz..].iter().sum();
        }
        for i in 0..sz-1  {
            numbers[i] = numbers[i+1] - numbers[i];
        }

        sz -= 1;
    }

    unreachable!()
}