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

    /// Returns the opposite direction. Eg. North <-> South, East <-> West.
    pub fn opposite(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }

    /// Returns the direction rotated 90 degrees clockwise.
    pub fn rotation(&self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

#[derive(Default, Clone)]
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

#[derive(Debug, Default)]
pub struct PosHelper {
    width: usize,
    height: usize,
}

impl PosHelper {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

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
