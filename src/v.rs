//! 2d vector type, parameterized by number of rows and columns
use crate::errors::VError;
use crate::ix::{BoundedIx2, Ix2CardinalNeighbors, Ix2Neighbors, V2Indices};
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
    /// get a value by 2d index
    pub fn get(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        Some(&self[ix])
    }
    /// get a mutable value by 2d index
    pub fn get_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        Some(&mut self[ix])
    }
    /// an iterator over indices from left to right, top to bottom
    pub fn indices() -> V2Indices<N_ROWS, N_COLS> {
        V2Indices::new()
    }
    /// an iterator over the vector's rows from top to bottom
    pub fn rows(&self) -> V2Rows<'_, T, N_ROWS, N_COLS> {
        V2Rows::new(&self.data)
    }
    /// an iterator over the vector's columns from left to right
    pub fn cols(&self) -> V2Cols<'_, T, N_ROWS, N_COLS> {
        V2Cols::new(&self.data)
    }
    /// an iterator over the neighboring values of a position in the vector
    ///
    /// the iterator starts at the "upper-left" and proceeds left to right, top to bottom
    /// only in-bounds results will be returned
    ///
    /// given a 2d vector like
    ///
    /// `0 1 2`    
    /// `3 4 5`    
    /// `6 7 8`
    ///
    ///
    /// the neighbors of `4` are `[0,1,2,3,5,6,7,8]`, while the neighbors of `2` would be
    /// `[1,4,5]`
    pub fn neighbors_of(
        &self,
        ix: BoundedIx2<N_ROWS, N_COLS>,
    ) -> V2Neighbors<'_, T, N_ROWS, N_COLS> {
        V2Neighbors::new(&self.data, ix)
    }
    /// an iterator over the cardinal neighbors of a position in the vector
    ///
    /// like [`V2::neighbors_of`], excluding "diagonals"
    ///
    /// given a 2d vector like
    ///
    ///
    /// `0 1 2`    
    /// `3 4 5`    
    /// `6 7 8`
    ///
    ///
    /// the cardinal neighbors of `4` are `[1,3,5,8]`, while the neighbors of `2` would be
    /// `[1,5]`
    pub fn cardinal_neighbors_of(
        &self,
        ix: BoundedIx2<N_ROWS, N_COLS>,
    ) -> V2CardinalNeighbors<'_, T, N_ROWS, N_COLS> {
        V2CardinalNeighbors::new(&self.data, ix)
    }
    /// an iterator over tuples of corresponding indices and values, left to right, top to bottom
    pub fn indexed(&self) -> V2Indexed<'_, T, N_ROWS, N_COLS> {
        V2Indexed::new(&self.data)
    }
    /// alter a value in-place
    pub fn mutate_at<F: Fn(&mut T)>(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>, f: F) {
        let i = ix.as_usize();
        if let Some(v) = self.data.get_mut(i) {
            f(v);
        }
    }
    /// possibly get a reference to the value "north" (same column, previous row)
    pub fn north_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::north_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "south" (same column, following row)
    pub fn south_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::south_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "east" (same row, following column)
    pub fn east_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::east_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "west" (same row, previous column)
    pub fn west_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::west_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "northeast" (previous column, previous row)
    pub fn northeast_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::northeast_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "northwest" (previous column, previous row)
    pub fn northwest_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::northwest_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "southeast" (following column, following row)
    pub fn southeast_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::southeast_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "southwest" (following column, previous row)
    pub fn southwest_of(&self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&T> {
        V2::<T, N_ROWS, N_COLS>::southwest_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a mutable reference to the value "north" (same column, previous row)
    pub fn north_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::north_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "south" (same column, following row)
    pub fn south_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::south_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "east" (same row, following column)
    pub fn east_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::east_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "west" (same row, previous column)
    pub fn west_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::west_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "northeast" (previous column, previous row)
    pub fn northeast_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::northeast_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "northwest" (previous column, previous row)
    pub fn northwest_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::northwest_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "southeast" (following column, following row)
    pub fn southeast_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::southeast_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "southwest" (following column, previous row)
    pub fn southwest_of_mut(&mut self, ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<&mut T> {
        V2::<T, N_ROWS, N_COLS>::southwest_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get the index "north" (same column, previous row)
    pub fn north_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.north()
    }
    /// possibly get the index "south" (same column, following row)
    pub fn south_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.south()
    }
    /// possibly get the index "east" (same row, following column)
    pub fn east_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.east()
    }
    /// possibly get the index "west" (same row, previous column)
    pub fn west_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.west()
    }
    /// possibly get the index "northeast" (following column, previous row)
    pub fn northeast_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.northeast()
    }
    /// possibly get the index "northwest" (previous column, previous row)
    pub fn northwest_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.northwest()
    }
    /// possibly get the index "southeast" (following column, following row)
    pub fn southeast_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.southeast()
    }
    /// possibly get the index "southwest" (previous column, following row)
    pub fn southwest_ix(ix: BoundedIx2<N_ROWS, N_COLS>) -> Option<BoundedIx2<N_ROWS, N_COLS>> {
        ix.southwest()
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

/// iterator over vector rows, top to bottom
pub struct V2Rows<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    curr_row: usize,
    data: &'a [T],
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Rows<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a [T]) -> Self {
        Self { data, curr_row: 0 }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator for V2Rows<'a, T, N_ROWS, N_COLS> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row == N_ROWS {
            None
        } else {
            let start = N_COLS * self.curr_row;
            self.curr_row += 1;
            let end = N_COLS * self.curr_row;
            Some(&self.data[start..end])
        }
    }
}

/// iterator over vector columns, left to right
pub struct V2Cols<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    curr_col: usize,
    data: &'a [T],
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Cols<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a [T]) -> Self {
        Self { data, curr_col: 0 }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator for V2Cols<'a, T, N_ROWS, N_COLS> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_col == N_COLS {
            None
        } else {
            let mut v = Vec::with_capacity(N_ROWS);
            for row_ix in 0..N_ROWS {
                let ix = row_ix * N_COLS + self.curr_col;
                v.push(&self.data[ix]);
            }
            self.curr_col += 1;
            Some(v)
        }
    }
}

/// iterator over neighbors
pub struct V2Neighbors<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    data: &'a [T],
    it: Ix2Neighbors<N_ROWS, N_COLS>,
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Neighbors<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a [T], start: BoundedIx2<N_ROWS, N_COLS>) -> Self {
        Self {
            data,
            it: Ix2Neighbors::new(start),
        }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator
    for V2Neighbors<'a, T, N_ROWS, N_COLS>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|i| &self.data[i.as_usize()])
    }
}

/// iterator over cardinal neighbors
pub struct V2CardinalNeighbors<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    data: &'a [T],
    it: Ix2CardinalNeighbors<N_ROWS, N_COLS>,
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2CardinalNeighbors<'a, T, N_ROWS, N_COLS> {
    pub fn new(data: &'a [T], start: BoundedIx2<N_ROWS, N_COLS>) -> Self {
        Self {
            data,
            it: Ix2CardinalNeighbors::new(start),
        }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator
    for V2CardinalNeighbors<'a, T, N_ROWS, N_COLS>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().map(|i| &self.data[i.as_usize()])
    }
}

/// iterator over `(index, reference to value)` tuples
pub struct V2Indexed<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    indices: V2Indices<N_ROWS, N_COLS>,
    data: &'a [T],
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Indexed<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a [T]) -> Self {
        Self {
            indices: V2Indices::new(),
            data,
        }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator
    for V2Indexed<'a, T, N_ROWS, N_COLS>
{
    type Item = (BoundedIx2<N_ROWS, N_COLS>, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        self.indices.next().map(|i| (i, &self.data[i.as_usize()]))
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
    #[test]
    fn test_rows() {
        let data: Vec<u8> = (0..9).collect();
        let rows: V2Rows<u8, 3, 3> = V2Rows::new(&data);
        let expected: Vec<&[u8]> = vec![&[0, 1, 2], &[3, 4, 5], &[6, 7, 8]];
        let actual = rows.into_iter().collect::<Vec<&[u8]>>();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_cols() {
        let data: Vec<u8> = (0..9).collect();
        let cols: V2Cols<u8, 3, 3> = V2Cols::new(&data);
        let expected: Vec<Vec<&u8>> = vec![vec![&0, &3, &6], vec![&1, &4, &7], vec![&2, &5, &8]];
        let actual = cols.into_iter().collect::<Vec<Vec<&u8>>>();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_neighbors() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();

        let expected_upper_left: Vec<&u8> = vec![&1, &3, &4];
        let actual_upper_left: Vec<&u8> = v2
            .neighbors_of(BoundedIx2::<3, 3>::new(0, 0).unwrap())
            .collect();
        assert_eq!(expected_upper_left, actual_upper_left, "upper_left");

        let expected_center: Vec<&u8> = vec![&0, &1, &2, &3, &5, &6, &7, &8];
        let actual_center: Vec<&u8> = v2
            .neighbors_of(BoundedIx2::<3, 3>::new(1, 1).unwrap())
            .collect();
        assert_eq!(expected_center, actual_center, "center");

        let expected_bottom_middle: Vec<&u8> = vec![&3, &4, &5, &6, &8];
        let actual_bottom_middle: Vec<&u8> = v2
            .neighbors_of(BoundedIx2::<3, 3>::new(2, 1).unwrap())
            .collect();
        assert_eq!(
            expected_bottom_middle, actual_bottom_middle,
            "bottom middle"
        );
    }
    #[test]
    fn test_mutate_at() {
        let mut v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
        let expected = vec![0, 1, 9, 3, 4, 5, 6, 7, 8];
        v.mutate_at(BoundedIx2::<3, 3>::new(0, 2).unwrap(), |v: &mut u8| *v = 9);
        assert_eq!(expected, v.data);
    }
    #[test]
    fn test_indexed() {
        let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
        let expected: Vec<(BoundedIx2<3, 3>, &u8)> = vec![
            (BoundedIx2::<3, 3>::new(0, 0).unwrap(), &0),
            (BoundedIx2::<3, 3>::new(0, 1).unwrap(), &1),
            (BoundedIx2::<3, 3>::new(0, 2).unwrap(), &2),
            (BoundedIx2::<3, 3>::new(1, 0).unwrap(), &3),
            (BoundedIx2::<3, 3>::new(1, 1).unwrap(), &4),
            (BoundedIx2::<3, 3>::new(1, 2).unwrap(), &5),
            (BoundedIx2::<3, 3>::new(2, 0).unwrap(), &6),
            (BoundedIx2::<3, 3>::new(2, 1).unwrap(), &7),
            (BoundedIx2::<3, 3>::new(2, 2).unwrap(), &8),
        ];
        let actual = v.indexed().collect::<Vec<(BoundedIx2<3, 3>, &u8)>>();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_get() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert_eq!(v2.get(BoundedIx2::<3, 3>::new(2, 2).unwrap()), Some(&8));
    }
}
