use std::env;
use std::error::Error;

use sokoban::{reader, solver};

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let config = reader::Config::new(&args)?;

    let mut puzzles = reader::read(config)?;

    solver::solve_collection(&mut puzzles);

    Ok(())
}
