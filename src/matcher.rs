use crate::config::Config;
use crate::record::Record;
use crate::options::Options;

// Reads provided input and finds matches.
pub fn matcher(config: &Config) -> Result<Vec<Record>, std::io::Error> {
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
            if config.options.contains(&Options::IgnoreCase) {
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