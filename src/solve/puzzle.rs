#![allow(dead_code)]
use bitflags::bitflags;
use rustc_hash::FxHashSet;
use std::fmt;

use crate::question;

bitflags! {
    struct Flags:u8 {
        const WALL     = 0b00001;
        const SPACE    = 0b00010;
        // Perhaps all these flags underneaths should also contain the flag for SPACE?
        const PLAYER   = 0b00100;
        const BOX      = 0b01000;
        const TARGET   = 0b10000;

        const WALKABLE  = Self::SPACE.bits | Self::TARGET.bits;
    }
}

impl Flags {
    /// Returns `true` if there is nothing on top of the square. ie. If the square is
    /// a space but there is not a player or box above it. (It can be a target.)
    fn is_walkable(&self) -> bool {
        *self | Self::TARGET == Self::WALKABLE
    }
    fn is_wall(&self) -> bool {
        *self == Self::WALL
    }
    fn is_player(&self) -> bool {
        self.contains(Self::PLAYER)
    }
    fn is_box(&self) -> bool {
        self.contains(Self::BOX)
    }
    fn is_target(&self) -> bool {
        self.contains(Self::TARGET)
    }
    fn is_placed(&self) -> bool {
        self.contains(Self::TARGET | Self::BOX)
    }
    fn is_space(&self) -> bool {
        self.contains(Self::SPACE)
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = if self.is_wall() {
            '#'
        } else if self.is_player() {
            '@'
        } else if self.is_placed() {
            '*'
        } else if self.is_box() {
            '$'
        } else if self.is_target() {
            '.'
        } else if self.is_space() {
            ' '
        } else {
            unreachable!("Impossible square flag: {:?}", self);
        };

        write!(f, "{}", c)
    }
}

#[derive(Debug, Default)]
struct PositionHelper {
    width: usize,
    height: usize,
}

impl PositionHelper {
    /// Returns the position directly to the west of `pos`.
    fn west(&self, pos: usize) -> Option<usize> {
        if pos % self.width == 0 {
            return None;
        }
        Some(pos - 1)
    }

    /// Returns the position directly to the east of `pos`.
    fn east(&self, pos: usize) -> Option<usize> {
        if (pos + 1) % self.width == 0 {
            return None;
        }
        Some(pos + 1)
    }

    /// Returns the position directly to the north of `pos`.
    fn north(&self, pos: usize) -> Option<usize> {
        pos.checked_sub(self.width)
    }

    /// Returns the position directly to the south of `pos`.
    fn south(&self, pos: usize) -> Option<usize> {
        if pos / self.width + 1 == self.height {
            return None;
        }
        Some(pos + self.width)
    }

    fn get_borders(&self, pos: usize) -> impl Iterator<Item = usize> {
        [
            self.north(pos),
            self.east(pos),
            self.south(pos),
            self.west(pos),
        ]
        .into_iter()
        .flatten()
    }
}

fn vec2d_to_string(grid: Vec<Vec<String>>) -> String {
    grid.iter()
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Default)]
pub struct Puzzle {
    grid: Vec<Flags>,
    width: usize,
    height: usize,
    boxes: FxHashSet<usize>,
    targets: FxHashSet<usize>,
    pos: usize,

    poshelper: PositionHelper,

    /// `movable_positions` should always be kept updated.
    movable_positions: FxHashSet<usize>,
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", vec2d_to_string(self.get_2d_grid_vec()))
    }
}

impl Puzzle {
    fn get_2d_grid_vec(&self) -> Vec<Vec<String>> {
        self.grid
            .chunks_exact(self.width)
            .map(|row| row.iter().map(|f| f.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    /// Returns a string view of the movable positions in the grid.
    pub fn view_movable_positions(&self) -> String {
        let mut grid = self.get_2d_grid_vec();
        for &pos in self.get_movable_positions() {
            if pos == self.pos {
                continue;
            }
            grid[pos / self.width][pos % self.width] = "+".to_string();
        }
        vec2d_to_string(grid)
    }
}

impl Puzzle {
    pub fn move_pos(&mut self, pos: usize) -> Result<(), &'static str> {
        if !self.movable_positions.contains(&pos) {
            return Err("Wanted to move to a position that cannot be reached");
        }

        self.grid[self.pos] &= !Flags::PLAYER;
        self.grid[pos] |= Flags::PLAYER;
        self.pos = pos;

        // // Slow as it has to dfs all over again. Faster would be know that we can move to
        // // any position we could have come from.
        // self.update_movable_positions();

        Ok(())
    }
}

impl Puzzle {
    fn find_movable_positions(&self) -> FxHashSet<usize> {
        let mut bag = vec![self.pos];
        let mut visited = FxHashSet::from_iter(bag.clone());

        while !bag.is_empty() {
            let current = bag.pop().unwrap();

            for new_pos in self.poshelper.get_borders(current) {
                if self.is_clear(new_pos) && !visited.contains(&new_pos) {
                    bag.push(new_pos);
                    visited.insert(new_pos);
                }
            }
        }

        visited
    }

    fn set_movable_positions(&mut self, positions: FxHashSet<usize>) {
        self.movable_positions = positions
    }

    fn update_movable_positions(&mut self) {
        self.set_movable_positions(self.find_movable_positions())
    }

    pub fn get_movable_positions(&self) -> &FxHashSet<usize> {
        &self.movable_positions
    }

    /// Returns `true` if the player can move to `pos` from its current position.
    pub fn can_move_to(&self, pos: usize) -> bool {
        self.movable_positions.contains(&pos)
    }
}

impl Puzzle {
    fn find_possible_pushes(&self) {
        for pos in self.boxes.iter() {}
    }
}

impl Puzzle {
    /// Returns `true` if a box placed on `pos` could be pushed to the west.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_west(&self, pos: usize) -> bool {
        let east = self.poshelper.east(pos);
        east.is_some() && self.is_clear_west(pos) && self.can_move_to(east.unwrap())
    }
    /// Returns `true` if a box placed on `pos` could be pushed to the east.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_east(&self, pos: usize) -> bool {
        let west = self.poshelper.west(pos);
        west.is_some() && self.is_clear_east(pos) && self.can_move_to(west.unwrap())
    }
    /// Returns `true` if a box placed on `pos` could be pushed to the north.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_north(&self, pos: usize) -> bool {
        let south = self.poshelper.south(pos);
        south.is_some() && self.is_clear_north(pos) && self.can_move_to(south.unwrap())
    }
    /// Returns `true` if a box placed on `pos` could be pushed to the south.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_south(&self, pos: usize) -> bool {
        let north = self.poshelper.north(pos);
        north.is_some() && self.is_clear_south(pos) && self.can_move_to(north.unwrap())
    }
}

impl Puzzle {
    /// Returns `true` if there is nothing on top of the square corresponding to `pos`.
    pub fn is_clear(&self, pos: usize) -> bool {
        self.grid[pos].is_walkable()
    }

    /// Returns `true` if there is nothing on top of the square directly to the west of `pos`.
    pub fn is_clear_west(&self, pos: usize) -> bool {
        self.west_of(pos).is_walkable()
    }
    /// Returns `true` if there is nothing on top of the square directly to the east of `pos`.
    pub fn is_clear_east(&self, pos: usize) -> bool {
        self.east_of(pos).is_walkable()
    }
    /// Returns `true` if there is nothing on top of the square directly to the north of `pos`.
    pub fn is_clear_north(&self, pos: usize) -> bool {
        self.north_of(pos).is_walkable()
    }
    /// Returns `true` if there is nothing on top of the square directly to the south of `pos`.
    pub fn is_clear_south(&self, pos: usize) -> bool {
        self.south_of(pos).is_walkable()
    }

    /// Returns the square directly to the west of `pos`. A wall is returned for an out of
    /// bounds position.
    fn west_of(&self, pos: usize) -> Flags {
        self._of_helper(self.poshelper.west(pos))
    }
    /// Returns the square directly to the east of `pos`. A wall is returned for an out of
    /// bounds position.
    fn east_of(&self, pos: usize) -> Flags {
        self._of_helper(self.poshelper.east(pos))
    }
    /// Returns the square directly to the north of `pos`. A wall is returned for an out of
    /// bounds position.
    fn north_of(&self, pos: usize) -> Flags {
        self._of_helper(self.poshelper.north(pos))
    }
    /// Returns the square directly to the south of `pos`. A wall is returned for an out of
    /// bounds position.
    fn south_of(&self, pos: usize) -> Flags {
        self._of_helper(self.poshelper.south(pos))
    }
    fn _of_helper(&self, new_pos: Option<usize>) -> Flags {
        match new_pos {
            Some(pos) => self.grid[pos],
            None => Flags::WALL,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<Q: std::borrow::Borrow<question::Question>> From<Q> for Puzzle {
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
        grid[start] |= Flags::PLAYER;

        let mut mapper =
            |it: &std::collections::HashSet<question::Position>, flag: Flags| -> FxHashSet<usize> {
                it.iter()
                    .map(|p| {
                        let idx = p.to_usize(width);
                        grid[idx] |= flag; // add the flag to the corresponding square on the grid.
                        idx
                    })
                    .collect::<FxHashSet<_>>()
            };

        let boxes = mapper(question.boxes(), Flags::BOX);
        let targets = mapper(question.targets(), Flags::TARGET);

        let mut ret = Self {
            grid,
            width,
            height,
            boxes,
            targets,
            pos: start,
            poshelper: PositionHelper { height, width },
            ..Self::default()
        };
        ret.update_movable_positions();
        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_puzzle(filename: &str, idx: usize) -> Puzzle {
        use crate::reader::test_config::create_collection;
        (&create_collection(filename)[idx]).into()
    }

    const FILENAME: &str = "puzzles.txt";

    #[test]
    fn test_directions() {
        let puzzle = create_puzzle(FILENAME, 0);
        println!("{}", puzzle);
    }
}
