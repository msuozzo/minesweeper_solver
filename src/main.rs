extern crate rand;
extern crate itertools;

use std::fmt;
use itertools::Itertools;
use rand::{IsaacRng, Rng, SeedableRng};

const X: usize = 20;
const Y: usize = 40;

trait Cell : Copy + Clone + fmt::Display { }

#[derive(Copy, Clone)]
struct NormalCell {
    is_mine: bool,
    adjacent_mines: usize
}
impl Cell for NormalCell { }
impl fmt::Display for NormalCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", if self.is_mine { "t" } else { "f" })
    }
}

#[derive(Copy, Clone)]
struct TestCell {
}
impl Cell for TestCell { }
impl fmt::Display for TestCell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}}}", "?")
    }
}

struct Board<T: Cell> {
    x: usize,
    y: usize,
    board: [[T; X]; Y]
}

impl<T: Cell> Board<T> {
    fn new(t: T) -> Board<T> {
        Board::<T>{x: X, y: Y, board: [[t; X]; Y]}
    }

    #[inline]
    fn get(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.board[row][col]
    }

    #[inline]
    fn get_absolute(&mut self, position: usize) -> &mut T {
        let row = position / X;
        let col = position % X;
        self.get(row, col)
    }

    fn is_adjacent(&self, position: usize, other_position: usize) -> bool {
        let max_pos = self.x * self.y - 1;
        if position < 0 || position > max_pos
            || other_position < 0 || other_position > max_pos
            || position == other_position
            || abs_difference(position / self.x, other_position / self.x) > 1
            || abs_difference(position % self.x, other_position % self.x) > 1 {
            false
        } else {
            true
        }
    }

    fn adjacent_positions(&self, position: usize) -> AbsoluteAdjacencyIterator {
        AbsoluteAdjacencyIterator::new(position, self.x, self.y)
    }

    fn set_adjacent_mines(&mut self) {
        let max_pos = self.x * self.y - 1;
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

struct AbsoluteAdjacencyIterator {
    position: usize,
    max_x: usize,
    max_y: usize,
    current: usize,
}

static ADJACENT_INDICES: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];
impl AbsoluteAdjacencyIterator {
    fn new(position: usize, max_x: usize, max_y: usize) -> AbsoluteAdjacencyIterator {
        AbsoluteAdjacencyIterator{
            position: position, max_x: max_x, max_y: max_y, current: 0}
    }
}

#[inline]
fn abs_difference(a: usize, b: usize) -> usize {
    if a < b { b - a } else { a - b }
}

fn is_adjacent(position: usize, other_position: usize, max_x: usize, max_y: usize) -> bool {
    let max_pos = max_x * max_y - 1;
    if position < 0 || position > max_pos
        || other_position < 0 || other_position > max_pos
        || position == other_position
        || abs_difference(position / max_x, other_position / max_x) > 1
        || abs_difference(position % max_x, other_position % max_x) > 1 {
        false
    } else {
        true
    }
}

impl Iterator for AbsoluteAdjacencyIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        loop {
            if self.current >= ADJACENT_INDICES.len() {
                return None;
            }
            let (x_multiplier, constant_value) = ADJACENT_INDICES[self.current];
            self.current += 1;
            let pos =
                self.position as i32 + x_multiplier * self.max_x as i32 + constant_value;
            // Perform bounds-checking.
            if pos >= 0 && is_adjacent(pos as usize, self.position, self.max_x, self.max_y) {
                return Some(pos as usize);
            }
        }
    }
}

fn random_board(num_mines: usize, rng_seed: &[u32]) -> Board<NormalCell> {
    let mut board: Board<NormalCell> = Board::new(
        NormalCell{is_mine: false, adjacent_mines: 0});
    // Create a vec of all possible positions, shuffle it, and select the first `num_mines`
    // positions to set as mines.
    let mut positions: Vec<usize> = (0..(X * Y)).collect();
    IsaacRng::from_seed(rng_seed).shuffle(positions.as_mut_slice());
    for position in positions.iter().take(num_mines) {
        board.get_absolute(*position).is_mine = true;
    }
    board
}

fn main() {
    let x: Board<NormalCell> = random_board(40, &[1,2,3,4,5]);
    println!("{}x{}", x.y, x.x);
    println!("{}", x);
    for i in x.adjacent_positions(20) {
        println!("{}", i);
    }
}
