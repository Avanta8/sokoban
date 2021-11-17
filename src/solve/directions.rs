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
