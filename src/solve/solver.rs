#![allow(dead_code)]

use super::puzzle::Puzzle;

struct Solver {
    puzzle: Puzzle,
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Self {
        Self { puzzle }
    }
}

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

impl Solver {
    pub fn solve(&mut self) {}
}
