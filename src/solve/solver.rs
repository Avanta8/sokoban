// #![allow(dead_code, unused_imports)]

use rustc_hash::FxHashSet;
use std::collections::VecDeque;
use std::rc::Rc;

use super::board::Board;
use super::deadlock::Detector;
use super::puller::Puller;
use super::puzzle::Puzzle;
use super::squares::Flags;

use crate::question;

pub struct Solver {
    // board: Rc<Board>,
    puzzles: VecDeque<Puzzle>,
    detector: Detector,
    visited: FxHashSet<Vec<usize>>,
}

impl Solver {
    pub fn solve(&mut self) {
        let mut solved_puzzle = None;

        let mut count = -1;

        // let ds = 1;
        let ds = 10000;

        while let Some(puzzle) = self.puzzles.pop_front() {
            count += 1;
            if count % ds == 0 {
                println!("\n{} count: {} {}", "-".repeat(30), count, "-".repeat(30));
                println!("Looking at puzzle:\n{}\n", puzzle);
                println!("moves: {:?}", puzzle.moves());
                // println!("movable_positions:\n{}", puzzle.view_movable_positions());
                // println!("valid positions:\n{}", puzzle.view_valid_positions());
            }

            if puzzle.is_solved() {
                solved_puzzle = Some(puzzle);
                break;
            }

            self.expand(puzzle);
        }

        println!("total iterations: {}", count);
        println!("visited: {}", self.visited.len());
        if let Some(puzzle) = solved_puzzle {
            println!("Solved:\n{}", puzzle);
            println!("Moves: {:?}", puzzle.moves());
        } else {
            println!("unsolved....");
        }
    }

    fn expand(&mut self, puzzle: Puzzle) {
        for (box_pos, dirs) in puzzle.find_all_pushes(true) {
            for (dir, &max_steps) in dirs.iter() {
                for steps in 1..=max_steps {
                    let mut new_puzzle = puzzle.clone();

                    let last_moved = new_puzzle.move_box(box_pos, dir, steps);
                    new_puzzle.move_to_top_left();

                    let encoding = new_puzzle.get_encoding();
                    if self.visited.contains(&encoding) {
                        continue;
                    }

                    if self.detector.is_deadlocked(
                        new_puzzle.player_pos,
                        &new_puzzle.boxes,
                        last_moved,
                    ) {
                        break;
                    }
                    self.visited.insert(encoding);
                    self.puzzles.push_back(new_puzzle)
                }
            }
        }
    }
}

impl Solver {
    fn _create(
        mut grid: Vec<Flags>,
        width: usize,
        height: usize,
        boxes: FxHashSet<usize>,
        targets: FxHashSet<usize>,
        start_pos: usize,
    ) -> Self {
        let puller = Puller::new(Board::new(width, height, grid.clone(), targets.clone()));

        for pos in puller.find_all_valid_positions() {
            grid[pos] |= Flags::VALID;
        }

        let board = Board::new(width, height, grid, targets);
        let detector = Detector::new(&board);

        let rc_board = Rc::new(board);

        let mut puzzle = Puzzle::new(Rc::clone(&rc_board), start_pos, boxes);
        puzzle.update_movable_positions();
        puzzle.move_to_top_left();

        Self {
            // board: rc_board,
            puzzles: vec![puzzle].into(),
            detector,
            visited: FxHashSet::default(),
        }
    }
}

impl<'a, Q: std::borrow::Borrow<question::Question>> From<Q> for Solver {
    fn from(question: Q) -> Self {
        let question = question.borrow();
        let (width, height) = (question.width(), question.height());
        let mut grid = Vec::with_capacity(width * height);
        for row in question.rows() {
            for sq in row {
                grid.push(match *sq {
                    question::Square::Wall => Flags::WALL,
                    question::Square::Space => Flags::SPACE,
                })
            }
        }

        let start = question.start().to_usize(width);

        let mapper = |it: &std::collections::HashSet<question::Position>| -> FxHashSet<usize> {
            it.iter()
                .map(|p| p.to_usize(width))
                .collect::<FxHashSet<_>>()
        };

        let boxes = mapper(question.boxes());
        let targets = mapper(question.targets());

        Self::_create(grid, width, height, boxes, targets, start)
    }
}
