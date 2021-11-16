#![allow(dead_code, unused_imports)]

use rustc_hash::FxHashSet;
use std::collections::VecDeque;

use crate::solve::expansions::ExpansionsHelper;

use super::puzzle::Puzzle;

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
After making a move, analyse it simply.
Only need to consider the box that was just moved, and the ones now connecting to it.
If the box was move into a corner that doesn't contain a target, then we instantly know that this
is an incorrect solution.

If the box was moved so that it is now adjacent to a wall, if the wall does not go 'outwards' again, and
there are more boxes than target adjacent to that wall then we know this is incorrect.

If a box is pushed to that it is now connected to one or more boxes, and none of these connected boxes can be pushed,
then we know this is incorrect. We could do some caching - of all the possible moves of all the boxes, and see if
the new position of the box would make is so that there are no moves left for any of them. Implementing this would
also implement #1.
*/

pub fn solve(inital_puzzle: Puzzle) {
    // let mut bag = vec![grid];
    // let mut visited = FxHashSet::from_iter([inital_puzzle.boxes().clone()]);

    let mut visited = FxHashSet::from_iter([inital_puzzle.get_encoding()]);
    let mut bag = VecDeque::from([inital_puzzle]);

    let mut solved_puzzle = None;

    let mut count = -1;
    // let ds = 1;
    let ds = 10000;

    while let Some(puzzle) = bag.pop_front() {
        // if bag.len() % 10000 == 0 {
        //     println!("bag len: {}", bag.len());
        //     println!("{}", puzzle)
        // }

        count += 1;
        if count % ds == 0 {
            println!("\n{} count: {} {}", "-".repeat(30), count, "-".repeat(30));
            println!("Looking at puzzle:\n{}\n", puzzle);
            println!("{:?}", puzzle.moves());
            // println!("{}", puzzle.view_movable_positions());
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

        // let expansions = ExpansionsHelper::find_expansions(
        //     puzzle.find_all_pushes(false).collect::<Vec<_>>(),
        //     puzzle.movable_positions(),
        // );

        for new_puzzle in puzzle.find_expansions() {
            let encoding = new_puzzle.get_encoding();
            if visited.contains(&encoding) {
                continue;
            }

            visited.insert(encoding);
            bag.push_back(new_puzzle);
        }

        // for (pos, dirs) in puzzle.find_all_pushes() {
        // let all_pushes = puzzle.find_all_valid_pushes();
        // if all_pushes
        //     .clone()
        //     .any(|(_, dirs)| dirs.iter().all(|(_, steps)| *steps == 0))
        // {
        //     continue;
        // }

        // if puzzle.check_if_any_box_is_blocked() {
        //     continue;
        // }

        // for (pos, dirs) in all_pushes {
        // for (pos, dirs) in puzzle.find_all_valid_pushes() {
        //     for (dir, &max_steps) in dirs.iter() {
        //         // println!();
        //         // println!("dir: {:?}, max steps: {}", dir, max_steps);
        //         for steps in 1..=max_steps {
        //             // println!("{}", steps);
        //             let mut new_puzzle = puzzle.clone();
        //             new_puzzle.move_box(pos, dir, steps);
        //             // println!("{}", new_puzzle);
        //             // println!();
        //             // println!("{}", new_puzzle.view_movable_positions());
        //             // println!("{:?}, {:?}", new_puzzle.targets(), new_puzzle.boxes());

        //             let encoding = new_puzzle.get_encoding();

        //             if visited.contains(&encoding) {
        //                 continue;
        //             }

        //             visited.insert(encoding);
        //             bag.push_back(new_puzzle);
        //         }
        //     }
        // }
    }

    println!("total iterations: {}", count);
    println!("visited: {}", visited.len());
    if let Some(puzzle) = solved_puzzle {
        println!("Solved:\n{}", puzzle);
        println!("Moves: {:?}", puzzle.moves());
    } else {
        println!("unsolved....");
    }
}
