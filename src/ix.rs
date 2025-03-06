//! # custom index types

/// 2d index
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ix2 {
    /// y-coordinate
    pub row_ix: usize,
    /// x-coordinate
    pub col_ix: usize,
}

impl Ix2 {
    /// increase row by 1, returning `None` if out of bounds
    pub fn inc_row(&self) -> Option<Self> {
        if self.row_ix == usize::MAX {
            None
        } else {
            Some(Self {
                row_ix: self.row_ix + 1,
                col_ix: self.col_ix,
            })
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
        }
    }
}

impl Eq for Ix2 {}
