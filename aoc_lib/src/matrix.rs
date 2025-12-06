use std::ops::Div;

#[derive(Debug, Default)]
pub struct Matrix<T> {
    pub size: (usize, usize),
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn extend(&mut self, values: Vec<T>) {
        self.data.extend(values);
    }

    pub fn is_valid(&self) -> bool {
        self.data.len() == self.size.0 * self.size.1
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        MatrixIterator::new(self, Direction::LeftRight)
    }

    pub fn transpose_iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        MatrixIterator::new(self, Direction::UpDown)
    }
}

struct MatrixIterator<'a, T> {
    matrix: &'a Matrix<T>,
    pos: usize,
    size: usize,
    direction: Direction,
}

impl<'a, T> MatrixIterator<'a, T> {
    fn new(matrix: &'a Matrix<T>, direction: Direction) -> MatrixIterator<'a, T> {
        let size = matrix.size.0 * matrix.size.1;
        MatrixIterator {
            matrix,
            pos: 0,
            direction,
            size,
        }
    }

    fn next_left_right(&mut self) -> Option<(usize, usize, &'a T)> {
        if self.size == 0 || self.pos > self.size - 1 {
            return None;
        }
        let x = self.pos % self.matrix.size.0;
        let y = self.pos.div(self.matrix.size.0);
        self.pos += 1;
        Some((x, y, &self.matrix.data[self.pos - 1]))
    }

    fn next_up_down(&mut self) -> Option<(usize, usize, &'a T)> {
        if self.size == 0 || self.pos > self.size - 1 {
            return None;
        }
        let x = self.pos.div(self.matrix.size.1);
        let y = self.pos % self.matrix.size.1;
        self.pos += 1;
        let idx = y * self.matrix.size.0 + x;
        Some((x, y, &self.matrix.data[idx]))
    }
}

impl<'a, T> Iterator for MatrixIterator<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::LeftRight => self.next_left_right(),
            Direction::UpDown => self.next_up_down(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    LeftRight,
    UpDown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut matrix = Matrix::default();
        matrix.size = (3, 2);
        matrix.push("a");
        matrix.push("b");
        matrix.push("c");
        matrix.extend(vec!["d", "e", "f"]);
        assert!(matrix.is_valid());

        for (x, y, item) in matrix.iter() {
            println!("{} {} {}", x, y, item);
        }
        println!("------------");
        for (x, y, item) in matrix.transpose_iter() {
            println!("{} {} {}", x, y, item);
        }
    }
}
