use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub enum ErrorKind {}

#[derive(Debug)]
pub enum ParseError {
    Text(String),
    Kind(ErrorKind),
}

impl ParseError {
    fn new<T: AsRef<str>>(s: T) -> Self {
        Self::Text(s.as_ref().to_string())
    }
}

impl Error for ParseError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     None
    // }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Something went wrong... ParseError... In DISPLAY (change later)"
        )
    }
}

#[derive(Clone)]
pub enum Square {
    Space,
    Wall,
}

// impl TryFrom<char> for Square {
//     type Error = ParseError;

//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         match value {
//             '#' => Ok(Self::Wall),
//             ' ' | '$' | '.' => Ok(Self::Space),
//             _ => Err(ParseError::new("failed converting square")),
//         }
//     }
// }

pub struct PuzzleCollection {
    puzzles: Vec<Puzzle>,
}

impl FromStr for PuzzleCollection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut puzzles = vec![];
        while lines.peek().is_some() {
            let puzzle_str = lines
                .by_ref()
                .take_while(|line| !line.chars().all(char::is_whitespace))
                .collect::<Vec<_>>()
                .join("\n");

            // May need to do something for cases if there are multiple blank lines separating puzzles
            let puzzle = Puzzle::from_str(&puzzle_str).map_err(|err| {
                ParseError::new(format!("error on grid {}, {:?}", puzzles.len() + 1, err))
            })?;
            puzzles.push(puzzle);
        }

        Ok(Self { puzzles })
    }
}

impl PuzzleCollection {
    pub fn iter(&self) -> impl Iterator<Item = &Puzzle> {
        self.puzzles.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Puzzle> {
        self.puzzles.iter_mut()
    }
}

impl IntoIterator for PuzzleCollection {
    type Item = Puzzle;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.puzzles.into_iter()
    }
}

type Position = (usize, usize);

#[allow(dead_code)]
pub struct Puzzle {
    grid: Vec<Vec<Square>>,
    boxes: HashSet<Position>,
    targets: HashSet<Position>,
    pos: Position,
}

const WALL_CHAR: char = '#';
const SPACE_CHAR: char = ' ';
const BOX_CHAR: char = '$';
const TARGET_CHAR: char = '.';
const PLACED_CHAR: char = '*';
const START_CHAR: char = '@';

impl FromStr for Puzzle {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|row| row.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = rows
            .iter()
            .map(|row| row.len())
            .max()
            .ok_or_else(|| ParseError::new("Puzzle was empty"))?;

        let mut boxes = HashSet::new();
        let mut targets = HashSet::new();
        let mut start = None;

        let mut grid = vec![];
        for (y, row) in rows.iter().enumerate() {
            let mut reached_wall = false;
            let mut row_squares = Vec::with_capacity(width);
            for (x, &c) in row.iter().enumerate() {
                let square = if reached_wall {
                    match c {
                        WALL_CHAR => Square::Wall,
                        SPACE_CHAR | TARGET_CHAR | BOX_CHAR | START_CHAR | PLACED_CHAR => {
                            let pos = (x, y);
                            match c {
                                TARGET_CHAR => {
                                    targets.insert(pos);
                                }
                                BOX_CHAR => {
                                    boxes.insert(pos);
                                }
                                PLACED_CHAR => {
                                    targets.insert(pos);
                                    boxes.insert(pos);
                                }
                                START_CHAR => {
                                    match start {
                                        Some(_) => {
                                            return Err(ParseError::new("multiple start positions"))
                                        }
                                        None => start = Some(pos),
                                    };
                                }
                                SPACE_CHAR => (),
                                _ => unreachable!(),
                            };
                            Square::Space
                        }
                        _ => return Err(ParseError::new("invalid text in grid")),
                    }
                } else {
                    reached_wall = true;
                    Square::Wall
                };
                row_squares.push(square);
            }
            row_squares.resize(width, Square::Wall);
            grid.push(row_squares);
        }

        Ok(Self {
            grid,
            boxes,
            targets,
            pos: start.ok_or_else(|| ParseError::new("no start position"))?,
        })
    }
}
