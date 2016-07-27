extern crate rand;
extern crate itertools;

use itertools::Itertools;
use rand::{IsaacRng, Rng, SeedableRng};

pub mod util;
pub mod iter;
pub mod cell;
pub mod board;

use board::Board;
use cell::NormalCell;


pub fn random_board(num_mines: usize, rng_seed: &[u32]) -> Board<NormalCell> {
    let mut board: Board<NormalCell> = Board::new(
        NormalCell{is_mine: false, num_adjacent_mines: 0});
    // Create a vec of all possible positions, shuffle it, and select the first `num_mines`
    // positions to set as mines.
    let mut positions: Vec<usize> = (0..(board.x * board.y)).collect();
    IsaacRng::from_seed(rng_seed).shuffle(positions.as_mut_slice());
    for position in positions.iter().take(num_mines) {
        board.get_absolute(*position).is_mine = true;
    }
    board
}
