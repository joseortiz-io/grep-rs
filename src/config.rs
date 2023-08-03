use std::collections::HashSet;
use crate::options::Options;

// struct to hold user input
pub struct Config {
    pub path: String,
    pub query: String,
    pub options: HashSet<Options>,
}