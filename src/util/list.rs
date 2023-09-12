#[derive(Debug)]
pub struct Array2D<T> {
    line_size: usize,
    raw_data: Vec<T>,
}

impl<T> Array2D<T> {
    pub fn new(line_size: usize) -> Self {
        Array2D {
            line_size,
            raw_data: vec![],
        }
    }

    pub fn add_line(&mut self, line_iter: impl IntoIterator<Item = T>) {
        let len_before = self.raw_data.len();
        self.raw_data.extend(line_iter);
        let len_after = self.raw_data.len();

        if len_after - len_before != self.line_size {
            panic!("line_iter is too big or too small!")
        }
    }

    pub fn len(&self) -> usize {
        self.raw_data.len() / self.line_size
    }

    pub fn len_line(&self) -> usize {
        self.line_size
    }

    pub fn iter_keys(&self) -> Array2DIterKeys<T> {
        Array2DIterKeys {
            data: self,
            x: 0,
            y: 0,
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.raw_data[index.1 * self.line_size + index.0]
    }
}

pub struct Array2DIterKeys<'a, T> {
    data: &'a Array2D<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for Array2DIterKeys<'a, T> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let result = (self.x, self.y);

        if self.x < self.data.len_line() - 1 {
            self.x += 1;
        } else {
            self.x = 0;
            self.y += 1;
        }

        if self.y == self.data.len() {
            return None;
        }

        Some(result)
    }
}
