#![allow(dead_code)]

use bitflags::bitflags;
use rustc_hash::FxHashSet;

use crate::question;

bitflags! {
    struct Flags:u8 {
        const WALL     = 0b00001;
        const OPEN     = 0b00010;
        const PLAYER   = 0b00100;
        const BOX      = 0b01000;
        const TARGET   = 0b10000;

        const MOVABLE  = Self::OPEN.bits | Self::TARGET.bits;
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

            for method in [Self::west_of, Self::east_of, Self::north_of, Self::south_of] {
                if let Some(new_pos) = method(self, current) {
                    if !visited.contains(&new_pos) {
                        visited.insert(new_pos);
                    }
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
}

impl Puzzle {
    /// Returns `true` if a box placed on `pos` could be pushed to the west.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_west(&self, pos: usize) -> bool {
        let east = self.east_of(pos);
        east.is_some() && self.unblocked_west(pos) && self.can_move_to(east.unwrap())
    }
    /// Returns `true` if a box placed on `pos` could be pushed to the east.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_east(&self, pos: usize) -> bool {
        let west = self.west_of(pos);
        west.is_some() && self.unblocked_east(pos) && self.can_move_to(west.unwrap())
    }
    /// Returns `true` if a box placed on `pos` could be pushed to the north.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_north(&self, pos: usize) -> bool {
        let south = self.south_of(pos);
        south.is_some() && self.unblocked_north(pos) && self.can_move_to(south.unwrap())
    }
    /// Returns `true` if a box placed on `pos` could be pushed to the south.
    ///
    /// There doesn't have to be a box currently placed on `pos`.
    pub fn can_push_south(&self, pos: usize) -> bool {
        let north = self.north_of(pos);
        north.is_some() && self.unblocked_south(pos) && self.can_move_to(north.unwrap())
    }
}

impl Puzzle {
    /// Returns `true` if there is nothing on top of the square directly to the west of `pos`.
    pub fn unblocked_west(&self, pos: usize) -> bool {
        self._unblocked(self.west_of(pos))
    }
    /// Returns `true` if there is nothing on top of the square directly to the east of `pos`.
    pub fn unblocked_east(&self, pos: usize) -> bool {
        self._unblocked(self.east_of(pos))
    }
    /// Returns `true` if there is nothing on top of the square directly to the north of `pos`.
    pub fn unblocked_north(&self, pos: usize) -> bool {
        self._unblocked(self.north_of(pos))
    }
    /// Returns `true` if there is nothing on top of the square directly to the south of `pos`.
    pub fn unblocked_south(&self, pos: usize) -> bool {
        self._unblocked(self.south_of(pos))
    }
    fn _unblocked(&self, pos: Option<usize>) -> bool {
        match pos {
            Some(p) => self.is_unblocked(p),
            None => false,
        }
    }

    /// Returns `true` if there is nothing on top of the square corresponding to `pos`.
    pub fn is_unblocked(&self, pos: usize) -> bool {
        self.grid[pos] | Flags::TARGET == Flags::MOVABLE
    }

    /// Returns `true` if the character can move to `pos` from its current position.
    pub fn can_move_to(&self, pos: usize) -> bool {
        self.movable_positions.contains(&pos)
    }

    //
    // Perhaps these follwing methods should be called slightly differently.
    // The methods with these names should instead return Option<item on the direction>.

    /// Returns the position directly to the west of `pos`.
    fn west_of(&self, pos: usize) -> Option<usize> {
        pos.checked_sub(1)
    }
    /// Returns the position directly to the east of `pos`.
    fn east_of(&self, pos: usize) -> Option<usize> {
        pos.checked_add(1)
    }
    /// Returns the position directly to the north of `pos`.
    fn north_of(&self, pos: usize) -> Option<usize> {
        pos.checked_sub(self.width)
    }
    /// Returns the position directly to the south of `pos`.
    fn south_of(&self, pos: usize) -> Option<usize> {
        pos.checked_add(self.width)
    }
}

impl From<question::Question> for Puzzle {
    fn from(question: question::Question) -> Self {
        let (width, height) = (question.width(), question.height());
        let mut grid = Vec::with_capacity(width * height);
        for row in question.rows() {
            for sq in row {
                grid.push(match *sq {
                    question::Square::Wall => Flags::WALL,
                    question::Square::Space => Flags::OPEN,
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

        Self {
            grid,
            width,
            height,
            boxes,
            targets,
            pos: start,
            ..Self::default()
        }
    }
}
