// #![allow(unused_imports, dead_code)]
// #![allow(warnings)]

mod directions;
mod expansions;
mod puller;
mod puzzle;
mod solver;
mod squares;

use crate::question::{Question, QuestionCollection};

pub fn solve_collection(questions: &QuestionCollection) {
    for question in questions.iter() {
        solve_puzzle(question);
    }
}

pub fn solve_puzzle(question: &Question) {
    let puzzle = puzzle::Puzzle::from(question);

    println!("\n{}", "-".repeat(50));
    println!("Puzzle:\n{}", puzzle);
    // println!("movable_positions:\n{}", puzzle.view_movable_positions());
    println!("valid positions:\n{}", puzzle.view_valid_positions());

    solver::solve(puzzle);
}
