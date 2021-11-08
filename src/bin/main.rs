use std::env;

use sokoban::reader::Config;

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();

    let config = Config::new(&args)?;
    Ok(())
}
