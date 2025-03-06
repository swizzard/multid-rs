#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ix2 {
    pub(crate) row_ix: usize,
    pub(crate) col_ix: usize,
}

impl Ix2 {
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
