#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Options {
    // -b, --byte-offset
    // The offset in bytes of a matched pattern is displayed in front of the respective matched line.
    ByteOffset,
    // -c, --count
    // Only a count of selected lines is written to standard output.
    Count,
    // -i, --ignore-case
    // Perform case insensitive matching.  By default, grep is case sensitive.
    IgnoreCase
}