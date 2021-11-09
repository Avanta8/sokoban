// #![allow(unused_imports, dead_code)]

mod puzzle;
mod solver;

use crate::question::{Question, QuestionCollection};

pub fn solve_collection(questions: &mut QuestionCollection) {
    for question in questions.iter_mut() {
        solve_puzzle(question);
    }
}

pub fn solve_puzzle(_question: &mut Question) {
    todo!()
}
