use crate::{parse, dump};

#[test]
fn parse_reverses_dump() -> Result<(), ()> {
    let message = "Hello world ☺. 1+1=2; 2+2=4 \\0/";

    assert_eq!(parse::parse_str(&dump::dump_str(message))?, message);
    Ok(())
}
