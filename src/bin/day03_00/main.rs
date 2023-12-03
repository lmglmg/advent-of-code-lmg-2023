use std::mem::swap;

#[derive(Default)]
struct SymbolLocations(Vec<usize>);

impl SymbolLocations {
    pub fn push(&mut self, index: usize) {
        if let Some(last) = self.0.last() {
           assert!(*last < index);
        }
        self.0.push(index);
    }

    pub fn are_symbols_within_range(&self, min: usize, max: usize) -> bool {
        let first_possible_index = self.0.binary_search(&min).unwrap_or_else(|x| x);

        for i in first_possible_index..self.0.len() {
            let symbol_index = self.0[i];
            if symbol_index >= min && symbol_index <= max {
                return true;
            }
            if symbol_index > max {
                return false;
            }
        }
        false
    }
}

#[derive(Default)]
struct SmybolLocationBuffer{
    pub prev: SymbolLocations,
    pub curr: SymbolLocations,
    pub next: SymbolLocations,
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

#[derive(Default)]
struct NumberLocationBuffer{
    pub curr: Vec<NumberLocation>,
    pub next: Vec<NumberLocation>,
}

impl NumberLocationBuffer {
    pub fn advance(&mut self) {
        // Swap the buffers. Clear the next buffer. This should prevent many allocations.
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

        for number in &self.number_locations.curr {
            let min_location = number.start.saturating_sub(1) as usize;
            let max_location = number.start as usize + number.len as usize;

            if
                self.symbol_locations.prev.are_symbols_within_range(min_location, max_location) ||
                self.symbol_locations.curr.are_symbols_within_range(min_location, max_location) ||
                self.symbol_locations.next.are_symbols_within_range(min_location, max_location)
            {
                sum += number.value;
            }
        }

        sum
    }

    fn parse_next_line(&mut self, line: &[u8]) {
        let mut number_location = None;

        // Parse the line
        for i in 0..line.len() {
            let c = line[i];
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
                    if c != b'.' {
                        self.symbol_locations.next.push(i);
                    }
                }
                // No digit, but running number. Finish it.
                (false, Some(_)) => {
                    self.number_locations.next.push(number_location.take().unwrap());
                    if c != b'.' {
                        self.symbol_locations.next.push(i);
                    }
                }
            }
        }
        // Finish the running number
        match number_location {
            Some(val) => self.number_locations.next.push(val),
            None => {}
        }
    }
}

fn main() {

    let mut processor = Processor::default();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line = line.as_bytes();
        processor.process_line(line);
    }

    processor.process_line(&Vec::new());

    println!("{}", processor.sum)
}