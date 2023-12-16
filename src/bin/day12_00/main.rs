use std::collections::BTreeMap;

fn main() {
    let lines = std::io::stdin().lines();

    let total_combinations: usize = lines
        .map_while(Result::ok)
        .map(process_line)
        .enumerate()
        .map(|(i, x)| {
            println!("{:3} : {}", i+1, x);
            x
        })
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

fn is_valid(fully_resolved_line: &[u8], consecutive_broken_wells: &[usize]) -> bool {
    let broken_well_ranges: Vec<_> = fully_resolved_line
        .split(|&x| x == b'.')
        .map(|x| x.len())
        .filter(|&x| x > 0)
        .collect();
    consecutive_broken_wells == broken_well_ranges
}

fn resolve_and_backtrack(line: &mut [u8], consecutive_broken_wells: &[usize]) -> usize {
    match line.iter().position(|&x| x == b'?') {
        Some(i) => {
            let mut total = 0;

            line[i] = b'#';
            total += resolve_and_backtrack(line, consecutive_broken_wells);

            line[i] = b'.';
            total += resolve_and_backtrack(line, consecutive_broken_wells);

            line[i] = b'?';

            total
        },
        None => {
            if is_valid(line, consecutive_broken_wells) {
                1
            } else {
                0
            }
        }
    }
}

fn possible_well_combinations(map_part: &[u8], consecutive_broken_wells: &[usize]) -> usize {
    let line = map_part.to_owned();
    resolve_and_backtrack(&mut line.into_boxed_slice(), consecutive_broken_wells)
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

    #[test]
    fn test_wat_1() {
        assert_eq!(possible_well_combinations(b"?..#..?..?", &[1, 1, 1]), 3);
        //                                      x  x  x
        //                                      x  x     x
        //                                      x     x  x
        //                                         x  x  x
    }

    #[test]
    fn test_wat_2() {
        assert_eq!(possible_well_combinations(b"?..#", &[1]), 1);
    }
}