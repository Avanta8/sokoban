use std::env;
use std::error::Error;

use sokoban::{reader, solve};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let config = reader::Config::new(&args)?;

    let mut puzzles = reader::read(config)?;

    solve::solve_collection(&mut puzzles);

    Ok(())
}
