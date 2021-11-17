use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::ops::Index;
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

pub struct QuestionCollection {
    questions: Vec<Question>,
}

impl QuestionCollection {
    pub fn len(&self) -> usize {
        self.questions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl FromStr for QuestionCollection {
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
            let puzzle = Question::from_str(&puzzle_str).map_err(|err| {
                ParseError::new(format!("error on grid {}, {:?}", puzzles.len() + 1, err))
            })?;
            puzzles.push(puzzle);
        }

        Ok(Self { questions: puzzles })
    }
}

impl QuestionCollection {
    pub fn iter(&self) -> impl Iterator<Item = &Question> {
        self.questions.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Question> {
        self.questions.iter_mut()
    }
}

impl IntoIterator for QuestionCollection {
    type Item = Question;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.questions.into_iter()
    }
}

impl Index<usize> for QuestionCollection {
    type Output = Question;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.questions[idx]
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position(usize, usize);

impl Position {
    /// Converts the position into a 1D `usize`. The output position
    /// depends on the `width`.
    pub fn to_usize(&self, width: usize) -> usize {
        self.1 * width + self.0
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Question {
    width: usize,
    height: usize,
    grid: Vec<Vec<Square>>,
    boxes: HashSet<Position>,
    targets: HashSet<Position>,
    start: Position,
}

impl Question {
    pub fn rows(&self) -> impl Iterator<Item = &Vec<Square>> {
        self.grid.iter()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn boxes(&self) -> &HashSet<Position> {
        &self.boxes
    }

    pub fn targets(&self) -> &HashSet<Position> {
        &self.targets
    }

    pub fn start(&self) -> Position {
        self.start
    }
}

// TODO:
// Player on goal square represented by '+'

const WALL_CHAR: char = '#';
const SPACE_CHAR: char = ' ';
const BOX_CHAR: char = '$';
const TARGET_CHAR: char = '.';
const PLACED_CHAR: char = '*';
const START_CHAR: char = '@';

impl FromStr for Question {
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
                let square = match c {
                    WALL_CHAR => {
                        reached_wall = true;
                        Square::Wall
                    }
                    SPACE_CHAR | TARGET_CHAR | BOX_CHAR | START_CHAR | PLACED_CHAR => {
                        if !reached_wall {
                            Square::Wall
                        } else {
                            let pos = Position(x, y);
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
                    }
                    _ => return Err(ParseError::new("invalid text in grid")),
                };
                row_squares.push(square);
            }
            row_squares.resize(width, Square::Wall);
            grid.push(row_squares);
        }

        Ok(Self {
            width,
            height: rows.len(),
            grid,
            boxes,
            targets,
            start: start.ok_or_else(|| ParseError::new("no start position"))?,
        })
    }
}
