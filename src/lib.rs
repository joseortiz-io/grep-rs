// struct to hold user input
pub struct Config {
    pub path: String,
    pub query: String,
    pub options: Options,
}

impl Config {
    // Parses the user input and constructs an Options struct and Config struct
    // returning the Config.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut options = Options::default();
        let mut path: Option<String> = None;
        let mut query: Option<String> = None;
        args.next(); // skip program name

        for arg in args {
            // Parse options if '-' is found.
            // TODO: Move this to separate function perhaps?
            if arg.starts_with('-') {
                for c in arg.chars().skip(1) {
                    match c {
                        'b' => options.b = true,
                        'c' => options.c = true,
                        'i' => options.i = true,
                        _ => return Err("invalid option"),
                    }
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
}

// Holds option flags provided by user.
#[derive(Debug, Default, PartialEq)]
pub struct Options {
    // -b, --byte-offset
    // The offset in bytes of a matched pattern is displayed in front of the respective matched line.
    pub b: bool,
    // -c, --count
    // Only a count of selected lines is written to standard output.
    pub c: bool,
    // -i, --ignore-case
    // Perform case insensitive matching.  By default, grep is case sensitive.
    pub i: bool,
}

#[derive(Debug)]
// Stores matching records from input provided by user.
struct Record {
    line_number: usize,
    data: String,
}

// Run the logic. This is called in entry point.
pub fn run(config: Config) -> Result<(), &'static str> {
    let matches = match get_matches(&config) {
        Ok(m) => m,
        Err(_e) => return Err("Could not open file"),
    };
    if config.options.c {
        println!("{}", matches.len());
        return Ok(());
    }
    if config.options.b {
        matches
            .iter()
            .for_each(|line| println!("{}: {}", line.line_number, line.data));
        return Ok(());
    }
    if config.options.i {
        matches.iter().for_each(|line| println!("{}", line.data));
        return Ok(());
    }

    Ok(())
}

// Reads provided input and finds matches.
// Need to break this function up and refactor.
fn get_matches(config: &Config) -> Result<Vec<Record>, std::io::Error> {
    use std::io::BufRead;
    let mut matches: Vec<Record> = Vec::new();
    let fp = match std::fs::File::open(config.path.clone()) {
        Ok(fp) => fp,
        Err(e) => return Err(e),
    };
    let reader = std::io::BufReader::new(fp);
    for (i, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let mut processed_line = line.clone();
            let mut processed_query = (config.query).clone();
            if config.options.i {
                processed_line = processed_line.to_uppercase();
                processed_query = processed_query.to_uppercase();
            }
            if processed_line.contains(&processed_query) {
                matches.push(Record {
                    line_number: i + 1,
                    data: line,
                });
            }
        }
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_query() {
        let args = vec!["src".to_owned()].into_iter();
        match super::Config::build(args) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "No query specified."),
        }
    }

    #[test]
    fn no_query_with_flag() {
        let args = vec!["src".to_owned(), "-b".to_owned()].into_iter();
        match super::Config::build(args) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "No query specified."),
        }
    }

    #[test]
    fn no_path() {
        let args = vec!["src".to_owned(), "query".to_owned()].into_iter();
        match super::Config::build(args) {
            Ok(_) => assert!(false),
            Err(e) => assert_eq!(e, "No file path specified."),
        }
    }

    #[test]
    fn with_query_path() {
        let args = vec!["src".to_owned(), "query".to_owned(), "path".to_owned()].into_iter();
        match super::Config::build(args) {
            Ok(config) => {
                assert_eq!(config.query, "query");
                assert_eq!(config.path, "path");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn with_query_path_b() {
        let args = vec![
            "src".to_owned(),
            "-b".to_owned(),
            "query".to_owned(),
            "path".to_owned(),
        ]
        .into_iter();
        match super::Config::build(args) {
            Ok(config) => {
                assert_eq!(config.query, "query");
                assert_eq!(config.path, "path");
                assert_eq!(config.options.b, true);
                assert_eq!(config.options.c, false);
                assert_eq!(config.options.i, false);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn with_query_path_c() {
        let args = vec![
            "src".to_owned(),
            "-c".to_owned(),
            "query".to_owned(),
            "path".to_owned(),
        ]
        .into_iter();
        match super::Config::build(args) {
            Ok(config) => {
                assert_eq!(config.query, "query");
                assert_eq!(config.path, "path");
                assert_eq!(config.options.b, false);
                assert_eq!(config.options.c, true);
                assert_eq!(config.options.i, false);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn with_query_path_i() {
        let args = vec![
            "src".to_owned(),
            "-i".to_owned(),
            "query".to_owned(),
            "path".to_owned(),
        ]
        .into_iter();
        match super::Config::build(args) {
            Ok(config) => {
                assert_eq!(config.query, "query");
                assert_eq!(config.path, "path");
                assert_eq!(config.options.b, false);
                assert_eq!(config.options.c, false);
                assert_eq!(config.options.i, true);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn with_query_path_bci() {
        let args = vec![
            "src".to_owned(),
            "-b".to_owned(),
            "-c".to_owned(),
            "-i".to_owned(),
            "query".to_owned(),
            "path".to_owned(),
        ]
        .into_iter();
        match super::Config::build(args) {
            Ok(config) => {
                assert_eq!(config.query, "query");
                assert_eq!(config.path, "path");
                assert_eq!(config.options.b, true);
                assert_eq!(config.options.c, true);
                assert_eq!(config.options.i, true);
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_run() {
        let args = vec![
            "src".to_owned(),
            "-b".to_owned(),
            "-c".to_owned(),
            "-i".to_owned(),
            "name".to_owned(),
            "Cargo.toml".to_owned(),
        ]
        .into_iter();
        let config = super::Config::build(args).unwrap();
        match super::run(config) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_run_without_c() {
        let args = vec![
            "src".to_owned(),
            "-b".to_owned(),
            "-i".to_owned(),
            "nAme".to_owned(),
            "Cargo.toml".to_owned(),
        ]
        .into_iter();
        let config = super::Config::build(args).unwrap();
        match super::run(config) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
