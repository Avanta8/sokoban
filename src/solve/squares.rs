use bitflags::bitflags;
use std::fmt;

bitflags! {
    pub struct Flags:u8 {
        const WALL   = 0b001;
        const SPACE  = 0b010;
        const VALID  = 0b110;
    }
}

impl Flags {
    pub fn is_wall(&self) -> bool {
        *self == Self::WALL
    }

    pub fn is_space(&self) -> bool {
        self.contains(Self::SPACE)
    }

    pub fn is_valid(&self) -> bool {
        *self == Self::VALID
    }

    pub fn as_str(&self) -> &'static str {
        if self.is_wall() {
            "#"
        } else if self.is_space() {
            " "
        } else {
            unreachable!("Impossible square flag: {:?}", self);
        }
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
