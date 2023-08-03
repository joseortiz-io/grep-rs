use std::collections::HashSet;
use crate::config::Config;
use crate::options::Options;

// Parses the user input and constructs an Options struct and Config struct
// returning the Config.
pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    let mut options = HashSet::new();
    let mut path: Option<String> = None;
    let mut query: Option<String> = None;
    args.next(); // skip program name

    for arg in args {
        // Parse options if '-' is found.
        // TODO: Move this to separate function perhaps?
        if arg.starts_with('-') {
            for c in arg.chars().skip(1) {
                match c {
                    'b' => { let _ = options.insert(Options::ByteOffset); },
                    'c' => { let _ = options.insert(Options::Count); },
                    'i' => { let _ = options.insert(Options::IgnoreCase); },
                    _ => return Err("invalid option"),
                };
            }
        } else if query.is_none() {
            query = Some(arg);
        } else if path.is_none() {
            path = Some(arg);
        } else {
            return Err("Too many arguments.");
        }
    }

    if query.is_none() {
        return Err("No query specified.");
    }

    if path.is_none() {
        return Err("No file path specified.");
    }

    Ok(Config {
        path: path.unwrap(),
        query: query.unwrap(),
        options,
    })
}
