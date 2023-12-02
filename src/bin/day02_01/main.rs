#[derive(Copy, Clone, Default)]
struct Game {
    red  : i32,
    green: i32,
    blue : i32,
}

fn parse_single_game(s: &str) -> Game {
    s.split(',')
        .map(str::trim)
        .map(|s| s.split_once(' ').unwrap())
        .map(|(balls_count, color)| (balls_count.parse().unwrap(), color))
        .map(|(balls_count, color)| match color {
            "red"   => Game { red  : balls_count, ..Game::default() },
            "green" => Game { green: balls_count, ..Game::default() },
            "blue"  => Game { blue : balls_count, ..Game::default() },
            _ => panic!("Unknown color: {}", color),
        })
        .fold(Game::default(), per_element_max)
}

fn per_element_max(a: Game, b: Game) -> Game {
    Game {
        red  : a.red  .max(b.red  ),
        green: a.green.max(b.green),
        blue : a.blue .max(b.blue ),
    }
}

fn game_power_level(s: String) -> i32 {
    let s = s.strip_prefix("Game ").unwrap();

    let (_, games) = s.split_once(':').unwrap();

    let max_game = games
        .split(';')
        .map(parse_single_game)
        .fold(Game::default(), per_element_max);

    max_game.red * max_game.green * max_game.blue
}

fn main () {
    let sum: i32 = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(game_power_level)
        .sum();

    println!("{}", sum);
}