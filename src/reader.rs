use std::error::Error;
use std::fs;
use std::str::FromStr;

use crate::question::QuestionCollection;

pub struct Config<'a> {
    pub filename: &'a str,
    pub question_number: Option<usize>,
}

impl<'a> Config<'a> {
    // pub fn new(args: &'a [String]) -> Result<Config, String> {
    pub fn new(args: &'a [String]) -> Result<Config, Box<dyn Error>> {
        if args.len() <= 1 || args.len() > 3 {
            return Err(format!(
                "Incorrect number of args: got {}, must have 2 or 3.",
                args.len()
            )
            .into());
        }

        let filename = &args[1];
        let question_number = args.get(2).map(|x| x.parse::<usize>()).transpose()?;

        Ok(Self {
            filename,
            question_number,
        })
    }
}

pub fn read(config: &Config) -> Result<QuestionCollection, Box<dyn Error>> {
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
        Config {
            filename,
            question_number: None,
        }
    }

    pub fn create_collection(filename: &str) -> QuestionCollection {
        read(&create_config(filename)).unwrap()
    }

    #[test]
    fn test() {}
}
