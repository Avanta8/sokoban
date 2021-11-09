use crate::puzzle::{Puzzle, PuzzleCollection};

pub fn solve_collection(puzzles: &mut PuzzleCollection) {
    for puzzle in puzzles.iter_mut() {
        solve_puzzle(puzzle)
    }
}

pub fn solve_puzzle(puzzle: &mut Puzzle) {
    todo!()
}
