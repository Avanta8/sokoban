use rustc_hash::FxHashSet;

use super::directions::DirHolder;

type Pushes = Vec<(usize, DirHolder<usize>)>;

pub struct ExpansionsHelper<'a> {
    all_pushes: Pushes,
    movable_positions: &'a FxHashSet<usize>,
}

impl<'a> ExpansionsHelper<'a> {
    pub fn new(all_pushes: Pushes, movable_positions: &'a FxHashSet<usize>) -> Self {
        Self {
            all_pushes,
            movable_positions,
        }
    }

    pub fn find_expansions(all_pushes: Pushes, movable_positions: &'a FxHashSet<usize>) -> Pushes {
        todo!()
    }

    fn solve(&self) -> Pushes {
        todo!()
    }
}
