use std::fmt;

use itertools::Itertools;

use cell::{Cell, GameCell};
use iter::AbsoluteAdjacencyIterator;
use util::abs_difference;


const X: usize = 20;
const Y: usize = 40;

pub struct Board<T: Cell> {
    pub x: usize,
    pub y: usize,
    board: [[T; X]; Y]
}

impl<T: Cell> Board<T> {
    pub fn new(t: T) -> Board<T> {
        Board::<T>{x: X, y: Y, board: [[t; X]; Y]}
    }

    #[inline]
    pub fn get(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.board[row][col]
    }

    #[inline]
    pub fn get_absolute(&mut self, position: usize) -> &mut T {
        let row = position / X;
        let col = position % X;
        self.get(row, col)
    }

    pub fn is_adjacent(&self, position: usize, other_position: usize) -> bool {
        let max_pos = self.x * self.y - 1;
        if position > max_pos || other_position > max_pos
            || position == other_position
            || abs_difference(position / self.x, other_position / self.x) > 1
            || abs_difference(position % self.x, other_position % self.x) > 1 {
            false
        } else {
            true
        }
    }

    pub fn adjacent_positions(&self, position: usize) -> AbsoluteAdjacencyIterator {
        AbsoluteAdjacencyIterator::new(position, self.x, self.y)
    }
}

impl<T: GameCell> Board<T> {
    pub fn set_adjacent_mines(&mut self) {
        let max_pos = self.x * self.y - 1;
        for position in 0..max_pos {
            let count = self.adjacent_positions(position).map(|adj_pos| {
                if self.get_absolute(adj_pos).is_mine() { 1 } else { 0 }
            }).fold(0, |a, b| a + b);
            self.get_absolute(position).set_adjacent_mines(count);
        }
    }
}

impl<T: Cell> fmt::Display for Board<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let board_str = self.board.iter().enumerate().map(|(i, &row)| {
            format!("{} {}", i, row.iter().join(" "))
        }).join("\n");

        write!(f, "{}", board_str)
    }
}


