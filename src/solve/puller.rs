use rustc_hash::FxHashSet;

use super::{directions::PosHelper, squares::Flags};

pub struct Puller<'a> {
    grid: &'a [Flags],
    width: usize,
    height: usize,
    targets: &'a FxHashSet<usize>,
    poshelper: PosHelper,
}

impl<'a> Puller<'a> {
    pub fn new(
        grid: &'a [Flags],
        width: usize,
        height: usize,
        targets: &'a FxHashSet<usize>,
    ) -> Self {
        Self {
            grid,
            width,
            height,
            targets,
            poshelper: PosHelper::new(width, height),
        }
    }

    /// Returns all the squares which a box could be pulled to from any target.
    pub fn find_all_valid_positions(&self) -> FxHashSet<usize> {
        let mut all_squares = FxHashSet::default();
        for &target in self.targets {
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

            for (dir, box_pos) in self.poshelper.borders_with_dirs(current) {
                if self.grid[box_pos].is_wall() {
                    continue;
                }

                let player_pos = self.poshelper.step(box_pos, dir, 1);
                if let Some(pos) = player_pos {
                    if self.grid[pos].is_space() {
                        bag.push(box_pos);
                    }
                }
            }
        }

        visited
    }
}

#[cfg(test)]
mod test_find_squares {
    use super::*;

    #[test]
    fn test_name() {
        todo!()
    }
}
