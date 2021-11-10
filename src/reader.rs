use std::error::Error;
use std::fs;
use std::str::FromStr;

use crate::question::QuestionCollection;

pub struct Config<'a> {
    filename: &'a str,
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
    let filepath = format!("puzzles/{}", config.filename);
    let contents = fs::read_to_string(filepath)?;
    let puzzles = QuestionCollection::from_str(&contents)?;

    // todo!()
    Ok(puzzles)
}

#[cfg(test)]
pub mod test_config {

    use super::*;

    pub fn create_config(filename: &str) -> Config {
        Config { filename }
    }

    pub fn create_collection(filename: &str) -> QuestionCollection {
        read(create_config(filename)).unwrap()
    }

    #[test]
    fn test() {}
}
