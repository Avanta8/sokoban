#![allow(dead_code, unused_imports)]

use std::collections::VecDeque;

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

/*
BUG:
If a player in standing in front of a box. Then that box will not be able to be moved that way
because it will seem like the player is blocking it, even though they would have to move to being it
in order to move it.
However, we should be able to just treat the square the player is standing on as also walkable for this
scenario.
*/

pub fn solve(grid: Puzzle) {
    // let mut bag = vec![grid];
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
            // println!("count: {}", count);
            // println!(
            //     "\n-----------------------------------------------------\nLooking at puzzle:\n{}\n",
            //     puzzle
            // );
        }
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
                    bag.push_back(new_puzzle);
                }
            }
        }
    }

    if let Some(puzzle) = solved_puzzle {
        println!("Solved:\n{}", puzzle);
    } else {
        println!("unsolved....");
    }
}
