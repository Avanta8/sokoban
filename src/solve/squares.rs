use bitflags::bitflags;
use std::fmt;

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
    /// a space but there is not a box above it. (It can be a target.)
    pub fn is_walkable(&self) -> bool {
        *self | Self::TARGET | Self::PLAYER == Self::WALKABLE
    }
    pub fn is_wall(&self) -> bool {
        *self == Self::WALL
    }
    pub fn is_player(&self) -> bool {
        self.contains(Self::PLAYER)
    }
    pub fn is_box(&self) -> bool {
        self.contains(Self::BOX)
    }
    pub fn is_target(&self) -> bool {
        self.contains(Self::TARGET)
    }
    pub fn is_placed(&self) -> bool {
        self.contains(Self::TARGET | Self::BOX)
    }
    pub fn is_space(&self) -> bool {
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
