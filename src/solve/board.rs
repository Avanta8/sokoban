use rustc_hash::FxHashSet;

use super::directions::Dir;
use super::puller::Puller;
use super::squares::Flags;

#[derive(Debug, Default, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Flags>,
    pub targets: FxHashSet<usize>,
}

impl Board {
    pub fn new(
        width: usize,
        height: usize,
        mut grid: Vec<Flags>,
        targets: FxHashSet<usize>,
    ) -> Self {
        let valid = Puller::new(Self {
            width,
            height,
            grid: grid.clone(),
            targets: targets.clone(),
        })
        .find_all_valid_positions();

        for pos in valid {
            grid[pos] |= Flags::VALID;
        }

        Self {
            height,
            width,
            grid,
            targets,
        }
    }

    /// Returns the grid as a 2d vector of strings corresponding to each flag.
    pub fn to_2d_grid_str(&self, player_pos: usize, boxes: &FxHashSet<usize>) -> Vec<Vec<&str>> {
        let mut grid = self
            .grid
            .clone()
            .iter()
            .map(|f| f.as_str())
            .collect::<Vec<_>>();

        for &pos in boxes.iter() {
            grid[pos] = "$"
        }
        for &pos in self.targets.iter() {
            grid[pos] = if boxes.contains(&pos) { "*" } else { "." }
        }
        grid[player_pos] = "@";

        grid.chunks_exact(self.width)
            .map(|row| row.to_vec())
            .collect::<Vec<_>>()
    }
}

impl Board {
    /// Retuns the position that you would end up on after moving `steps` in `dir` direction startin from `pos`.
    /// Returns `None` if the resulting position would be out of bounds.
    pub fn step(&self, mut pos: usize, dir: Dir, steps: usize) -> Option<usize> {
        let f = match dir {
            Dir::North => Self::north,
            Dir::South => Self::south,
            Dir::East => Self::east,
            Dir::West => Self::west,
        };
        for _ in 0..steps {
            match f(self, pos) {
                Some(p) => pos = p,
                None => return None,
            }
        }
        Some(pos)
    }
    pub fn north(&self, pos: usize) -> Option<usize> {
        pos.checked_sub(self.width)
    }
    pub fn south(&self, pos: usize) -> Option<usize> {
        if pos / self.width + 1 == self.height {
            return None;
        }
        Some(pos + self.width)
    }
    pub fn east(&self, pos: usize) -> Option<usize> {
        if (pos + 1) % self.width == 0 {
            return None;
        }
        Some(pos + 1)
    }
    pub fn west(&self, pos: usize) -> Option<usize> {
        if pos % self.width == 0 {
            return None;
        }
        Some(pos - 1)
    }

    /// Returns an iterator of all the bordering positions around `pos` that are
    /// not out of bounds. (Doesn't check what the square acutally is. Just checks that
    /// it isn't out of bounds.)
    pub fn borders(&self, pos: usize) -> impl Iterator<Item = usize> {
        [
            self.north(pos),
            self.east(pos),
            self.south(pos),
            self.west(pos),
        ]
        .into_iter()
        .flatten()
    }

    /// Returns all the positions around `pos` that are not out of bounds as an iterator of
    /// tuples `(direction moved, resultant position)`.
    pub fn borders_with_dirs(&self, pos: usize) -> impl Iterator<Item = (Dir, usize)> + '_ {
        Dir::iter().filter_map(move |dir| self.step(pos, dir, 1).map(|new_pos| (dir, new_pos)))
    }
}

impl Board {
    pub fn square_at(&self, pos: usize) -> Flags {
        self.grid[pos]
    }
}
