#[derive(Default, Clone, Copy)]
struct SingleRangeMap {
    pub source_start: i64,
    pub target_start: i64,
    pub len: i64,
}

#[derive(Default, Clone)]
struct FullRangeMap {
    pub ranges: Vec<SingleRangeMap>,
}

impl FullRangeMap {
    pub fn add(&mut self, source_start: i64, target_start: i64, len: i64) {
        self.ranges.push(SingleRangeMap {
            source_start,
            target_start,
            len,
        });
        self.ranges.sort_by_key(|r| r.source_start);
    }

    pub fn get(&self, source: i64) -> i64 {
        for range in &self.ranges {
            if source >= range.source_start && source < range.source_start + range.len {
                return range.target_start + (source - range.source_start);
            }
            if source < range.source_start {
                return source;
            }
        }

        source
    }
}

fn main() {
    let mut maps = vec![FullRangeMap::default(); 7];
    let mut map_index = -1;

    let seed_ranges: Vec<i64> = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.is_empty() {
            continue;
        }

        if line.ends_with(" map:") {
            map_index += 1;
            continue;
        }

        let indices: Vec<i64> = line
            .split_whitespace()
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        // Note! Order is a bit weird here!
        maps[map_index as usize].add(
            indices[1],
            indices[0],
            indices[2],
        );
    }

    let mut lowest_location = None;

    for seed_range in seed_ranges.chunks_exact(2) {
        let start_range = seed_range[0];
        let range_len = seed_range[1];

        for seed in start_range..start_range+range_len {
            let mut location = seed;

            for map in &maps {
                location = map.get(location);
            }

            match lowest_location {
                None => lowest_location = Some(location),
                Some(l) => if location < l {
                    lowest_location = Some(location);
                }
            }
        }
    }

    println!("{}", lowest_location.unwrap());
}