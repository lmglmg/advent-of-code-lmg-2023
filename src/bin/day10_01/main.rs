fn main() {
    let lines = std::io::stdin().lines();

    let pipes_map: Vec<_> = lines
        .map_while(Result::ok)
        .map(String::into_bytes)
        .collect();

    let height = pipes_map.len();
    let width = pipes_map[0].len();

    let mut distances = vec![vec![-1; width]; height];

    let mut max_distance = 0;

    let starting_coordinate = find_starting_coordinate(&pipes_map);

    distances[starting_coordinate.0][starting_coordinate.1] = 0;

    let mut queue = std::collections::VecDeque::new();

    // Initialize the queue with the starting coordinate and its neighbors
    for i in (starting_coordinate.0.saturating_sub(1)).max(0)..(starting_coordinate.0+2).min(height) {
        for j in (starting_coordinate.1.saturating_sub(1)).max(0)..(starting_coordinate.1+2).min(width) {
            if let Some((diff_a, diff_b)) = pipes_map[i][j].diff() {
                let coord_a = add((i, j), diff_a);
                let coord_b = add((i, j), diff_b);

                if coord_a == starting_coordinate || coord_b == starting_coordinate {
                    queue.push_back((i, j));
                    distances[i][j] = 1;
                    max_distance = 1;
                }
            }
        }
    }

    while let Some((i, j)) = queue.pop_front() {
        let current_distance = distances[i][j];

        let (diff_a, diff_b) = pipes_map[i][j].diff().unwrap();

        let coord_a = add((i, j), diff_a);
        let coord_b = add((i, j), diff_b);

        if coord_a.0 < height && coord_a.1 < width && distances[coord_a.0][coord_a.1] == -1 {
            max_distance = max_distance.max(current_distance + 1);
            distances[coord_a.0][coord_a.1] = current_distance + 1;
            queue.push_back(coord_a);
        }

        if coord_b.0 < height && coord_b.1 < width && distances[coord_b.0][coord_b.1] == -1 {
            max_distance = max_distance.max(current_distance + 1);
            distances[coord_b.0][coord_b.1] = current_distance + 1;
            queue.push_back(coord_b);
        }
    }

    println!("{}", max_distance);

    let mut enclosed_count = 0;

    for i in 0..height {
        let mut is_in_enclosed_area = false;
        for j in 0..width {
            match pipes_map[i][j] {
                b'.' => {
                    if is_in_enclosed_area {
                        enclosed_count += 1;
                    }
                },
                b'|' | b'7' | b'F' | b'S' => {
                    if distances[i][j] != -1 {
                        is_in_enclosed_area = !is_in_enclosed_area;
                    } else if is_in_enclosed_area {
                        enclosed_count += 1;
                    }
                },
                _ => {
                    if distances[i][j] == -1 && is_in_enclosed_area {
                        enclosed_count += 1;
                    }
                }
            }
        }
    }

    println!("{}", enclosed_count);
}

fn find_starting_coordinate(pipes_map: &[Vec<u8>]) -> (usize, usize) {
    for (i, row) in pipes_map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'S' {
                return (i, j);
            }
        }
    }

    unreachable!()
}

trait CoordinateDiff {
    fn diff(&self) -> Option<((isize, isize), (isize, isize))>;
}

impl CoordinateDiff for u8 {
    fn diff(&self) -> Option<((isize, isize), (isize, isize))> {
        match self {
            b'-' => Some(((0, -1), (0, 1))),
            b'|' => Some(((-1, 0), (1, 0))),

            b'L' => Some(((-1, 0), (0, 1))),
            b'J' => Some(((-1, 0), (0, -1))),
            b'F' => Some(((1, 0), (0, 1))),
            b'7' => Some(((1, 0), (0, -1))),
            _ => None
        }
    }
}

fn add(a: (usize, usize), b: (isize, isize)) -> (usize, usize) {
    (a.0.checked_add_signed(b.0).unwrap(), a.1.checked_add_signed(b.1).unwrap())
}