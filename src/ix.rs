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

impl Eq for Ix2 {}
