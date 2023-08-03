use greprs::options::Options;

#[test]
fn no_query() {
    let args = vec!["src".to_owned()].into_iter();
    match greprs::parser::parse(args) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "No query specified."),
    }
}

#[test]
fn no_query_with_flag() {
    let args = vec!["src".to_owned(), "-b".to_owned()].into_iter();
    match greprs::parser::parse(args) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "No query specified."),
    }
}

#[test]
fn no_path() {
    let args = vec!["src".to_owned(), "query".to_owned()].into_iter();
    match greprs::parser::parse(args) {
        Ok(_) => assert!(false),
        Err(e) => assert_eq!(e, "No file path specified."),
    }
}

#[test]
fn with_query_path() {
    let args = vec!["src".to_owned(), "query".to_owned(), "path".to_owned()].into_iter();
    match greprs::parser::parse(args) {
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
    match greprs::parser::parse(args) {
        Ok(config) => {
            assert_eq!(config.query, "query");
            assert_eq!(config.path, "path");
            assert_eq!(config.options.contains(&Options::ByteOffset), true);
            assert_eq!(config.options.contains(&Options::Count), false);
            assert_eq!(config.options.contains(&Options::IgnoreCase), false);
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
    match greprs::parser::parse(args) {
        Ok(config) => {
            assert_eq!(config.query, "query");
            assert_eq!(config.path, "path");
            assert_eq!(config.options.contains(&Options::ByteOffset), false);
            assert_eq!(config.options.contains(&Options::Count), true);
            assert_eq!(config.options.contains(&Options::IgnoreCase), false);
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
    match greprs::parser::parse(args) {
        Ok(config) => {
            assert_eq!(config.query, "query");
            assert_eq!(config.path, "path");
            assert_eq!(config.options.contains(&Options::ByteOffset), false);
            assert_eq!(config.options.contains(&Options::Count), false);
            assert_eq!(config.options.contains(&Options::IgnoreCase), true);
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
    match greprs::parser::parse(args) {
        Ok(config) => {
            assert_eq!(config.query, "query");
            assert_eq!(config.path, "path");
            assert_eq!(config.options.contains(&Options::ByteOffset), true);
            assert_eq!(config.options.contains(&Options::Count), true);
            assert_eq!(config.options.contains(&Options::IgnoreCase), true);
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
    match greprs::startup::run(args) {
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
    match greprs::startup::run(args) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}