//! 2d vector type, parameterized by number of rows and columns
use crate::errors::VError;
use crate::ix::BoundedIx2;
use std::ops::{Index, IndexMut};

/// 2d vector type, parameterized by number of rows and columns
pub struct V2<T, const N_ROWS: usize, const N_COLS: usize> {
    data: Vec<T>,
}

impl<T, const N_ROWS: usize, const N_COLS: usize> V2<T, N_ROWS, N_COLS> {
    /// create a new 2d vector from a preexisting 1d vector
    ///
    /// errors if the provided data is the wrong length
    pub fn new(data: Vec<T>) -> Result<Self, VError> {
        if N_ROWS * N_COLS != data.len() {
            Err(VError::SizingError {
                expected: data.len(),
                actual: N_ROWS * N_COLS,
            })
        } else {
            Ok(Self { data })
        }
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> PartialEq for V2<T, N_ROWS, N_COLS>
where
    T: PartialEq,
{
    fn eq(&self, other: &V2<T, N_ROWS, N_COLS>) -> bool {
        self.data == other.data
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> Index<BoundedIx2<N_ROWS, N_COLS>>
    for V2<T, N_ROWS, N_COLS>
{
    type Output = T;

    fn index(&self, index: BoundedIx2<N_ROWS, N_COLS>) -> &Self::Output {
        &self.data[index.as_usize()]
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> IndexMut<BoundedIx2<N_ROWS, N_COLS>>
    for V2<T, N_ROWS, N_COLS>
{
    fn index_mut(&mut self, index: BoundedIx2<N_ROWS, N_COLS>) -> &mut Self::Output {
        &mut self.data[index.as_usize()]
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> V2<T, N_ROWS, N_COLS>
where
    T: Clone,
{
    /// create a clone of this vector with an additional column
    ///
    /// errors if the length of the new column doesn't match the number of rows in the vector
    pub fn add_col(self, col: Vec<T>) -> Result<V2<T, N_ROWS, { N_COLS + 1 }>, VError> {
        if col.len() != N_ROWS {
            Err(VError::SizingError {
                expected: N_ROWS,
                actual: col.len(),
            })
        } else {
            let mut new_data = self.data;
            for (row_ix, item) in col.iter().enumerate() {
                new_data.insert(row_ix * N_COLS + (N_COLS + row_ix), item.clone())
            }
            Ok(V2 { data: new_data })
        }
    }
    /// create a clone of this vector with an additional row
    ///
    /// errors if the length of the new row doesn't match the number of columns in the vector
    pub fn add_row(self, row: Vec<T>) -> Result<V2<T, { N_ROWS + 1 }, N_COLS>, VError> {
        if row.len() != N_COLS {
            Err(VError::SizingError {
                expected: N_COLS,
                actual: row.len(),
            })
        } else {
            let mut new_data = self.data;
            new_data.extend(row);
            Ok(V2 { data: new_data })
        }
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> std::fmt::Debug for V2<T, N_ROWS, N_COLS>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "V2<{}, {}> {{ data: {:?} }}", N_ROWS, N_COLS, self.data)
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> Clone for V2<T, N_ROWS, N_COLS>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> Default for V2<T, N_ROWS, N_COLS>
where
    T: Default,
{
    fn default() -> Self {
        let len = N_ROWS * N_COLS;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(T::default())
        }
        Self { data }
    }
}

impl<T, const N_ROWS: usize, const N_COLS: usize> std::fmt::Display for V2<T, N_ROWS, N_COLS>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut d = self.data.iter().enumerate();
        write!(f, "{} ", d.next().unwrap().1)?;
        for (i, v) in d {
            let ni = i + 1;
            if ni == self.data.len() {
                write!(f, "{}", v)?;
            } else if ni % N_COLS == 0 {
                writeln!(f, "{}", v)?;
            } else {
                write!(f, "{} ", v)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_col() {
        let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
        let c: Vec<u8> = vec![9, 10, 11];
        let expected = vec![0, 1, 2, 9, 3, 4, 5, 10, 6, 7, 8, 11];
        let actual = v.add_col(c).unwrap();
        assert_eq!(expected, actual.data);
    }

    #[test]
    fn test_add_row() {
        let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
        let r: Vec<u8> = vec![9, 10, 11];
        let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let actual = v.add_row(r).unwrap();
        assert_eq!(expected, actual.data);
    }
    #[test]
    fn test_display() {
        let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
        let expected = "0 1 2\n3 4 5\n6 7 8";
        let actual = format!("{}", v);
        assert_eq!(expected, actual);
    }
}
