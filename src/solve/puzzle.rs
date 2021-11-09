#![allow(dead_code)]

use bitflags::bitflags;
use rustc_hash::FxHashSet;

use crate::question;

bitflags! {
    struct Flags:u8 {
        const WALL     = 0b00001;
        const SPACE    = 0b00010;
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
        Some(pos + 1)
    }
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

impl Puzzle {
    pub fn move_pos(&mut self, pos: usize) -> Result<(), &'static str> {
        if !self.movable_positions.contains(&pos) {
            return Err("Wanted to move to a position that cannot be reached");
        }

        self.grid[self.pos] &= !Flags::PLAYER;
        self.grid[pos] |= Flags::PLAYER;
        self.pos = pos;

        Ok(())
    }
}

impl Puzzle {
    fn find_movable_positions(&self) -> FxHashSet<usize> {
        let mut bag = vec![self.pos];
        let mut visited = FxHashSet::from_iter(bag.clone());

        while !bag.is_empty() {
            let current = bag.pop().unwrap();

            for new_pos in [
                self.poshelper.west(current),
                self.poshelper.east(current),
                self.poshelper.north(current),
                self.poshelper.south(current),
            ]
            .into_iter()
            .flatten()
            {
                if !visited.contains(&new_pos) {
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

    /// Returns `true` if the player can move to `pos` from its current position.
    pub fn can_move_to(&self, pos: usize) -> bool {
        self.movable_positions.contains(&pos)
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
}

// impl From<&question::Question> for Puzzle {
//     fn from(question: &question::Question) -> Self {
impl<Q: std::borrow::Borrow<question::Question>> From<Q> for Puzzle {
    fn from(question: Q) -> Self {
        println!("In Puzzle::From<Question>");
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
