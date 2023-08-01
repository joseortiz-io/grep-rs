fn main() -> Result<(), &'static str> {
    let args = std::env::args();
    let config = greprs::Config::build(args)?;
    greprs::run(config)?;
    Ok(())
}
