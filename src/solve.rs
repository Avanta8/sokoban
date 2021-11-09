// #![allow(unused_imports, dead_code)]

mod puzzle;
mod solver;

use crate::question::{Question, QuestionCollection};
// use solver;

pub fn solve_collection(questions: &QuestionCollection) {
    for question in questions.iter() {
        solve_puzzle(question);
    }
}

pub fn solve_puzzle(question: &Question) {
    let _puzzle = puzzle::Puzzle::from(question);
}
