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
    fn convert_ix(&self, col_ix: usize, row_ix: usize) -> usize {
        row_ix * N_COLS + col_ix
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
