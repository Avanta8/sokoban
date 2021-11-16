// #![allow(dead_code)]
use rustc_hash::{FxHashMap, FxHashSet};
use std::rc::Rc;
use std::{collections::VecDeque, fmt};

use crate::question;

use super::directions::{Dir, DirHolder, PosHelper};
use super::expansions::ExpansionsHelper;
use super::puller::Puller;
use super::squares::Flags;

/// Joins a 2d vector of strings into a single output string.
///
/// Each item of a row is joined without any padding.
/// Each row is joined by a newline.
///
/// Each item should be the same number of characters long, as this
/// method does not do any padding / formatting.
fn vec2d_to_string(grid: Vec<Vec<&str>>) -> String {
    grid.iter()
        .map(|row| row.join(""))
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Default, Clone)]
pub struct Puzzle {
    grid: Rc<Vec<Flags>>,
    width: usize,
    height: usize,
    boxes: FxHashSet<usize>,
    targets: Rc<FxHashSet<usize>>,
    player_pos: usize,
    moves: Vec<Dir>,

    poshelper: Rc<PosHelper>,

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

    pub fn boxes(&self) -> &FxHashSet<usize> {
        &self.boxes
    }

    pub fn targets(&self) -> &FxHashSet<usize> {
        &self.targets
    }

    pub fn movable_positions(&self) -> &FxHashSet<usize> {
        &self.movable_positions
    }

    pub fn moves(&self) -> &Vec<Dir> {
        &self.moves
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", vec2d_to_string(self.get_2d_grid_vec()))
    }
}

impl Puzzle {
    /// Returns the grid as a 2d vector of strings corresponding to each flag.
    fn get_2d_grid_vec(&self) -> Vec<Vec<&str>> {
        let mut grid = (*self.grid)
            .clone()
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>();

        for &pos in self.boxes.iter() {
            grid[pos] = "$"
        }
        for &pos in self.targets.iter() {
            grid[pos] = if self.boxes.contains(&pos) { "*" } else { "." }
        }
        grid[self.player_pos] = "@";

        grid.chunks_exact(self.width)
            .map(|row| row.to_vec())
            .collect::<Vec<_>>()
    }

    pub fn view_valid_positions(&self) -> String {
        let mut grid = self.get_2d_grid_vec();
        for (pos, sq) in self.grid.iter().enumerate() {
            if sq.is_valid() {
                grid[pos / self.width][pos % self.width] = "O";
            }
        }
        vec2d_to_string(grid)
    }

    /// Returns a string view of the movable positions in the grid.
    pub fn view_movable_positions(&self) -> String {
        let mut grid = self.get_2d_grid_vec();
        for &pos in self.movable_positions.iter() {
            grid[pos / self.width][pos % self.width] = "+";
        }
        vec2d_to_string(grid)
    }

    /// Computes and returs all the positions the player can move to without pushing any boxes.
    fn find_movable_positions(&self) -> FxHashSet<usize> {
        let mut bag = vec![self.player_pos];
        let mut visited = FxHashSet::from_iter(bag.clone());

        while !bag.is_empty() {
            let current = bag.pop().unwrap();

            for new_pos in self.poshelper.borders(current) {
                if self.is_pos_walkable(new_pos) && !visited.contains(&new_pos) {
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
}

// #[allow(dead_code)]
impl Puzzle {
    /// Moves the player position the top left square that can be reached without having
    /// to push any boxes. Top left is the leftmost position on the upmost row.
    fn move_to_top_left(&mut self) {
        let &top_left = self.movable_positions.iter().min().unwrap();
        self.move_to(top_left);
    }

    /// Returns the directions each box can be pushed in, and the distance they can be moved in that direction.
    /// If `reachable` is true, then the player must be able to reach the position required to move the box
    /// without having to push anything to get there.
    pub fn find_all_pushes(
        &self,
        reachable: bool,
    ) -> impl Iterator<Item = (usize, DirHolder<usize>)> + Clone + '_ {
        self.boxes.iter().map(move |&box_pos| {
            let mut possible_steps = DirHolder::<usize>::default();

            possible_steps.iter_mut().for_each(|(dir, steps)| {
                // Check that the push square is within bounds.
                // println!("{:?}", dir);
                if let Some(push_pos) = self.get_push_pos(box_pos, dir) {
                    // Check that the push square can be walked on and reached.
                    if self.is_pos_walkable(push_pos) && (!reachable || self.can_move_to(push_pos))
                    {
                        // println!("able {:?}", dir);
                        let mut new_pos = box_pos;
                        while let Some(p) = self.poshelper.step(new_pos, dir, 1) {
                            if !self.grid[p].is_valid() || self.boxes.contains(&p) {
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

        let push_pos = self.get_push_pos(pos, dir).unwrap_or_else(|| {
            panic!("The push square of a box on pos: {} is out of bounds.", pos)
        });
        self.move_to(push_pos);

        let new_box_pos = self
            .poshelper
            .step(pos, dir, steps)
            .expect("was not a valid move");

        let new_player_pos = self
            .get_push_pos(new_box_pos, dir)
            .expect("was not a valid move. Player ended up out of bounds.");

        self.update_box_pos(pos, new_box_pos);
        self.update_player_pos(new_box_pos, true);

        for _ in 0..steps {
            self.moves.push(dir);
        }
    }

    /// Moves the player position to `pos`.
    pub fn move_to(&mut self, target: usize) {
        assert!(
            self.can_move_to(target),
            "pos: {} cannot be moved to without pushing any box.",
            target
        );

        let mut bag = VecDeque::new();
        bag.push_back(self.player_pos);

        let mut visited = FxHashMap::<usize, Option<Dir>>::default();
        visited.insert(self.player_pos, None);

        while let Some(current) = bag.pop_front() {
            if current == target {
                break;
            }

            for (dir, new_pos) in self.poshelper.borders_with_dirs(current) {
                if self.is_pos_walkable(new_pos) && !visited.contains_key(&new_pos) {
                    assert!(!self.grid[new_pos].is_wall());
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
            moves.push_front(dir);
            pos = self
                .poshelper
                .step(pos, dir.opposite(), 1)
                .expect("Rebuilding path encountered out of bounds position.");
        }

        self.update_player_pos(target, false);
        self.add_moves(moves);
    }

    fn update_box_pos(&mut self, old_pos: usize, new_pos: usize) {
        assert!(
            self.boxes.contains(&old_pos),
            "old_pas was {}. It was a box on the grid. But wasn't a box in self.boxes",
            old_pos
        );

        self.boxes.remove(&old_pos);
        self.boxes.insert(new_pos);
    }

    fn update_player_pos(&mut self, new_pos: usize, update: bool) {
        self.player_pos = new_pos;
        if update {
            self.update_movable_positions();
        }
    }

    fn add_moves(&mut self, moves: impl IntoIterator<Item = Dir>) {
        self.moves.extend(moves);
    }

    /// Returns the position the player would need to stand on to push a box placed
    /// on `pos` in `dir` direction, or None if the position is out of bounds.
    fn get_push_pos(&self, pos: usize, dir: Dir) -> Option<usize> {
        self.poshelper.step(pos, dir.opposite(), 1)
    }

    fn is_pos_walkable(&self, pos: usize) -> bool {
        self.grid[pos].is_space() && !self.boxes.contains(&pos)
    }

    pub fn get_encoding(&self) -> Vec<usize> {
        let mut v = self.boxes.iter().copied().collect::<Vec<_>>();
        v.push(self.player_pos);
        v
    }
}

impl Puzzle {
    pub fn is_solved(&self) -> bool {
        self.targets
            .iter()
            .all(|target| self.boxes.contains(target))
    }
}

impl Puzzle {
    pub fn find_expansions(&self) -> Vec<Self> {
        let mut expansions = vec![];

        for (box_pos, dirs) in self.find_all_pushes(true) {
            for (dir, &max_steps) in dirs.iter() {
                for steps in 1..=max_steps {
                    let new_puzzle = self.find_expansions_helper(box_pos, dir, steps);
                    if let Some(mut puzzle) = new_puzzle {
                        puzzle.move_to_top_left();
                        expansions.push(puzzle);
                    }
                }
            }
        }

        expansions
    }

    fn find_expansions_helper(&self, box_pos: usize, dir: Dir, steps: usize) -> Option<Self> {
        let mut new_puzzle = self.clone();

        new_puzzle.move_box(box_pos, dir, steps);
        let new_pos = self.poshelper.step(box_pos, dir, steps).unwrap();
        // if false {
        if !self.targets.contains(&new_pos) && new_puzzle.check_box_blocked(new_pos) {
            None
        } else {
            Some(new_puzzle)
        }
    }

    fn check_box_blocked(&self, box_pos: usize) -> bool {
        let mut considered = FxHashSet::default();
        self.check_box_blocked_direction(box_pos, &mut considered, Dir::North)
            && self.check_box_blocked_direction(box_pos, &mut considered, Dir::East)
    }

    fn check_box_blocked_direction(
        &self,
        box_pos: usize,
        considered: &mut FxHashSet<usize>,
        dir: Dir,
    ) -> bool {
        considered.insert(box_pos);
        let a = self.poshelper.step(box_pos, dir, 1).unwrap();
        let b = self.poshelper.step(box_pos, dir.opposite(), 1).unwrap();

        self.grid[a].is_wall()
            || self.grid[b].is_wall()
            || !self.grid[a].is_valid() && !self.grid[b].is_valid()
            || considered.contains(&a)
            || considered.contains(&b)
            || self.boxes.contains(&a)
                && self.check_box_blocked_direction(a, considered, dir.rotation())
            || self.boxes.contains(&b)
                && self.check_box_blocked_direction(b, considered, dir.rotation())
    }
}

impl Puzzle {
    fn _create(
        mut grid: Vec<Flags>,
        width: usize,
        height: usize,
        boxes: FxHashSet<usize>,
        targets: FxHashSet<usize>,
        start_pos: usize,
    ) -> Self {
        let puller = Puller::new(&grid, width, height, &targets);

        for pos in puller.find_all_valid_positions() {
            grid[pos] |= Flags::VALID;
        }

        let mut puzzle = Self {
            grid: Rc::new(grid),
            width,
            height,
            boxes,
            player_pos: start_pos,
            targets: Rc::new(targets),
            poshelper: Rc::new(PosHelper::new(width, height)),
            ..Default::default()
        };

        puzzle.update_movable_positions();
        puzzle.move_to_top_left();

        puzzle
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

        let mapper = |it: &std::collections::HashSet<question::Position>| -> FxHashSet<usize> {
            it.iter()
                .map(|p| p.to_usize(width))
                .collect::<FxHashSet<_>>()
        };

        let boxes = mapper(question.boxes());
        let targets = mapper(question.targets());

        Self::_create(grid, width, height, boxes, targets, start)
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
