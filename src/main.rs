fn main() -> Result<(), &'static str> {
    let args = std::env::args();
    greprs::startup::run(args)?;
    Ok(())
}
