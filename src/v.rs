//! 2d vector type, parameterized by number of rows and columns
use crate::errors::VError;
use crate::ix::Ix2;

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
    fn get_ix(&self, Ix2 { row_ix, col_ix }: Ix2) -> Option<usize> {
        if row_ix >= N_ROWS || col_ix >= N_COLS {
            None
        } else {
            Some(self.convert_ix(col_ix, row_ix))
        }
    }
    /// get a value by 2d index
    pub fn get(&self, ix: Ix2) -> Option<&T> {
        self.get_ix(ix).map(|i| &self.data[i])
    }
    /// get a mutable value by 2d index
    pub fn get_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.get_ix(ix).map(|i| &mut self.data[i])
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
    pub fn neighbors_of(&self, ix: Ix2) -> V2Neighbors<'_, T, N_ROWS, N_COLS> {
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
    pub fn cardinal_neighbors_of(&self, ix: Ix2) -> V2CardinalNeighbors<'_, T, N_ROWS, N_COLS> {
        V2CardinalNeighbors::new(&self.data, ix)
    }
    /// an iterator over tuples of corresponding indices and values, left to right, top to bottom
    pub fn indexed(&self) -> V2Indexed<'_, T, N_ROWS, N_COLS> {
        V2Indexed::new(&self.data)
    }
    /// alter a value in-place
    pub fn mutate_at<F: Fn(&mut T)>(&mut self, Ix2 { row_ix, col_ix }: Ix2, f: F) {
        let i = self.convert_ix(col_ix, row_ix);
        if let Some(v) = self.data.get_mut(i) {
            f(v);
        }
    }
    /// possibly get a reference to the value "north" (same column, previous row)
    pub fn north_of(&self, ix: Ix2) -> Option<&T> {
        self.north_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "south" (same column, following row)
    pub fn south_of(&self, ix: Ix2) -> Option<&T> {
        self.south_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "east" (same row, following column)
    pub fn east_of(&self, ix: Ix2) -> Option<&T> {
        self.east_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "west" (same row, previous column)
    pub fn west_of(&self, ix: Ix2) -> Option<&T> {
        self.west_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "northeast" (previous column, previous row)
    pub fn northeast_of(&self, ix: Ix2) -> Option<&T> {
        self.northeast_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "northwest" (previous column, previous row)
    pub fn northwest_of(&self, ix: Ix2) -> Option<&T> {
        self.northwest_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "southeast" (following column, following row)
    pub fn southeast_of(&self, ix: Ix2) -> Option<&T> {
        self.southeast_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a reference to the value "southwest" (following column, previous row)
    pub fn southwest_of(&self, ix: Ix2) -> Option<&T> {
        self.southwest_ix(ix).and_then(|i| self.get(i))
    }
    /// possibly get a mutable reference to the value "north" (same column, previous row)
    pub fn north_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.north_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "south" (same column, following row)
    pub fn south_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.south_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "east" (same row, following column)
    pub fn east_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.east_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "west" (same row, previous column)
    pub fn west_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.west_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "northeast" (previous column, previous row)
    pub fn northeast_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.northeast_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "northwest" (previous column, previous row)
    pub fn northwest_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.northwest_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "southeast" (following column, following row)
    pub fn southeast_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.southeast_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get a mutable reference to the value "southwest" (following column, previous row)
    pub fn southwest_of_mut(&mut self, ix: Ix2) -> Option<&mut T> {
        self.southwest_ix(ix).and_then(|i| self.get_mut(i))
    }
    /// possibly get the index "north" (same column, previous row)
    pub fn north_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.dec_row()
    }
    /// possibly get the index "south" (same column, following row)
    pub fn south_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.inc_row()
    }
    /// possibly get the index "east" (same row, following column)
    pub fn east_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.inc_col()
    }
    /// possibly get the index "west" (same row, previous column)
    pub fn west_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.dec_col()
    }
    /// possibly get the index "northeast" (following column, previous row)
    pub fn northeast_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.dec_row().and_then(|i| i.inc_col())
    }
    /// possibly get the index "northwest" (previous column, previous row)
    pub fn northwest_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.dec_row().and_then(|i| i.dec_col())
    }
    /// possibly get the index "southeast" (following column, following row)
    pub fn southeast_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.inc_row().and_then(|i| i.inc_col())
    }
    /// possibly get the index "southwest" (previous column, following row)
    pub fn southwest_ix(&self, ix: Ix2) -> Option<Ix2> {
        ix.inc_row().and_then(|i| i.dec_col())
    }
    fn convert_ix(&self, col_ix: usize, row_ix: usize) -> usize {
        row_ix * N_COLS + col_ix
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

/// iterator over vector indices
pub struct V2Indices<const N_ROWS: usize, const N_COLS: usize> {
    curr_row: usize,
    curr_col: usize,
}

impl<const N_ROWS: usize, const N_COLS: usize> V2Indices<N_ROWS, N_COLS> {
    fn new() -> Self {
        Self {
            curr_row: 0,
            curr_col: 0,
        }
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Iterator for V2Indices<N_ROWS, N_COLS> {
    type Item = Ix2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row < N_ROWS {
            let col_ix = self.curr_col;
            let row_ix = self.curr_row;
            if self.curr_col == N_COLS - 1 {
                self.curr_col = 0;
                self.curr_row += 1;
            } else {
                self.curr_col += 1;
            }
            Some(Ix2 { row_ix, col_ix })
        } else {
            None
        }
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
    center_col_ix: usize,
    center_row_ix: usize,
    curr_col_ix: usize,
    curr_row_ix: usize,
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Neighbors<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a [T], Ix2 { row_ix, col_ix }: Ix2) -> Self {
        Self {
            data,
            center_col_ix: col_ix,
            center_row_ix: row_ix,
            curr_col_ix: 0,
            curr_row_ix: 0,
        }
    }
    fn dec_col(&self) -> Option<usize> {
        if self.center_col_ix == 0 {
            None
        } else {
            Some(self.center_col_ix - 1)
        }
    }
    fn inc_col(&self) -> Option<usize> {
        if self.center_col_ix == N_COLS - 1 {
            None
        } else {
            Some(self.center_col_ix + 1)
        }
    }
    fn dec_row(&self) -> Option<usize> {
        if self.center_row_ix == 0 {
            None
        } else {
            Some(self.center_row_ix - 1)
        }
    }
    fn inc_row(&self) -> Option<usize> {
        if self.center_row_ix == N_ROWS - 1 {
            None
        } else {
            Some(self.center_row_ix + 1)
        }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator
    for V2Neighbors<'a, T, N_ROWS, N_COLS>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr_row_ix < 3 {
            if self.curr_col_ix == 1 && self.curr_row_ix == 1 {
                self.curr_col_ix = 2;
                continue;
            }
            let col_ix = match self.curr_col_ix {
                0 => self.dec_col(),
                1 => Some(self.center_col_ix),
                2 => self.inc_col(),
                _ => panic!("unreachable col"),
            };
            let row_ix = match self.curr_row_ix {
                0 => self.dec_row(),
                1 => Some(self.center_row_ix),
                2 => self.inc_row(),
                _ => panic!("unreachable row"),
            };
            if col_ix.is_none() {
                self.curr_col_ix = if self.curr_col_ix == 2 {
                    0
                } else {
                    self.curr_col_ix + 1
                };
                continue;
            };
            if row_ix.is_none() {
                self.curr_row_ix += 1;
                continue;
            }
            if self.curr_col_ix == 2 {
                self.curr_row_ix += 1;
                self.curr_col_ix = 0;
            } else {
                self.curr_col_ix += 1;
            }
            return Some(&self.data[row_ix.unwrap() * N_COLS + col_ix.unwrap()]);
        }
        None
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum NeighborDirection {
    N,
    S,
    E,
    W,
}

impl NeighborDirection {
    fn new() -> Option<Self> {
        Some(Self::N)
    }
    fn next(&self) -> Option<Self> {
        match self {
            Self::N => Some(Self::E),
            Self::E => Some(Self::S),
            Self::S => Some(Self::W),
            Self::W => None,
        }
    }
}

/// iterator over cardinal neighbors
pub struct V2CardinalNeighbors<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    data: &'a [T],
    center_col_ix: usize,
    center_row_ix: usize,
    direction: Option<NeighborDirection>,
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2CardinalNeighbors<'a, T, N_ROWS, N_COLS> {
    pub fn new(data: &'a [T], Ix2 { row_ix, col_ix }: Ix2) -> Self {
        Self {
            data,
            center_col_ix: col_ix,
            center_row_ix: row_ix,
            direction: NeighborDirection::new(),
        }
    }
    fn dec_col(&self) -> Option<usize> {
        if self.center_col_ix == 0 {
            None
        } else {
            Some(self.center_col_ix - 1)
        }
    }
    fn inc_col(&self) -> Option<usize> {
        if self.center_col_ix == N_COLS - 1 {
            None
        } else {
            Some(self.center_col_ix + 1)
        }
    }
    fn dec_row(&self) -> Option<usize> {
        if self.center_row_ix == 0 {
            None
        } else {
            Some(self.center_row_ix - 1)
        }
    }
    fn inc_row(&self) -> Option<usize> {
        if self.center_row_ix == N_ROWS - 1 {
            None
        } else {
            Some(self.center_row_ix + 1)
        }
    }
    fn get_north(&self) -> Option<usize> {
        self.dec_row().map(|nr| nr * N_COLS + self.center_col_ix)
    }
    fn get_south(&self) -> Option<usize> {
        self.inc_row().map(|nr| nr * N_COLS * self.center_col_ix)
    }
    fn get_east(&self) -> Option<usize> {
        self.inc_col().map(|nc| self.center_row_ix * N_COLS + nc)
    }
    fn get_west(&self) -> Option<usize> {
        self.dec_col().map(|nc| self.center_row_ix * N_COLS + nc)
    }
    fn get_dir(&self, direction: NeighborDirection) -> Option<usize> {
        match direction {
            NeighborDirection::N => self.get_north(),
            NeighborDirection::S => self.get_south(),
            NeighborDirection::E => self.get_east(),
            NeighborDirection::W => self.get_west(),
        }
    }
    fn next_direction(&mut self) {
        self.direction = self.direction.and_then(|d: NeighborDirection| d.next());
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator
    for V2CardinalNeighbors<'a, T, N_ROWS, N_COLS>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dir) = self.direction {
            self.next_direction();
            if let Some(d) = self.get_dir(dir) {
                return Some(&self.data[d]);
            }
        }
        None
    }
}

/// iterator over `(index, reference to value)` tuples
pub struct V2Indexed<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    indices: V2Indices<N_ROWS, N_COLS>,
    i: usize,
    data: &'a [T],
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Indexed<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a [T]) -> Self {
        Self {
            indices: V2Indices::new(),
            i: 0,
            data,
        }
    }
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> Iterator
    for V2Indexed<'a, T, N_ROWS, N_COLS>
{
    type Item = (Ix2, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ix) = self.indices.next() {
            let old_ix = self.i;
            self.i += 1;
            Some((ix, &self.data[old_ix]))
        } else {
            None
        }
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
    fn test_indices() {
        let ixs: V2Indices<3, 3> = V2Indices::new();
        let expected = vec![
            Ix2 {
                row_ix: 0,
                col_ix: 0,
            },
            Ix2 {
                row_ix: 0,
                col_ix: 1,
            },
            Ix2 {
                row_ix: 0,
                col_ix: 2,
            },
            Ix2 {
                row_ix: 1,
                col_ix: 0,
            },
            Ix2 {
                row_ix: 1,
                col_ix: 1,
            },
            Ix2 {
                row_ix: 1,
                col_ix: 2,
            },
            Ix2 {
                row_ix: 2,
                col_ix: 0,
            },
            Ix2 {
                row_ix: 2,
                col_ix: 1,
            },
            Ix2 {
                row_ix: 2,
                col_ix: 2,
            },
        ];
        let actual = ixs.into_iter().collect::<Vec<Ix2>>();
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
            .neighbors_of(Ix2 {
                row_ix: 0,
                col_ix: 0,
            })
            .collect();
        assert_eq!(expected_upper_left, actual_upper_left, "upper_left");

        let expected_center: Vec<&u8> = vec![&0, &1, &2, &3, &5, &6, &7, &8];
        let actual_center: Vec<&u8> = v2
            .neighbors_of(Ix2 {
                row_ix: 1,
                col_ix: 1,
            })
            .collect();
        assert_eq!(expected_center, actual_center, "center");

        let expected_bottom_middle: Vec<&u8> = vec![&3, &4, &5, &6, &8];
        let actual_bottom_middle: Vec<&u8> = v2
            .neighbors_of(Ix2 {
                row_ix: 2,
                col_ix: 1,
            })
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
        v.mutate_at(
            Ix2 {
                row_ix: 0,
                col_ix: 2,
            },
            |v: &mut u8| *v = 9,
        );
        assert_eq!(expected, v.data);
    }
    #[test]
    fn test_indexed() {
        let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
        let expected = vec![
            (
                Ix2 {
                    row_ix: 0,
                    col_ix: 0,
                },
                &0,
            ),
            (
                Ix2 {
                    row_ix: 0,
                    col_ix: 1,
                },
                &1,
            ),
            (
                Ix2 {
                    row_ix: 0,
                    col_ix: 2,
                },
                &2,
            ),
            (
                Ix2 {
                    row_ix: 1,
                    col_ix: 0,
                },
                &3,
            ),
            (
                Ix2 {
                    row_ix: 1,
                    col_ix: 1,
                },
                &4,
            ),
            (
                Ix2 {
                    row_ix: 1,
                    col_ix: 2,
                },
                &5,
            ),
            (
                Ix2 {
                    row_ix: 2,
                    col_ix: 0,
                },
                &6,
            ),
            (
                Ix2 {
                    row_ix: 2,
                    col_ix: 1,
                },
                &7,
            ),
            (
                Ix2 {
                    row_ix: 2,
                    col_ix: 2,
                },
                &8,
            ),
        ];
        let actual = v.indexed().collect::<Vec<(Ix2, &u8)>>();
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_get() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert!(
            v2.get(Ix2 {
                row_ix: 2,
                col_ix: 3
            })
            .is_none()
        );
        assert!(
            v2.get(Ix2 {
                row_ix: 3,
                col_ix: 2
            })
            .is_none()
        );
        assert_eq!(
            v2.get(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(&8)
        );
    }
    #[test]
    fn test_north() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert!(
            v2.north_ix(Ix2 {
                row_ix: 0,
                col_ix: 0
            })
            .is_none()
        );
        assert_eq!(
            v2.north_ix(Ix2 {
                row_ix: 1,
                col_ix: 0
            }),
            Some(Ix2 {
                row_ix: 0,
                col_ix: 0
            })
        );
    }
    #[test]
    fn test_south() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert_eq!(
            v2.south_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 3,
                col_ix: 2
            })
        );
    }
    #[test]
    fn test_east() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert_eq!(
            v2.east_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 2,
                col_ix: 3
            })
        );
    }
    #[test]
    fn test_west() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert!(
            v2.west_ix(Ix2 {
                row_ix: 0,
                col_ix: 0
            })
            .is_none()
        );
        assert_eq!(
            v2.west_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 2,
                col_ix: 1
            })
        );
    }
    #[test]
    fn test_northwest() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert!(
            v2.northwest_of(Ix2 {
                row_ix: 0,
                col_ix: 1
            })
            .is_none()
        );
        assert_eq!(
            v2.northwest_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 1,
                col_ix: 1
            })
        );
    }
    #[test]
    fn test_northeast() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert!(
            v2.northeast_ix(Ix2 {
                row_ix: 0,
                col_ix: 1
            })
            .is_none()
        );
        assert_eq!(
            v2.northeast_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 1,
                col_ix: 3
            })
        );
    }
    #[test]
    fn test_southwest() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert_eq!(
            v2.southwest_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 3,
                col_ix: 1
            })
        );
    }
    #[test]
    fn test_southeast() {
        let v2: V2<u8, 3, 3> = V2::new((0..=8).collect::<Vec<u8>>()).unwrap();
        assert_eq!(
            v2.southeast_ix(Ix2 {
                row_ix: 2,
                col_ix: 2
            }),
            Some(Ix2 {
                row_ix: 3,
                col_ix: 3
            })
        );
    }
}
