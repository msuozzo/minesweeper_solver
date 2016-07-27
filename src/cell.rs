use std::fmt;

pub trait Cell : Copy + Clone + fmt::Display { }

#[derive(Copy, Clone)]
pub struct NormalCell {
    pub is_mine: bool,
    pub num_adjacent_mines: usize
}
impl Cell for NormalCell { }
impl fmt::Display for NormalCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", if self.is_mine { "t" } else { "f" })
    }
}

#[derive(Copy, Clone)]
pub struct TestCell {
}
impl Cell for TestCell { }
impl fmt::Display for TestCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", "?")
    }
}

