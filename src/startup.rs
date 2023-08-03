use crate::options::Options;
use crate::parser::parse;
use crate::matcher::matcher;

// Run the logic. This is called in entry point.
pub fn run(args: impl Iterator<Item = String>) -> Result<(), &'static str> {
    let config = match parse(args) {
        Ok(c) => c,
        Err(_e) => return Err("Could not parse input"),
    };
    let matches = match matcher(&config) {
        Ok(m) => m,
        Err(_e) => return Err("Could not open file"),
    };
    if config.options.contains(&Options::Count) {
        println!("{}", matches.len());
        return Ok(());
    }
    if config.options.contains(&Options::ByteOffset) {
        matches
            .iter()
            .for_each(|line| println!("{}: {}", line.line_number, line.data));
        return Ok(());
    }
    if config.options.contains(&Options::IgnoreCase) {
        matches.iter().for_each(|line| println!("{}", line.data));
        return Ok(());
    }

    Ok(())
}