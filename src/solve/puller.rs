use rustc_hash::FxHashSet;

use super::board::Board;

pub struct Puller {
    board: Board,
}

impl Puller {
    pub fn new(board: Board) -> Self {
        Self { board }
    }

    /// Returns all the squares which a box could be pulled to from any target.
    pub fn find_all_valid_positions(&self) -> FxHashSet<usize> {
        let mut all_squares = FxHashSet::default();
        for &target in self.board.targets.iter() {
            all_squares.extend(self.find_valid_positions_from(target));
        }
        all_squares
    }

    /// Returns all the squares which a box could be pulled to from `target`.
    fn find_valid_positions_from(&self, target: usize) -> FxHashSet<usize> {
        let mut visited = FxHashSet::default();
        let mut bag = vec![target];

        while let Some(current) = bag.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);

            for (dir, box_pos) in self.board.borders_with_dirs(current) {
                if self.board.square_at(box_pos).is_wall() {
                    continue;
                }

                let player_pos = self.board.step(box_pos, dir, 1);
                if let Some(pos) = player_pos {
                    if self.board.square_at(pos).is_space() {
                        bag.push(box_pos);
                    }
                }
            }
        }

        visited
    }
}
