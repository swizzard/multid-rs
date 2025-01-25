use crate::errors::VError;
use crate::ix::Ix2;

pub struct V2<T, const N_ROWS: usize, const N_COLS: usize> {
    data: Vec<T>,
}

impl<T, const N_ROWS: usize, const N_COLS: usize> V2<T, N_ROWS, N_COLS> {
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
    pub fn get(&self, Ix2 { row_ix, col_ix }: Ix2) -> Option<&T> {
        if row_ix > N_ROWS || col_ix > N_COLS {
            None
        } else {
            Some(&self.data[self.convert_ix(col_ix, row_ix)])
        }
    }
    pub fn indices(&self) -> V2Indices<N_ROWS, N_COLS> {
        V2Indices::new()
    }
    pub fn rows(&self) -> V2Rows<'_, T, N_ROWS, N_COLS> {
        V2Rows::new(&self.data)
    }
    pub fn cols(&self) -> V2Cols<'_, T, N_ROWS, N_COLS> {
        V2Cols::new(&self.data)
    }
    pub fn neighbors_of(&self, ix: Ix2) -> V2Neighbors<'_, T, N_ROWS, N_COLS> {
        V2Neighbors::new(&self.data, ix)
    }
    fn convert_ix(&self, col_ix: usize, row_ix: usize) -> usize {
        row_ix * N_COLS + col_ix
    }
}
impl<T, const N_ROWS: usize, const N_COLS: usize> V2<T, N_ROWS, N_COLS>
where
    T: Clone,
{
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

#[cfg(test)]
#[test]
fn test_add_col() {
    let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
    let c: Vec<u8> = vec![9, 10, 11];
    let expected = vec![0, 1, 2, 9, 3, 4, 5, 10, 6, 7, 8, 11];
    let actual = v.add_col(c).unwrap();
    assert_eq!(expected, actual.data);
}

#[cfg(test)]
#[test]
fn test_add_row() {
    let v: V2<u8, 3, 3> = V2::new((0..=8).collect()).unwrap();
    let r: Vec<u8> = vec![9, 10, 11];
    let expected = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let actual = v.add_row(r).unwrap();
    assert_eq!(expected, actual.data);
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
    // prettyPrint
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

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

#[cfg(test)]
#[test]
fn test_v2_indices() {
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

#[cfg(test)]
#[test]
fn test_v2_rows() {
    let data: Vec<u8> = (0..9).collect();
    let rows: V2Rows<u8, 3, 3> = V2Rows::new(&data);
    let expected: Vec<&[u8]> = vec![&[0, 1, 2], &[3, 4, 5], &[6, 7, 8]];
    let actual = rows.into_iter().collect::<Vec<&[u8]>>();
    assert_eq!(expected, actual);
}

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
#[cfg(test)]
#[test]
fn test_v2_cols() {
    let data: Vec<u8> = (0..9).collect();
    let cols: V2Cols<u8, 3, 3> = V2Cols::new(&data);
    let expected: Vec<Vec<&u8>> = vec![vec![&0, &3, &6], vec![&1, &4, &7], vec![&2, &5, &8]];
    let actual = cols.into_iter().collect::<Vec<Vec<&u8>>>();
    assert_eq!(expected, actual);
}

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

#[cfg(test)]
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

pub struct V2Indexed<'a, T, const N_ROWS: usize, const N_COLS: usize> {
    indices: V2Indices<N_ROWS, N_COLS>,
    i: usize,
    data: &'a T,
}

impl<'a, T, const N_ROWS: usize, const N_COLS: usize> V2Indexed<'a, T, N_ROWS, N_COLS> {
    fn new(data: &'a T) -> Self {
        Self {
            indices: V2Indices::new(),
            i: 0,
            data,
        }
    }
}

