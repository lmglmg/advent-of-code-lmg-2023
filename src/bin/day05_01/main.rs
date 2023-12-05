#[derive(Clone, Default, Copy, Debug)]
struct SingleRange{
    pub source: i64,
    pub target: i64,
}

#[derive(Clone, Debug)]
struct FullRangeMap {
    pub ranges: Vec<SingleRange>,
}

impl FullRangeMap {
    pub fn new() -> Self {
        Self {
            ranges: vec![
                SingleRange{ source: 0, target: 0 },
                SingleRange{ source: i64::MAX, target: i64::MAX },
            ],
        }
    }

    // Assume that the ranges will not overlap. Watch out for zero
    pub fn add(&mut self, source_start: i64, target_start: i64, length: i64) {
        // Add ranges in pair. And then just sort them to push them back in the right order

        let start_range = SingleRange{
            source: source_start,
            target: target_start,
        };

        let end_range  = SingleRange{
            source: source_start + length,
            target: source_start + length,
        };

        let start_range_index = self.ranges.binary_search_by(|r| r.source.cmp(&start_range.source));

        // If ranges overlap, then just replace the start range.
        if let Ok(index) = start_range_index {
            self.ranges[index] = start_range;
        } else {
            self.ranges.insert(start_range_index.unwrap_err(), start_range);
        }

        let end_range_index = self.ranges.binary_search_by(|r| r.source.cmp(&end_range.source));
        if let Ok(_) = end_range_index {
            // If ranges overlap, just leave the end range as is.
        } else {
            self.ranges.insert(end_range_index.unwrap_err(), end_range);
        }
    }

    fn matching_range_index(&self, source: i64) -> usize {
        self.ranges.binary_search_by(|r| r.source.cmp(&source)).unwrap_or_else(|i| i - 1)
    }

    pub fn matching_range(&self, source: i64) -> SingleRange {
        let range_index = self.matching_range_index(source);
        self.ranges[range_index]
    }

    pub fn get(&self, source: i64) -> i64 {
        let range = self.matching_range(source);
        (source - range.source) + range.target
    }

    fn fuse_composite_ranges(range_a: &Self, range_b: &Self) -> Self {
        let mut result = Self::new();

        for index_a in 0..(range_a.ranges.len()-1) {
            let (a_source_start, a_source_end) = (range_a.ranges[index_a].source, range_a.ranges[index_a+1].source);
            let source_len = a_source_end - a_source_start;
            let a_target_start = range_a.ranges[index_a].target;
            let mut b_range_index = range_b.matching_range_index(a_target_start);
            let mut i = 0;

            while i < source_len {
                let b_range      = &range_b.ranges[b_range_index];
                let b_range_next = &range_b.ranges[b_range_index+1];

                let offset = (a_target_start - range_b.ranges[b_range_index].source) + i;

                let final_target_start = b_range.target + offset;
                let final_range_len = b_range_next.source - b_range.source - offset;

                let range_len = final_range_len.min(source_len - i);

                result.add(a_source_start + i, final_target_start, range_len);

                i += range_len;
                b_range_index += 1;
            }
        }

        result
    }
}



fn main() {
    let mut maps = vec![FullRangeMap::new(); 7];
    let mut map_index: i64 = -1;

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

    let fused_map = maps.iter().fold(FullRangeMap::new(), |a, b| FullRangeMap::fuse_composite_ranges(&a, &b));

    let mut lowest_location = None;

    for seed_range in seed_ranges.chunks_exact(2) {
        let start_range = seed_range[0];
        let range_len = seed_range[1];

        for seed in start_range..start_range+range_len {
            let location = fused_map.get(seed);

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