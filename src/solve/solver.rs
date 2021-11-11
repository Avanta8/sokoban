#![allow(dead_code, unused_imports)]

use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use super::puzzle::{Flags, Puzzle};

/*
If we push a box to a place where can no longer be pushed from any direction,
the we know this is the wrong solution.
If a box is blocked by another box, recursively check whether each connecting
box can be pushed or not.
A box doesn't necesarily have to be unpushable to block. It we cannot get to, and push out of the way
a box that is bocking the box in front of us, then (perhaps if this is the only exit, or if all other
exits are also blocked), then we know this is incorrect already.

If a box is pushed towards a wall, we know we can never get it off the wall, unless the wall goes more out.
*/

pub fn solve(grid: Puzzle) {
    // let mut bag = vec![grid];
    let mut visited = FxHashSet::from_iter([grid.grid().clone()]);
    let mut bag = VecDeque::from([grid]);

    let mut solved_puzzle = None;

    let mut count = -1;

    while let Some(puzzle) = bag.pop_front() {
        // if bag.len() % 10000 == 0 {
        //     println!("bag len: {}", bag.len());
        //     println!("{}", puzzle)
        // }

        count += 1;
        if count % 10_000 == 0 {
            println!("\ncount: {}", count);
            println!(
                "\n-----------------------------------------------------\nLooking at puzzle:\n{}\n",
                puzzle
            );
            println!("{}", puzzle.view_movable_positions());
        }

        // println!();
        // println!();
        // println!("{}", puzzle);
        // println!();
        // println!("{}", puzzle.view_movable_positions());

        if puzzle.is_solved() {
            solved_puzzle = Some(puzzle);
            break;
        }

        for (pos, dirs) in puzzle.find_all_pushes() {
            for (dir, &max_steps) in dirs.iter() {
                // println!();
                // println!("dir: {:?}, max steps: {}", dir, max_steps);
                for steps in 1..=max_steps {
                    let mut new_puzzle = puzzle.clone();
                    new_puzzle.move_box(pos, dir, steps);
                    // println!("{}", new_puzzle);
                    // println!();
                    // println!("{}", new_puzzle.view_movable_positions());
                    // println!("{:?}, {:?}", new_puzzle.targets(), new_puzzle.boxes());

                    if visited.contains(new_puzzle.grid()) {
                        continue;
                    }

                    visited.insert(new_puzzle.grid().clone());
                    bag.push_back(new_puzzle);
                }
            }
        }
    }

    println!("total iterations: {}", count);
    println!("visited: {}", visited.len());
    if let Some(puzzle) = solved_puzzle {
        println!("Solved:\n{}", puzzle);
    } else {
        println!("unsolved....");
    }
}
