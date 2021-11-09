use std::env;
use std::error::Error;

use sokoban::{reader, solve};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let config = reader::Config::new(&args)?;

    let puzzles = reader::read(config)?;

    // solve::solve_collection(&puzzles);
    solve::solve_puzzle(&puzzles[0]);

    Ok(())
}
