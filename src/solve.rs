// #![allow(unused_imports, dead_code)]

mod puzzle;
mod solver;

use crate::question::{Question, QuestionCollection};

pub fn solve_collection(questions: &QuestionCollection) {
    for question in questions.iter() {
        solve_puzzle(question);
    }
}

pub fn solve_puzzle(question: &Question) {
    let puzzle = puzzle::Puzzle::from(question);

    println!("Puzzle:\n{}", puzzle);
    println!("{:?}", puzzle.movable_positions());
    println!("{}, {}", puzzle.width(), puzzle.height());
    println!("{}", puzzle.view_movable_positions());

    solver::solve(puzzle);
}
