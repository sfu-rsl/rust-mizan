use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::{alloc, fmt, mem, ops};

/// Rectangular table of elements (two-dimensional array).
///
pub struct Matrix<'a, T>
where
    T: Default + Clone,
{
    cols: usize,
    buffer: &'a mut [T],
}

impl<'a, T> Matrix<'a, T>
where
    T: Default + Clone,
{
    /// Creates new Matrix and fills it with default values.
    ///
    /// `rows` - rows number.
    /// `cols` - columns number.
    /// Panic, if memory allocation is not succesfully.
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            cols,
            buffer: Self::alloc(rows, cols),
        }
    }

    /// Fills matrix with a default values.
    ///
    pub fn clear(&mut self) {
        Self::fill_with(self.buffer, T::default());
    }

    /// Fills matrix with a `value`.
    ///
    pub fn fill(&mut self, value: T) {
        Self::fill_with(self.buffer, value);
    }

    /// Returns rows number.
    ///
    pub fn rows(&self) -> usize {
        self.buffer.len() / self.cols
    }

    /// Returns columns number.
    ///
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Returns number of elements in matrix.
    ///
    pub fn elements_number(&self) -> usize {
        self.buffer.len()
    }

    /// Returns the n-th element of the table in line traversal order.
    ///
    pub fn nth(&self, index: usize) -> &T {
        &self.buffer[index]
    }

    /// Returns value at [row][col] position.
    ///
    /// There are bounds checking.
    /// If index out of range, then panic.
    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.buffer[self.linear_index(row, col)]
    }

    /// Sets the `value` of element at [row][col] position.
    ///
    /// There are bounds checking.
    /// If index out of range, then panic.
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.buffer[self.linear_index(row, col)] = value;
    }

    /// Iterator over matrix in line traversal order.
    ///
    pub fn iter(&self) -> Iter<'_, T> {
        self.buffer.iter()
    }

    /// Mutable iterator over matrix in line traversal order.
    ///
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        self.buffer.iter_mut()
    }

    /// Memory allocation for data buffer.
    ///
    fn alloc(rows: usize, cols: usize) -> &'a mut [T] {
        unsafe {
            let buf = alloc::alloc(layout::<T>(rows * cols).unwrap()) as *mut T;
            let slice = std::slice::from_raw_parts_mut(buf, rows * cols);
            Self::fill_with(slice, T::default());
            slice
        }
    }

    /// Fills data buffer with a `value`.
    ///
    fn fill_with(buf: &mut [T], value: T) {
        for e in buf {
            *e = value.clone();
        }
    }

    fn linear_index(&self, row: usize, col: usize) -> usize {
        if row >= self.rows() || col >= self.cols {
            panic!("index out of bounds");
        }
        row * self.cols + col
    }

    fn is_same_size(&self, other: &Self) -> bool {
        self.cols == other.cols && self.buffer.len() == other.buffer.len()
    }
}

impl<'a, T> Drop for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(
                self.buffer.as_mut_ptr() as *mut u8,
                layout::<T>(self.buffer.len()).unwrap(),
            );
        }
    }
}

fn layout<T>(size: usize) -> Result<alloc::Layout, alloc::LayoutErr> {
    alloc::Layout::from_size_align(size * mem::size_of::<T>(), mem::align_of::<T>())
}

impl<'a, T> PartialEq for Matrix<'a, T>
where
    T: Default + Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.cols == other.cols && self.buffer == other.buffer {
            true
        } else {
            false
        }
    }
}

impl<'a, T> Index<usize> for Matrix<'a, T>
where
    T: Default + Clone,
{
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        if row >= self.rows() {
            panic!("index out of bounds")
        }
        &self.buffer[row * self.cols..(row + 1) * self.cols]
    }
}

impl<'a, T> IndexMut<usize> for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        if row >= self.rows() {
            panic!("index out of bounds")
        }
        &mut self.buffer[row * self.cols..(row + 1) * self.cols]
    }
}

impl<'a, T> Clone for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn clone(&self) -> Self {
        let new_buf = Self::alloc(self.rows(), self.cols());
        for idx in 0..self.buffer.len() {
            new_buf[idx] = self.buffer[idx].clone();
        }
        Matrix {
            cols: self.cols,
            buffer: new_buf,
        }
    }
}

impl<'a, T> fmt::Debug for Matrix<'a, T>
where
    T: Default + Clone + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.rows() {
            if i > 0 {
                write!(f, " ")?;
            }
            write!(f, "{{")?;
            for j in 0..self.cols() {
                write!(f, "{}", self[i][j])?;
                if j + 1 < self.cols() {
                    write!(f, ",")?;
                }
            }
            write!(f, "}}")?;
            if i + 1 < self.rows() {
                writeln!(f)?;
            }
        }
        write!(f, "}}")
    }
}

impl<'a, T> ops::Add for Matrix<'a, T>
where
    T: Default + Clone + ops::Add<Output = T>,
{
    type Output = Self;

    /// Performs addition of two matrices.
    /// Panics, if the sizes of the operands do not match.
    fn add(self, other: Self) -> Self {
        if !self.is_same_size(&other) {
            panic!("operands vary in size");
        }
        let result = Self::new(self.rows(), self.cols());
        for idx in 0..self.elements_number() {
            result.buffer[idx] = self.buffer[idx].clone() + other.buffer[idx].clone();
        }
        result
    }
}

impl<'a, T> ops::Sub for Matrix<'a, T>
where
    T: Default + Clone + ops::Sub<Output = T>,
{
    type Output = Self;

    /// Performs subtraction of two matrices.
    /// Panics, if the sizes of the operands do not match.
    fn sub(self, other: Self) -> Self {
        if !self.is_same_size(&other) {
            panic!("operands vary in size");
        }
        let result = Self::new(self.rows(), self.cols());
        for idx in 0..self.elements_number() {
            result.buffer[idx] = self.buffer[idx].clone() - other.buffer[idx].clone();
        }
        result
    }
}

impl<'a, T> ops::Mul<T> for Matrix<'a, T>
where
    T: Default + Clone + ops::Mul<Output = T>,
{
    type Output = Self;

    /// Performs multiplication the matrix by a number.
    fn mul(self, value: T) -> Self {
        let result = Self::new(self.rows(), self.cols());
        for idx in 0..self.elements_number() {
            result.buffer[idx] = self.buffer[idx].clone() * value.clone();
        }
        result
    }
}

impl<'a, T> ops::Deref for Matrix<'a, T>
where
    T: Default + Clone,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.buffer
    }
}

impl<'a, T> ops::DerefMut for Matrix<'a, T>
where
    T: Default + Clone,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.buffer
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use std::fmt::Debug;

    #[test]
    fn new_ok() {
        let m = Matrix::<i32>::new(100, 100);
        assert_eq_all::<i32>(&m, 0);
    }

    #[test]
    fn fill_ok() {
        let mut m = Matrix::<i32>::new(100, 100);
        m.fill(1);
        assert_eq_all::<i32>(&m, 1);
    }

    #[test]
    fn clear_ok() {
        let mut m = Matrix::<i32>::new(100, 100);
        m.fill(1);
        assert_eq_all::<i32>(&m, 1);
        m.clear();
        assert_eq_all::<i32>(&m, 0);
    }

    #[test]
    fn get_set_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.set(1, 1, 777);
        assert_eq!(m.get(1, 1), &777);
    }

    #[test]
    fn index_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.set(1, 1, 777);
        assert_eq!(m[0][2], 0);
        assert_eq!(m[1][1], 777);
    }

    #[test]
    fn index_mut_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m[1][1] = 777;
        assert_eq!(m.get(1, 1), &777);
        m[0][0] = m[1][1] - 111;
        assert_eq!(m.get(0, 0), &666);
    }

    #[test]
    fn clone_ok() {
        // numbers
        let mut a = Matrix::<i32>::new(2, 3);
        a.fill(100);
        let b = a.clone();
        a.fill(200);
        assert_eq_all(&b, 100);
        assert_eq_all(&a, 200);

        // Strings
        let mut s1 = Matrix::<String>::new(2, 3);
        s1.fill(String::from("first"));
        let s2 = s1.clone();
        s1.fill(String::from("second"));
        assert_eq_all(&s2, String::from("first"));
        assert_eq_all(&s1, String::from("second"));
    }

    #[test]
    fn debug_ok() {
        let mut a = Matrix::<i32>::new(3, 3);
        a.fill(2);
        println!("{:?}", a);
    }

    #[test]
    fn iter_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.fill(7);
        let mut count = 0;
        for e in m.iter() {
            assert_eq!(e, &7);
            count += 1;
        }
        assert_eq!(count, m.elements_number());
    }

    #[test]
    fn iter_mut_ok() {
        let mut m1 = Matrix::<i32>::new(2, 3);
        for e in m1.iter_mut() {
            *e = 7;
        }
        let mut m2 = Matrix::<i32>::new(2, 3);
        m2.fill(7);
        assert_eq!(m1, m2);
    }

    #[test]
    fn add_ok() {
        let mut a = Matrix::<i32>::new(2, 3);
        a.fill(7);
        let mut b = Matrix::<i32>::new(2, 3);
        b.fill(5);
        let c = a + b;
        assert_eq_all(&c, 12);
    }

    #[test]
    fn sub_ok() {
        let mut a = Matrix::<i32>::new(2, 3);
        a.fill(7);
        let mut b = Matrix::<i32>::new(2, 3);
        b.fill(5);
        let c = a - b;
        assert_eq_all(&c, 2);
    }

    #[test]
    fn mul_ok() {
        let mut a = Matrix::<i32>::new(2, 3);
        a.fill(7);
        let b = a * 10;
        assert_eq_all(&b, 70);
    }

    #[test]
    fn deref_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m[0][0] = 7;
        m[0][1] = 12;
        m[0][2] = 17;
        m[1][0] = 25;
        m[1][1] = 31;
        m[1][2] = 100;
        assert_eq!(m.binary_search(&31), Ok(4));
    }

    #[test]
    fn deref_mut_ok() {
        let mut m = Matrix::<i32>::new(2, 3);
        m.fill(7);
        if let Some(first) = m.first_mut() {
            *first = 70;
        }
        assert_eq!(m[0][0], 70);
    }

    fn assert_eq_all<T: Default + Clone + PartialEq + Debug>(m: &Matrix<T>, value: T) {
        for i in 0..m.rows() {
            for j in 0..m.cols() {
                assert_eq!(m.get(i, j), &value);
            }
        }
    }
}
