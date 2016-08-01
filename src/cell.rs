use std::fmt;

pub trait Cell : Copy + Clone + fmt::Display { }
pub trait GameCell : Cell {
    fn is_mine(&self) -> bool;
    fn set_mine(&mut self, value: bool);
    fn num_adjacent_mines(&self) -> usize;
    fn set_adjacent_mines(&mut self, value: usize);
}

#[derive(Copy, Clone)]
pub struct NormalCell {
    _is_mine: bool,
    _num_adjacent_mines: usize
}
impl NormalCell {
    pub fn new() -> NormalCell {
        NormalCell{ _is_mine: false, _num_adjacent_mines: 0 }
    }
}
impl Cell for NormalCell { }
impl GameCell for NormalCell {
    #[inline]
    fn is_mine(&self) -> bool { self._is_mine }

    #[inline]
    fn set_mine(&mut self, value: bool) { self._is_mine = value }

    #[inline]
    fn num_adjacent_mines(&self) -> usize { self._num_adjacent_mines }

    #[inline]
    fn set_adjacent_mines(&mut self, value: usize) {
        self._num_adjacent_mines = value
    }
}
impl fmt::Display for NormalCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", if self.is_mine() { "t" } else { "f" })
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

