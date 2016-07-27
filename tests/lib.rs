#![cfg(test)]

extern crate minesweeper_solver;

use minesweeper_solver::util::{abs_difference, is_adjacent};
use minesweeper_solver::board::Board;
use minesweeper_solver::cell::NormalCell;
use minesweeper_solver::random_board;


#[test]
fn test_abs_difference() {
    assert_eq!(1, abs_difference(0, 1));
    assert_eq!(1, abs_difference(1, 0));

    assert_eq!(0, abs_difference(0, 0));
}

#[test]
fn test_is_adjacent() {
    let x = 40;
    let y = 20;
    let test = |pos, other| {
        is_adjacent(pos, other, x, y)
    };

    // Corner
    assert!(test(0, 1));
    assert!(test(0, x));
    assert!(test(0, x + 1));

    // Edge
    assert!(test(x, 2 * x));
    assert!(test(x, 2 * x + 1));
    assert!(test(x, x + 1));
    assert!(test(x, 1));
    assert!(test(x, 0));

    // Middle
    assert!(test(x + 10, 10));
    assert!(test(x + 10, 11));
    assert!(test(x + 10, x + 11));
    assert!(test(x + 10, 2 * x + 11));
    assert!(test(x + 10, 2 * x + 10));
    assert!(test(x + 10, 2 * x + 9));
    assert!(test(x + 10, x + 9));
    assert!(test(x + 10, 9));

    // Reflexive
    assert!(test(1, 0) && test(0, 1));
    assert!(test(10, x + 10) && test(x + 10, 10));

    // Far Corner
    assert!(test(x * y - 1, x * (y - 1) - 1));

    // Not self-adjacent
    assert!(!test(1, 1));

    // Not adjacent across row boundaries
    assert!(!test(x, x - 1));
}

#[test]
fn test_main() {
    let x: Board<NormalCell> = random_board(40, &[1,2,3,4,5]);
    // TODO: Test board output.
    println!("{}x{}", x.y, x.x);
    println!("{}", x);
    for i in x.adjacent_positions(20) {
        println!("{}", i);
    }
}
