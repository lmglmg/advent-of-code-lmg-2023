use std::collections::BTreeMap;

fn main() {
    let lines = std::io::stdin().lines();

    let total_combinations: usize = lines
        .map_while(Result::ok)
        .inspect(| x| println!("{}", x))
        .map(process_line)
        .inspect(|x| println!("{}", x))
        .sum();

    println!("{}", total_combinations);
}

fn simplify_line(line: &[u8]) -> Vec<u8> {
    let mut iter = line.split(|&x| x == b'.').filter(|&x| x.len() > 0);
    let mut result = Vec::with_capacity(line.len());

    while let Some(x) = iter.next() {
        result.extend_from_slice(x);
        result.push(b'.');
    }

    result
}

fn process_line(s: String) -> usize {
    let (map_part, record) = s.split_once(' ').unwrap();

    let mut full_map = Vec::with_capacity(map_part.len() * 5 + 4);
    for _ in 0..4 {
        full_map.extend_from_slice(map_part.as_bytes());
        full_map.push(b'?')
    }
    full_map.extend_from_slice(map_part.as_bytes());
    let full_map = simplify_line(&full_map);

    let consecutive_broken_wells: Vec<usize> = record
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut full_consecutive_broken_wells = Vec::with_capacity(consecutive_broken_wells.len() * 5);
    for _ in 0..5 {
        full_consecutive_broken_wells.extend_from_slice(&consecutive_broken_wells);
    }

    possible_well_combinations(&full_map, &full_consecutive_broken_wells)
}

fn is_valid(fully_resolved_line: &[u8], consecutive_broken_wells: &[usize]) -> bool {
    let broken_well_ranges: Vec<_> = fully_resolved_line
        .split(|&x| x == b'.')
        .map(|x| x.len())
        .filter(|&x| x > 0)
        .collect();
    consecutive_broken_wells == broken_well_ranges
}

fn resolve_and_backtrack(
    mem: &mut BTreeMap<Vec<u8>, usize>,
    line: &mut [u8],
    consecutive_broken_wells: &[usize]
) -> usize {
    if let Some(&x) = mem.get(line) {
        return x;
    }

    let mut line = simplify_line(line);

    match line.iter().position(|&x| x == b'?') {
        Some(i) => {

            // Check early if points are unfeasible
            let existing_consectutive_wells = line[..i]
                .split(|&x| x == b'.')
                .map(|x| x.len())
                .filter(|&x| x > 0)
                .collect::<Vec<_>>();

            let existing_consucutive_wells_len = existing_consectutive_wells.len();

            if existing_consucutive_wells_len > consecutive_broken_wells.len() {
                return 0;
            }

            if existing_consectutive_wells[..existing_consucutive_wells_len.saturating_sub(1)] != consecutive_broken_wells[..existing_consucutive_wells_len.saturating_sub(1)] {
                return 0;
            }

            if let Some(last_existing_consecutive_well_len) = existing_consectutive_wells.last() {
                if last_existing_consecutive_well_len > &consecutive_broken_wells[existing_consucutive_wells_len.saturating_sub(1)] {
                    return 0;
                }
            }

            let mut total = 0;

            line[i] = b'.';
            total += resolve_and_backtrack(mem, &mut line, consecutive_broken_wells);

            line[i] = b'#';
            total += resolve_and_backtrack(mem, &mut line, consecutive_broken_wells);

            line[i] = b'?';

            mem.insert(line, total);

            total
        },
        None => {
            if is_valid(&line, consecutive_broken_wells) {
                1
            } else {
                0
            }
        }
    }
}

fn possible_well_combinations(map_part: &[u8], consecutive_broken_wells: &[usize]) -> usize {
    let line = map_part.to_owned();
    let mut mem: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
    resolve_and_backtrack(&mut mem, &mut line.into_boxed_slice(), consecutive_broken_wells)
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
    fn test_one_combination_1_2() {
        assert_eq!(possible_well_combinations(b"???.###????.###", &[1, 1, 3, 1, 1, 3]), 1);
    }

    #[test]
    fn test_one_combination_1_5() {
        assert_eq!(possible_well_combinations(b"???.###????.###????.###????.###????.###", &[1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]), 1);
    }

    #[test]
    fn test_one_combination_2() {
        assert_eq!(possible_well_combinations(b"?#?#?#?#?#?#?#?", &[1, 3, 1, 6]), 1);
    }

    #[test]
    fn test_one_combination_3() {
        assert_eq!(possible_well_combinations(b"????.#...#...", &[4, 1, 1]), 1);
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