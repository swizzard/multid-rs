#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ix2 {
    pub(crate) row_ix: usize,
    pub(crate) col_ix: usize,
}

impl Eq for Ix2 {}
