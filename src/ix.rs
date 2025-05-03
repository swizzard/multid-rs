//! # custom index types

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundedIx2<const N_ROWS: usize, const N_COLS: usize> {
    /// y-coordinate
    row_ix: usize,
    /// x-coordinate
    col_ix: usize,
}

impl<const N_ROWS: usize, const N_COLS: usize> Eq for BoundedIx2<N_ROWS, N_COLS> {}

impl<const N_ROWS: usize, const N_COLS: usize> BoundedIx2<N_ROWS, N_COLS> {
    #[inline]
    const fn in_bounds(BoundedIx2 { col_ix, row_ix }: &BoundedIx2<N_ROWS, N_COLS>) -> bool {
        *col_ix < N_COLS && *row_ix < N_ROWS
    }

    pub fn new(row_ix: usize, col_ix: usize) -> Option<Self> {
        if col_ix < N_COLS && row_ix < N_ROWS {
            Some(Self { col_ix, row_ix })
        } else {
            None
        }
    }

    /// convert to 1d index
    pub fn as_usize(&self) -> usize {
        self.row_ix * N_COLS + self.col_ix
    }
    /// increase row by 1, returning `None` if out of bounds
    pub fn inc_row(&self) -> Option<Self> {
        if self.row_ix == usize::MAX {
            None
        } else {
            Some(Self {
                row_ix: self.row_ix + 1,
                col_ix: self.col_ix,
            })
            .filter(BoundedIx2::<N_ROWS, N_COLS>::in_bounds)
        }
    }
    /// increase col by 1, returning `None` if out of bounds
    pub fn inc_col(&self) -> Option<Self> {
        if self.col_ix == usize::MAX {
            None
        } else {
            Some(Self {
                row_ix: self.row_ix,
                col_ix: self.col_ix + 1,
            })
            .filter(BoundedIx2::<N_ROWS, N_COLS>::in_bounds)
        }
    }
    /// decrease row by 1, returning `None` if out of bounds
    pub fn dec_row(&self) -> Option<Self> {
        if self.row_ix == 0 {
            None
        } else {
            Some(Self {
                row_ix: self.row_ix - 1,
                col_ix: self.col_ix,
            })
            .filter(BoundedIx2::<N_ROWS, N_COLS>::in_bounds)
        }
    }
    /// decrease col by 1, returning `None` if out of bounds
    pub fn dec_col(&self) -> Option<Self> {
        if self.col_ix == 0 {
            None
        } else {
            Some(Self {
                row_ix: self.row_ix,
                col_ix: self.col_ix - 1,
            })
            .filter(BoundedIx2::<N_ROWS, N_COLS>::in_bounds)
        }
    }
    /// decrease row by 1, returning `None` if out of bounds
    pub fn north(&self) -> Option<Self> {
        self.dec_row()
    }
    /// increase row by 1, returning `None` if out of bounds
    pub fn south(&self) -> Option<Self> {
        self.inc_row()
    }
    /// increase col by 1, returning `None` if out of bounds
    pub fn east(&self) -> Option<Self> {
        self.inc_col()
    }
    /// decrease col by 1, returning `None` if out of bounds
    pub fn west(&self) -> Option<Self> {
        self.dec_col()
    }
    /// decrease row by 1 and increase col by 1, returning `None` if out of bounds
    pub fn northeast(&self) -> Option<Self> {
        self.dec_row().and_then(|i| i.inc_col())
    }
    /// decrease row by 1 and decrease col by 1, returning `None` if out of bounds
    pub fn northwest(&self) -> Option<Self> {
        self.dec_row().and_then(|i| i.dec_col())
    }
    /// increase row by 1 and increase col by 1, returning `None` if out of bounds
    pub fn southeast(&self) -> Option<Self> {
        self.inc_row().and_then(|i| i.inc_col())
    }
    /// increase row by 1 and decrease col by 1, returning `None` if out of bounds
    pub fn southwest(&self) -> Option<Self> {
        self.inc_row().and_then(|i| i.dec_col())
    }
}

/// iterator over vector indices
pub struct V2Indices<const N_ROWS: usize, const N_COLS: usize> {
    curr_row: usize,
    curr_col: usize,
}

impl<const N_ROWS: usize, const N_COLS: usize> V2Indices<N_ROWS, N_COLS> {
    pub fn new() -> Self {
        Self {
            curr_row: 0,
            curr_col: 0,
        }
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Iterator for V2Indices<N_ROWS, N_COLS> {
    type Item = BoundedIx2<N_ROWS, N_COLS>;

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
            Some(BoundedIx2 { row_ix, col_ix })
        } else {
            None
        }
    }
}

pub struct Ix2Neighbors<const N_ROWS: usize, const N_COLS: usize> {
    start: BoundedIx2<N_ROWS, N_COLS>,
    curr_ix: u8,
}

impl<const N_ROWS: usize, const N_COLS: usize> Ix2Neighbors<N_ROWS, N_COLS> {
    pub fn new(start: BoundedIx2<N_ROWS, N_COLS>) -> Self {
        Self { start, curr_ix: 0 }
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Iterator for Ix2Neighbors<N_ROWS, N_COLS> {
    type Item = BoundedIx2<N_ROWS, N_COLS>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr_ix < 8 {
            let res = match self.curr_ix {
                0 => self.start.northwest(),
                1 => self.start.north(),
                2 => self.start.northeast(),
                3 => self.start.west(),
                4 => self.start.east(),
                5 => self.start.southwest(),
                6 => self.start.south(),
                7 => self.start.southeast(),
                _ => panic!("invalid"),
            };
            self.curr_ix += 1;
            if res.is_some() {
                return res;
            }
        }
        None
    }
}

pub struct Ix2CardinalNeighbors<const N_ROWS: usize, const N_COLS: usize> {
    start: BoundedIx2<N_ROWS, N_COLS>,
    curr_ix: u8,
}

impl<const N_ROWS: usize, const N_COLS: usize> Ix2CardinalNeighbors<N_ROWS, N_COLS> {
    pub fn new(start: BoundedIx2<N_ROWS, N_COLS>) -> Self {
        Self { start, curr_ix: 0 }
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Iterator for Ix2CardinalNeighbors<N_ROWS, N_COLS> {
    type Item = BoundedIx2<N_ROWS, N_COLS>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.curr_ix < 4 {
            let res = match self.curr_ix {
                0 => self.start.north(),
                1 => self.start.east(),
                2 => self.start.south(),
                3 => self.start.west(),
                _ => panic!("invalid"),
            };
            self.curr_ix += 1;
            if res.is_some() {
                return res;
            }
        }
        None
    }
}

pub struct BoundedIx2Rows<const N_ROWS: usize, const N_COLS: usize> {
    row: std::ops::Range<usize>,
}

impl<const N_ROWS: usize, const N_COLS: usize> BoundedIx2Rows<N_ROWS, N_COLS> {
    pub fn new() -> Self {
        Self { row: 0..N_ROWS }
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Default for BoundedIx2Rows<N_ROWS, N_COLS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N_ROWS: usize, const N_COLS: usize> Iterator for BoundedIx2Rows<N_ROWS, N_COLS> {
    type Item = [BoundedIx2<N_ROWS, N_COLS>; N_COLS];
}

#[cfg(test)]
mod test {
    use super::*;
    type I = BoundedIx2<3, 3>;
    #[test]
    fn test_north() {
        assert!(I::new(0, 0).unwrap().north().is_none());
        assert_eq!(
            I::new(1, 0).unwrap().north().unwrap(),
            BoundedIx2 {
                row_ix: 0,
                col_ix: 0
            }
        )
    }
    #[test]
    fn test_south() {
        assert!(I::new(2, 2).unwrap().south().is_none());
        assert_eq!(
            I::new(1, 2).unwrap().south().unwrap(),
            BoundedIx2 {
                row_ix: 2,
                col_ix: 2
            }
        )
    }
    #[test]
    fn test_east() {
        assert!(I::new(1, 2).unwrap().east().is_none());
        assert_eq!(
            I::new(1, 1).unwrap().east().unwrap(),
            BoundedIx2 {
                row_ix: 1,
                col_ix: 2
            }
        )
    }
    #[test]
    fn test_west() {
        assert!(I::new(1, 0).unwrap().west().is_none());
        assert_eq!(
            I::new(2, 2).unwrap().west().unwrap(),
            BoundedIx2 {
                row_ix: 2,
                col_ix: 1
            }
        );
    }
    #[test]
    fn test_northwest() {
        assert!(I::new(0, 1).unwrap().northwest().is_none());
        assert!(I::new(1, 0).unwrap().northwest().is_none());
        assert_eq!(
            I::new(2, 2).unwrap().northwest().unwrap(),
            BoundedIx2 {
                col_ix: 1,
                row_ix: 1
            }
        );
    }
    #[test]
    fn test_northeast() {
        assert!(I::new(0, 1).unwrap().northeast().is_none());
        assert!(I::new(2, 2).unwrap().northeast().is_none());
        assert_eq!(
            I::new(2, 0).unwrap().northeast().unwrap(),
            BoundedIx2 {
                col_ix: 1,
                row_ix: 1
            }
        );
    }
    #[test]
    fn test_southwest() {
        assert!(I::new(2, 1).unwrap().southwest().is_none());
        assert!(I::new(1, 0).unwrap().southwest().is_none());
        assert_eq!(
            I::new(0, 2).unwrap().southwest().unwrap(),
            BoundedIx2 {
                col_ix: 1,
                row_ix: 1
            }
        )
    }
    #[test]
    fn test_southeast() {
        assert!(I::new(1, 2).unwrap().southeast().is_none());
        assert!(I::new(2, 1).unwrap().southeast().is_none());
        assert_eq!(
            I::new(0, 0).unwrap().southeast().unwrap(),
            BoundedIx2 {
                col_ix: 1,
                row_ix: 1
            }
        );
    }
    #[test]
    fn test_neighbors_center() {
        let start: BoundedIx2<3, 3> = BoundedIx2 {
            row_ix: 1,
            col_ix: 1,
        };
        let expected: Vec<BoundedIx2<3, 3>> = vec![
            BoundedIx2 {
                row_ix: 0,
                col_ix: 0,
            },
            BoundedIx2 {
                row_ix: 0,
                col_ix: 1,
            },
            BoundedIx2 {
                row_ix: 0,
                col_ix: 2,
            },
            BoundedIx2 {
                row_ix: 1,
                col_ix: 0,
            },
            BoundedIx2 {
                row_ix: 1,
                col_ix: 2,
            },
            BoundedIx2 {
                row_ix: 2,
                col_ix: 0,
            },
            BoundedIx2 {
                row_ix: 2,
                col_ix: 1,
            },
            BoundedIx2 {
                row_ix: 2,
                col_ix: 2,
            },
        ];
        let actual: Vec<BoundedIx2<3, 3>> = Ix2Neighbors::new(start).collect();
        assert_eq!(actual, expected)
    }
    #[test]
    fn test_neighbors_side() {
        let start: BoundedIx2<3, 3> = BoundedIx2 {
            row_ix: 1,
            col_ix: 0,
        };
        let expected: Vec<BoundedIx2<3, 3>> = vec![
            BoundedIx2 {
                row_ix: 0,
                col_ix: 0,
            },
            BoundedIx2 {
                row_ix: 0,
                col_ix: 1,
            },
            BoundedIx2 {
                row_ix: 1,
                col_ix: 1,
            },
            BoundedIx2 {
                row_ix: 2,
                col_ix: 0,
            },
            BoundedIx2 {
                row_ix: 2,
                col_ix: 1,
            },
        ];
        let actual: Vec<BoundedIx2<3, 3>> = Ix2Neighbors::new(start).collect();
        assert_eq!(actual, expected)
    }
}
