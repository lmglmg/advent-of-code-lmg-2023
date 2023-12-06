fn read_elements(s: &str, prefix: &str) -> Vec<i64>  {
    s.strip_prefix(prefix)
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn possible_outcomes(time: f64, distance: f64) -> i64 {
    // Add eps in order to satisfy the condition that
    // the distance traveled must be larger than specified.
    //
    // (D+eps) = X * (T - X) = TX - X^2
    // X^2 - TX + (D+eps) = 0
    // X = (T +- sqrt(T^2 - 4(D+eps))) / 2

    const EPS: f64 = 1e-6;

    let discriminant = time * time - 4.0 * (distance + EPS);

    if discriminant < 0.0 {
        println!("IMPOSSIBLE");
        return 0;
    }

    let discriminant_sqrt = discriminant.sqrt();

    let x_0 = (time - discriminant_sqrt) * 0.5;
    let x_1 = (time + discriminant_sqrt) * 0.5;

    let x_0 = x_0.ceil () as i64;
    let x_1 = x_1.floor() as i64;

    x_1 - x_0 + 1
}

fn main() {
    let mut lines = std::io::stdin().lines();

    let times = lines.next().unwrap().unwrap();
    let times = read_elements(&times, "Time:");

    let distances = lines.next().unwrap().unwrap();
    let distances = read_elements(&distances, "Distance:");

    assert_eq!(times.len(), distances.len());

    let possible_outcomes: i64 = times.iter().zip(distances.iter())
        .map(|(&time, &distance)| possible_outcomes(time as f64, distance as f64))
        .product();

    println!("{}", possible_outcomes);
}