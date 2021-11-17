// #![allow(unused_imports, dead_code)]
// #![allow(warnings)]

mod board;
mod deadlock;
mod directions;
mod puller;
mod puzzle;
mod solver;
mod squares;

use crate::question::{Question, QuestionCollection};
use solver::Solver;

pub fn solve_collection(questions: &QuestionCollection) {
    for question in questions.iter() {
        solve_puzzle(question);
    }
}

pub fn solve_puzzle(question: &Question) {
    // let (board, start, boxes) = create_board(question);
    // let puzzle = Puzzle::new(&board, start, boxes);

    // println!("\n{}", "-".repeat(50));
    // println!("Puzzle:\n{}", puzzle);
    // println!("moves: {:?}", puzzle.moves());
    // println!("movable_positions:\n{}", puzzle.view_movable_positions());
    // println!("valid positions:\n{}", puzzle.view_valid_positions());

    // solver::solve(puzzle);
    let mut solver = Solver::from(question);
    solver.solve();
}
