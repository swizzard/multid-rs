use crate::errors::VError;
use crate::ix::Ix2;

pub struct V2<T> {
    num_rows: usize,
    num_cols: usize,
    data: Vec<T>,
}

impl<T> V2<T> {
    pub fn new(data: Vec<T>, num_rows: usize, num_cols: usize) -> Result<Self, VError> {
        if num_rows * num_cols != data.len() {
            Err(VError::SizingError {
                expected: data.len(),
                actual: num_rows * num_cols,
            })
        } else {
            Ok(Self {
                num_rows,
                num_cols,
                data,
            })
        }
    }
    pub fn get(&self, Ix2 { row_ix, col_ix }: Ix2) -> Option<&T> {
        if row_ix > self.num_rows || col_ix > self.num_cols {
            None
        } else {
            Some(&self.data[self.convert_ix(col_ix, row_ix)])
        }
    }
    pub fn indices(&self) -> V2Indices {
        V2Indices::new(self.num_rows, self.num_cols)
    }
    pub fn rows(&self) -> V2Rows<'_, T> {
        V2Rows::new(&self.data, self.num_rows, self.num_cols)
    }
    fn convert_ix(&self, col_ix: usize, row_ix: usize) -> usize {
        row_ix * self.num_cols + col_ix
    }
}

impl<T> std::fmt::Debug for V2<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("V2")
            .field("num_rows", &self.num_rows)
            .field("num_cols", &self.num_cols)
            .field("data", &self.data)
            .finish()
    }
}
impl<T> Clone for V2<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            num_rows: self.num_rows,
            num_cols: self.num_cols,
            data: self.data.clone(),
        }
    }
}
impl<T> std::fmt::Display for V2<T>
where
    T: std::fmt::Display,
{
    // prettyPrint
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

pub struct V2Indices {
    curr_row: usize,
    curr_col: usize,
    end_row: usize,
    end_col: usize,
}

impl V2Indices {
    fn new(end_row: usize, end_col: usize) -> Self {
        Self {
            end_row,
            end_col,
            curr_row: 0,
            curr_col: 0,
        }
    }
}

impl Iterator for V2Indices {
    type Item = Ix2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row < self.end_row {
            let col_ix = self.curr_col;
            let row_ix = self.curr_row;
            if self.curr_col == self.end_col - 1 {
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
    let ixs = V2Indices::new(3, 3);
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

pub struct V2Rows<'a, T> {
    curr_row: usize,
    max_row: usize,
    num_cols: usize,
    data: &'a [T],
}

impl<'a, T> V2Rows<'a, T> {
    fn new(data: &'a [T], max_row: usize, num_cols: usize) -> Self {
        Self {
            data,
            num_cols,
            max_row,
            curr_row: 0,
        }
    }
}

impl<'a, T> Iterator for V2Rows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_row == self.max_row {
            None
        } else {
            let start = self.num_cols * self.curr_row;
            self.curr_row += 1;
            let end = self.num_cols * self.curr_row;
            Some(&self.data[start..end])
        }
    }
}

#[cfg(test)]
#[test]
fn test_v2_rows() {
    let data: Vec<u8> = (0..9).collect();
    let rows = V2Rows::new(&data, 3, 3);
    let expected: Vec<&[u8]> = vec![&[0, 1, 2], &[3, 4, 5], &[6, 7, 8]];
    let actual = rows.into_iter().collect::<Vec<&[u8]>>();
    assert_eq!(expected, actual);
}
