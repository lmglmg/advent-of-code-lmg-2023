use advent_of_code_lmg_2023::read_matrix_from_input;

const EXPANSION_FACTOR: i64 = 1000000i64;

fn main() {
    let input = read_matrix_from_input();

    let mut row_distance = vec![EXPANSION_FACTOR; input.height() as usize];
    let mut col_distance = vec![EXPANSION_FACTOR; input.width () as usize];

    let mut element_indices: Vec<(usize, usize)> = Vec::new();

    for (h, w, element) in input.iter(){
        if element != b'.' {
            row_distance[h as usize] = 1;
            col_distance[w as usize] = 1;
            element_indices.push((h as usize, w as usize));
        }
    }

    let row_accumulated_distances: Vec<_> = row_distance
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    let col_accumulated_distances: Vec<_> = col_distance
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    let mut distance_sum = 0;

    for i in 0..element_indices.len() {
        for j in i+1..element_indices.len() {
            let (h1, w1) = element_indices[i];
            let (h2, w2) = element_indices[j];

            let dist_h = (row_accumulated_distances[h2] - row_accumulated_distances[h1]).abs();
            let dist_w = (col_accumulated_distances[w2] - col_accumulated_distances[w1]).abs();

            distance_sum += dist_h + dist_w;
        }
    }

    println!("{}", distance_sum);
}