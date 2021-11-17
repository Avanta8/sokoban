use rustc_hash::FxHashSet;

use super::{board::Board, directions::Dir};

pub struct Detector {
    board: Board,
}

impl Detector {
    pub fn new(board: &Board) -> Self {
        Self {
            board: board.to_owned(),
        }
    }

    /// Returns `true` if it is sure that the position is deadlocked.
    pub fn is_deadlocked(
        &self,
        player_pos: usize,
        boxes: &FxHashSet<usize>,
        last_moved: usize,
    ) -> bool {
        !self.board.targets.contains(&last_moved)
            && (self.is_dead_square(boxes, last_moved) || self.check_box_blocked(boxes, last_moved))
    }
}

impl Detector {
    fn is_dead_square(&self, boxes: &FxHashSet<usize>, last_moved: usize) -> bool {
        !self.board.square_at(last_moved).is_valid()
    }
}

impl Detector {
    fn check_box_blocked(&self, boxes: &FxHashSet<usize>, box_pos: usize) -> bool {
        let mut considered = FxHashSet::default();
        self.check_box_blocked_direction(boxes, box_pos, &mut considered, Dir::North)
            && self.check_box_blocked_direction(boxes, box_pos, &mut considered, Dir::East)
    }

    fn check_box_blocked_direction(
        &self,
        boxes: &FxHashSet<usize>,
        box_pos: usize,
        considered: &mut FxHashSet<usize>,
        dir: Dir,
    ) -> bool {
        considered.insert(box_pos);
        let a = self.board.step(box_pos, dir, 1).unwrap();
        let b = self.board.step(box_pos, dir.opposite(), 1).unwrap();

        self.board.square_at(a).is_wall()
            || self.board.square_at(b).is_wall()
            || !self.board.square_at(a).is_valid() && !self.board.square_at(b).is_valid()
            || considered.contains(&a)
            || considered.contains(&b)
            || boxes.contains(&a)
                && self.check_box_blocked_direction(boxes, a, considered, dir.rotation())
            || boxes.contains(&b)
                && self.check_box_blocked_direction(boxes, b, considered, dir.rotation())
    }
}
