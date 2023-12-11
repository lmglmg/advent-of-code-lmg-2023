use std::io::{StdinLock, Lines};

pub trait StdinExt {
    fn read_line(&mut self) -> String;

    fn skip_line(&mut self);
}

impl StdinExt for Lines<StdinLock<'_>> {
    fn read_line(&mut self) -> String {
        self.next().unwrap().unwrap()
    }

    fn skip_line(&mut self) {
        let _ = self.next().unwrap().unwrap();
    }

}

pub fn read_all_lines_as_bytes() -> Vec<Vec<u8>> {
    let lines = std::io::stdin().lines();

    lines
        .map_while(Result::ok)
        .map(String::into_bytes)
        .collect::<Vec<_>>()
}

struct ByteMatrixBuilder {
    width: Option<usize>,
    data: Vec<u8>,
}

impl ByteMatrixBuilder {
    pub fn new() -> Self {
        Self {
            width: None,
            data: Vec::new(),
        }
    }

    pub fn add_line(&mut self, line: &[u8]) {
        if let Some(width) = self.width {
            assert_eq!(width, line.len());
        } else {
            self.width = Some(line.len());

            // Pre-allocate a square matrix
            self.data.reserve(line.len() * line.len());
        }

        self.data.extend_from_slice(line);
    }

    pub fn build(self) -> ByteMatrix {
        let width = self.width.unwrap();
        let height = self.data.len() / width;

        ByteMatrix {
            data: self.data,
            width,
            height,
        }
    }
}

pub struct ByteMatrix {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl ByteMatrix {
    pub fn at(&self, x: i64, y: i64) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }
        if x > self.width() || y > self.height() {
            return None;
        }
        let index = (y * self.width() + x) as usize;
        unsafe{ Some(*self.data.get_unchecked(index)) }
    }

    pub fn at_mut(&mut self, x: i64, y: i64) -> Option<&mut u8> {
        if x < 0 || y < 0 {
            return None;
        }
        if x > self.width() || y > self.height() {
            return None;
        }
        let index = (y * self.width() + x) as usize;
        unsafe{ Some(self.data.get_unchecked_mut(index)) }
    }

    pub fn width(&self) -> i64 {
        self.width as i64
    }


    pub fn height(&self) -> i64 {
        self.height as i64
    }

    pub fn iter(&self) -> ByteMatrixIter {
        ByteMatrixIter {
            matrix: self,
            x: 0,
            y: 0,
            total_iterations: 0,
        }
    }
}

pub struct ByteMatrixIter<'a> {
    matrix: &'a ByteMatrix,
    x: i64,
    y: i64,
    total_iterations: i64,
}

impl<'a> Iterator for ByteMatrixIter<'a> {
    type Item = (i64, i64, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_iterations as usize >= self.matrix.data.len() {
            return None;
        }
        let x = self.x;
        let y = self.y;
        let i = self.total_iterations;

        self.x += 1;
        if self.x >= self.matrix.width() {
            self.x = 0;
            self.y += 1;
        }
        self.total_iterations += 1;

        unsafe{ Some((y, x, *self.matrix.data.get_unchecked(i as usize))) }
    }
}

pub fn read_matrix_from_input() -> ByteMatrix {
    let mut builder = ByteMatrixBuilder::new();

    for line in std::io::stdin().lines() {
        builder.add_line(&line.unwrap().into_bytes());
    }

    builder.build()
}
