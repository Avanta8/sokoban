use std::error::Error;
use std::fs;
use std::str::FromStr;

use crate::question::QuestionCollection;

pub struct Config<'a> {
    filename: &'a String,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Result<Config, String> {
        if args.len() != 2 {
            Err(format!(
                "Incorrect number of args: got {}, expected 2",
                args.len()
            ))
        } else {
            Ok(Self { filename: &args[1] })
        }
    }
}

pub fn read(config: Config) -> Result<QuestionCollection, Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let puzzles = QuestionCollection::from_str(&contents)?;

    // todo!()
    Ok(puzzles)
}

#[cfg(test)]
mod test_collection {
    #[test]
    fn test() {}
}
