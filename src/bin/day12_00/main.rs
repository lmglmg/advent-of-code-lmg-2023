use std::collections::BTreeMap;

fn main() {
    let lines = std::io::stdin().lines();

    let total_combinations: usize = lines
        .map_while(Result::ok)
        .map(process_line)
        .sum();

    println!("{}", total_combinations);
}

fn process_line(s: String) -> usize {
    let (map_part, record) = s.split_once(' ').unwrap();

    let map_part = map_part.as_bytes();

    let consecutive_broken_wells: Vec<usize> = record
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    possible_well_combinations(map_part, &consecutive_broken_wells)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PartialSolvedState {
    num_runs    : usize,
    ending_index: usize,
}

fn possible_well_combinations(map_part: &[u8], consecutive_broken_wells: &[usize]) -> usize {

    let mut possible_subset_combinations: BTreeMap<PartialSolvedState, usize> = BTreeMap::new();

    let total_len  = map_part                .len();
    let total_runs = consecutive_broken_wells.len();

    let minimal_total_length: usize = consecutive_broken_wells
        .iter()
        .map(|x| x+1)
        .sum();

    let minimal_total_length = minimal_total_length - 1;

    let possible_starting_positions: Vec<usize> = consecutive_broken_wells
        .iter()
        .scan(0, |acc, &x| {
            let current = *acc;
            *acc += x + 1;
            Some(current)
        })
        .collect();

    let possible_ending_positions = possible_starting_positions
        .iter()
        .map(|&x| x + total_len - minimal_total_length + 1)
        .collect::<Vec<_>>();

    for consecutive_element_index in 0..total_runs {
        let starting_pos = possible_starting_positions[consecutive_element_index];
        let ending_pos   = possible_ending_positions  [consecutive_element_index];

        let mut possible_states_for_run = 0usize;

        for map_pos in starting_pos..ending_pos {
            let run_length = consecutive_broken_wells[consecutive_element_index];

            let mut is_possible = map_part[map_pos..map_pos+run_length]
                .iter()
                .all(|&x| x == b'?' || x == b'#');
            println!("({}) {}", map_pos, is_possible);

            if map_pos > 1 {
                is_possible &= map_part[map_pos-1] != b'#';
            }

            if map_pos + run_length < total_len {
                is_possible &= map_part[map_pos+run_length] != b'#';
            }

            let map_index = PartialSolvedState{
                num_runs    : consecutive_element_index,
                ending_index: map_pos + run_length + 1,
            };

            if is_possible {
                if consecutive_element_index == 0 {
                    possible_states_for_run += 1;

                } else {
                    let possible_combinations_until_here = possible_subset_combinations
                        .get(&PartialSolvedState{
                            num_runs    : consecutive_element_index - 1,
                            ending_index: map_pos,
                        })
                        .unwrap();
                    possible_states_for_run += possible_combinations_until_here;
                }
            }

            println!("{:02} | {:02} {:02} -> {} + {}", consecutive_element_index, map_pos, run_length, possible_states_for_run, is_possible);

            possible_subset_combinations.insert(map_index, possible_states_for_run);
        }
    }

    dbg!(&possible_subset_combinations);

    *possible_subset_combinations
        .get(&PartialSolvedState{
            num_runs    : total_runs - 1,
            ending_index: total_len + 1,
        })
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_combination_1() {
        assert_eq!(possible_well_combinations(b"#.#.###", &[1, 1, 3]), 1);
        assert_eq!(possible_well_combinations(b"???.###", &[1, 1, 3]), 1);
    }

    #[test]
    fn test_one_combination_2() {
        assert_eq!(possible_well_combinations(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
    }

    #[test]
    fn test_one_combination_3() {
        assert_eq!(possible_well_combinations(b"????.#...#... 4,1,1", &[4, 1, 1]), 1);
    }

    #[test]
    fn test_four_combination_1() {
        assert_eq!(possible_well_combinations(b".??..??...?##.", &[1, 1, 3]), 4);
    }

    #[test]
    fn test_four_combination_2() {
        assert_eq!(possible_well_combinations(b"????.######..#####.", &[1, 6, 5]), 4);
        //                                      0123456789012345678
    }

    #[test]
    fn test_ten_combination() {
        assert_eq!(possible_well_combinations(b"?###????????", &[3, 2, 1]), 10);
    }

    #[test]
    fn test_custom_1() {
        assert_eq!(possible_well_combinations(b"???????", &[3, 3]), 1);
    }
}