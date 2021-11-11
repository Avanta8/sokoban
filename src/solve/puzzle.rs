// #![allow(dead_code)]
use bitflags::bitflags;
use rustc_hash::{FxHashMap, FxHashSet};
use std::{collections::VecDeque, fmt};

use crate::question;

#[derive(Clone, Copy, Debug)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::North, Self::East, Self::South, Self::West].into_iter()
    }

    fn opposite_of(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

bitflags! {
    pub struct Flags:u8 {
        const WALL     = 0b00001;
        const SPACE    = 0b00010;
        const PLAYER   = 0b00100;
        const BOX      = 0b01000;
        const TARGET   = 0b10000;

        const WALKABLE = Self::SPACE.bits | Self::TARGET.bits | Self::PLAYER.bits;
    }
}

impl Flags {
    /// Returns `true` if there is nothing on top of the square. ie. If the square is
    /// a space but there is not a player or box above it. (It can be a target.)
    fn is_walkable(&self) -> bool {
        *self | Self::TARGET | Self::PLAYER == Self::WALKABLE
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

#[derive(Default)]
pub struct DirHolder<T> {
    north: T,
    south: T,
    east: T,
    west: T,
}

impl<T> DirHolder<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Dir, &T)> {
        [
            (Dir::North, &self.north),
            (Dir::East, &self.east),
            (Dir::South, &self.south),
            (Dir::West, &self.west),
        ]
        .into_iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Dir, &mut T)> {
        [
            (Dir::North, &mut self.north),
            (Dir::East, &mut self.east),
            (Dir::South, &mut self.south),
            (Dir::West, &mut self.west),
        ]
        .into_iter()
    }
    #[allow(dead_code)]
    pub fn set(&mut self, dir: Dir, val: T) {
        match dir {
            Dir::North => self.north = val,
            Dir::South => self.south = val,
            Dir::East => self.east = val,
            Dir::West => self.west = val,
        }
    }
}

/// Joins a 2d vector of strings into a single output string.
///
/// Each item of a row is joined without any padding.
/// Each row is joined by a newline.
///
/// Each item should be the same number of characters long, as this
/// method does not do any padding / formatting.
fn vec2d_to_string(grid: Vec<Vec<String>>) -> String {
    grid.iter()
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Default, Clone)]
pub struct Puzzle {
    grid: Vec<Flags>,
    width: usize,
    height: usize,
    boxes: FxHashSet<usize>,
    targets: FxHashSet<usize>,
    player_pos: usize,
    moves: Vec<Dir>,

    // poshelper: PositionHelper,
    /// `movable_positions` should always be kept updated.
    movable_positions: FxHashSet<usize>,
}

impl Puzzle {
    pub fn grid(&self) -> &Vec<Flags> {
        &self.grid
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    #[allow(dead_code)]
    pub fn boxes(&self) -> &FxHashSet<usize> {
        &self.boxes
    }

    #[allow(dead_code)]
    pub fn targets(&self) -> &FxHashSet<usize> {
        &self.targets
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", vec2d_to_string(self.get_2d_grid_vec()))
    }
}

impl Puzzle {
    /// Returns the grid as a 2d vector of strings corresponding to each flag.
    fn get_2d_grid_vec(&self) -> Vec<Vec<String>> {
        self.grid
            .chunks_exact(self.width)
            .map(|row| row.iter().map(|f| f.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    /// Returns a string view of the movable positions in the grid.
    pub fn view_movable_positions(&self) -> String {
        let mut grid = self.get_2d_grid_vec();
        for &pos in self.movable_positions() {
            // if pos == self.player_pos {
            //     continue;
            // }
            grid[pos / self.width][pos % self.width] = "+".to_string();
        }
        vec2d_to_string(grid)
    }

    // Computes and returs all the positions the player can move to without pushing any boxes.
    fn find_movable_positions(&self) -> FxHashSet<usize> {
        let mut bag = vec![self.player_pos];
        let mut visited = FxHashSet::from_iter(bag.clone());

        while !bag.is_empty() {
            let current = bag.pop().unwrap();

            for new_pos in self.pos_borders(current) {
                if self.grid[new_pos].is_walkable() && !visited.contains(&new_pos) {
                    bag.push(new_pos);
                    visited.insert(new_pos);
                }
            }
        }

        visited
    }

    pub fn update_movable_positions(&mut self) {
        self.movable_positions = self.find_movable_positions()
    }

    pub fn movable_positions(&self) -> &FxHashSet<usize> {
        &self.movable_positions
    }
}

impl Puzzle {
    /// Returns the directions each box can be pushed in, and the distance they can be moved in that direction.
    pub fn find_all_pushes(&self) -> impl Iterator<Item = (usize, DirHolder<usize>)> + '_ {
        self.boxes.iter().map(|&box_pos| {
            let mut possible_steps = DirHolder::<usize>::default();

            possible_steps.iter_mut().for_each(|(dir, steps)| {
                // Check that the push square is within bounds.
                // println!("{:?}", dir);
                if let Some(push_pos) = self.get_push_pos(box_pos, dir) {
                    // Check that the push square can be walked on and reached.
                    // println!(
                    //     "sdfdsf {:?}, is walkabl: {}, can move to: {}",
                    //     dir,
                    //     self.grid[push_pos].is_walkable(),
                    //     self.can_move_to(push_pos)
                    // );
                    if self.grid[push_pos].is_walkable() && self.can_move_to(push_pos) {
                        // println!("able {:?}", dir);
                        let mut new_pos = box_pos;
                        while let Some(p) = self.pos_move(new_pos, dir, 1) {
                            if !self.grid[p].is_walkable() {
                                break;
                            }

                            new_pos = p;
                            *steps += 1;
                        }
                    }
                }
            });
            (box_pos, possible_steps)
        })
    }

    /// Returns true if the player can move to `pos`.
    fn can_move_to(&self, pos: usize) -> bool {
        self.movable_positions.contains(&pos)
    }

    /// Makes the move. The move must be valid.
    ///
    /// `pos` is the position of the box that should be moved.
    pub fn move_box(&mut self, pos: usize, dir: Dir, steps: usize) {
        assert!(
            self.boxes.contains(&pos),
            "pos {} is not in the boxes.",
            pos
        );
        assert!(
            self.grid[pos].is_box(),
            "pos {} is not a box in the grid. But it does appear in the boxes set.",
            pos
        );

        let push_pos = self.get_push_pos(pos, dir).unwrap_or_else(|| {
            panic!("The push square of a box on pos: {} is out of bounds.", pos)
        });
        self.move_to(push_pos);

        let new_box_pos = self
            .pos_move(pos, dir, steps)
            .expect("was not a valid move");
        self.update_player_pos(
            self.get_push_pos(new_box_pos, dir)
                .expect("was not a valid move. Player ended up out of bounds."),
        );
        self.update_box_pos(pos, new_box_pos);

        self.update_movable_positions();
    }

    /// Moves the player position to `pos`.
    pub fn move_to(&mut self, target: usize) {
        assert!(
            self.can_move_to(target),
            "pos: {} cannot be moved to without pushing any box.",
            target
        );

        let mut bag = VecDeque::new();
        bag.push_back(target);

        let mut visited = FxHashMap::<usize, Option<Dir>>::default();
        visited.insert(target, None);

        while let Some(current) = bag.pop_front() {
            if current == target {
                break;
            }

            for (dir, new_pos) in self.pos_borders_with_dirs(target) {
                if self.grid[new_pos].is_walkable() && !visited.contains_key(&new_pos) {
                    bag.push_back(new_pos);
                    visited.insert(new_pos, Some(dir));
                }
            }
        }

        assert!(
            visited.contains_key(&target),
            "pos: {} was in the movable positions, but it wasn't acutally able to be moved to",
            target
        );

        let mut moves = VecDeque::new();
        let mut pos = target;
        while let Some(&Some(dir)) = visited.get(&pos) {
            moves.push_back(dir);
            pos = self
                .pos_move(pos, dir.opposite_of(), 1)
                .expect("Rebuilding path encountered out of bounds position.");
        }

        // Not required as of yet.
        // BUT MAY DO IN THE FUTURE!!
        // self.update_player_pos(target);
        // self.update_movable_positions();

        self.add_moves(moves);
    }

    fn update_box_pos(&mut self, old_pos: usize, new_pos: usize) {
        assert!(
            self.grid[old_pos].is_box(),
            "old_pos was {} but that square isn't a box on the grid.",
            old_pos
        );
        assert!(
            self.boxes.contains(&old_pos),
            "old_pas was {}. It was a box on the grid. But wasn't a box in self.boxes",
            old_pos
        );

        self.grid[old_pos] &= !Flags::BOX;
        self.grid[new_pos] |= Flags::BOX;
        self.boxes.remove(&old_pos);
        self.boxes.insert(new_pos);
    }

    fn update_player_pos(&mut self, new_pos: usize) {
        assert!(
            self.grid[self.player_pos].is_player(),
            "Player should be on pos: {} but it wasn't there in the grid.",
            self.player_pos
        );

        self.grid[self.player_pos] &= !Flags::PLAYER;
        self.grid[new_pos] |= Flags::PLAYER;
        self.player_pos = new_pos;
    }

    fn add_moves(&mut self, moves: impl IntoIterator<Item = Dir>) {
        self.moves.extend(moves);
    }

    /// Returns the position the player would need to stand on to push a box placed
    /// on `pos` in `dir` direction, or None if the position is out of bounds.
    fn get_push_pos(&self, pos: usize, dir: Dir) -> Option<usize> {
        self.pos_move(pos, dir.opposite_of(), 1)
    }
}

impl Puzzle {
    pub fn is_solved(&self) -> bool {
        self.targets
            .iter()
            .all(|target| self.boxes.contains(target))
    }

    /// Retuns the position that you would end up on after moving `steps` in `dir` direction startin from `pos`.
    /// Returns `None` if the resulting position would be out of bounds.
    fn pos_move(&self, mut pos: usize, dir: Dir, steps: usize) -> Option<usize> {
        let f = match dir {
            Dir::North => Self::pos_north,
            Dir::South => Self::pos_south,
            Dir::East => Self::pos_east,
            Dir::West => Self::pos_west,
        };
        for _ in 0..steps {
            match f(self, pos) {
                Some(p) => pos = p,
                None => return None,
            }
        }
        Some(pos)
    }
    fn pos_north(&self, pos: usize) -> Option<usize> {
        pos.checked_sub(self.width)
    }
    fn pos_south(&self, pos: usize) -> Option<usize> {
        if pos / self.width + 1 == self.height {
            return None;
        }
        Some(pos + self.width)
    }
    fn pos_east(&self, pos: usize) -> Option<usize> {
        if (pos + 1) % self.width == 0 {
            return None;
        }
        Some(pos + 1)
    }
    fn pos_west(&self, pos: usize) -> Option<usize> {
        if pos % self.width == 0 {
            return None;
        }
        Some(pos - 1)
    }
    /// Returns an iterator of all the bordering positions around `pos` that are
    /// not out of bounds. (Doesn't check what the square acutally is. Just checks that
    /// it isn't out of bounds.)
    fn pos_borders(&self, pos: usize) -> impl Iterator<Item = usize> {
        [
            self.pos_north(pos),
            self.pos_east(pos),
            self.pos_south(pos),
            self.pos_west(pos),
        ]
        .into_iter()
        .flatten()
    }
    /// Returns all the positions around `pos` that are not out of bounds as an iterator of
    /// tuples `(direction moved, resultant position)`.
    fn pos_borders_with_dirs(&self, pos: usize) -> impl Iterator<Item = (Dir, usize)> + '_ {
        Dir::iter().filter_map(move |dir| self.pos_move(pos, dir, 1).map(|new_pos| (dir, new_pos)))
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
            player_pos: start,
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
