use std::mem::swap;

#[derive(Default)]
struct GearLocations(Vec<u16>);

impl GearLocations {
    pub fn push(&mut self, index: u16) {
        if let Some(last) = self.0.last() {
           assert!(*last < index);
        }
        self.0.push(index);
    }
}

#[derive(Default)]
struct SmybolLocationBuffer{
    pub prev: GearLocations,
    pub curr: GearLocations,
    pub next: GearLocations,
}

impl SmybolLocationBuffer {
    pub fn advance(&mut self) {
        // Swap the buffers. Clear the next buffer. This should prevent many allocations.
        swap( &mut self.prev, &mut self.curr);
        swap( &mut self.curr, &mut self.next);
        self.next.0.clear();
    }
}

struct NumberLocation {
    pub value: i32,
    pub start: u16,
    pub len: u8,
}

impl NumberLocation {
    pub fn is_adjecant(&self, gear_index: u16) -> bool {
        (self.start.saturating_sub(1)..self.start + self.len as u16 + 1).contains(&gear_index)
    }
}

fn adjecant_locations(numbers: &[NumberLocation], gear_index: u16) -> Vec<i32> {
    let start_index = gear_index.saturating_sub(1);
    let closest_index = numbers.binary_search_by(|probe| probe.start.cmp(&start_index)).unwrap_or_else(|x| x);

    let mut result = Vec::new();

    let min_index = closest_index.saturating_sub(1);
    let max_index = (closest_index + 2).min(numbers.len());

    for number in numbers.iter().take(max_index).skip(min_index) {
        if number.is_adjecant(gear_index) {
            result.push(number.value);
        }
    }

    result
}

#[derive(Default)]
struct NumberLocationBuffer{
    pub prev: Vec<NumberLocation>,
    pub curr: Vec<NumberLocation>,
    pub next: Vec<NumberLocation>,
}

impl NumberLocationBuffer {
    pub fn advance(&mut self) {
        // Swap the buffers. Clear the next buffer. This should prevent many allocations.
        swap( &mut self.prev, &mut self.curr);
        swap( &mut self.curr, &mut self.next);
        self.next.clear();
    }
}

#[derive(Default)]
struct Processor {
    pub sum: i32,
    pub number_locations: NumberLocationBuffer,
    pub symbol_locations: SmybolLocationBuffer,
}

impl Processor {
    pub fn process_line(&mut self, line: &[u8]) {

        self.number_locations.advance();
        self.symbol_locations.advance();

        self.parse_next_line(line);

        self.sum += self.current_line_sum();
    }

    fn current_line_sum(&self) -> i32 {
        let mut sum = 0;

        for gear_location in &self.symbol_locations.curr.0 {
            let mut all_nubmers = Vec::new();
            all_nubmers.extend(adjecant_locations(&self.number_locations.prev, *gear_location));
            all_nubmers.extend(adjecant_locations(&self.number_locations.curr, *gear_location));
            all_nubmers.extend(adjecant_locations(&self.number_locations.next, *gear_location));


            if all_nubmers.len() == 2 {
                sum += all_nubmers[0] * all_nubmers[1];
            }
        }

        sum
    }

    fn parse_next_line(&mut self, line: &[u8]) {
        let mut number_location = None;

        // Parse the line
        for (i, &c) in line.iter().enumerate() {

            match (c.is_ascii_digit(), &mut number_location) {
                // Start parsing a new number
                (true, None) => {
                    number_location = Some(NumberLocation {
                        value: (c - b'0') as i32,
                        start: i as u16,
                        len: 1,
                    });
                }
                // Continue parsing a number
                (true, Some(val)) => {
                    val.value = val.value * 10 + (c - b'0') as i32;
                    val.len += 1;
                }
                // No digit, no running number
                (false, None) => {
                    if c == b'*' {
                        self.symbol_locations.next.push(i as u16);
                    }
                }
                // No digit, but running number. Finish it.
                (false, Some(_)) => {
                    self.number_locations.next.push(number_location.take().unwrap());
                    if c == b'*' {
                        self.symbol_locations.next.push(i as u16);
                    }
                }
            }

        }

        // Finish the running number
        if let Some(val) = number_location {
            self.number_locations.next.push(val);
        }
    }

    pub fn finalize(self) -> i32 {
        let mut s = self;
        s.process_line(&[]);
        s.sum
    }
}

fn main() {

    let mut processor = Processor::default();

    std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.into_bytes())
        .for_each(|line| processor.process_line(&line));

    let sum = processor.finalize();

    println!("{}", sum)
}