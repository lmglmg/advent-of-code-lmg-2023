use advent_of_code_lmg_2023::read_matrix_from_input;

fn main() {
    let input = read_matrix_from_input();

    let mut row_distance = vec![2i64; input.height() as usize];
    let mut col_distance = vec![2i64; input.width () as usize];

    let mut element_indices: Vec<(i64, i64)> = Vec::new();

    for (h, w, element) in input.iter(){
        if element != b'.' {
            row_distance[h as usize] = 1;
            col_distance[w as usize] = 1;
            element_indices.push((h, w));
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

            let dist_h = (row_accumulated_distances[h2 as usize] - row_accumulated_distances[h1 as usize]).abs();
            let dist_w = (col_accumulated_distances[w2 as usize] - col_accumulated_distances[w1 as usize]).abs();

            let dist = dist_h + dist_w;

            distance_sum += dist;
        }
    }

    println!("{}", distance_sum);
}