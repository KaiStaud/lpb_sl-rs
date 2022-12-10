use crate::font;
use error_stack::{IntoReport, ResultExt};
use std::{error::Error, fmt};
#[derive(Debug)]

pub struct FrontDisplayError;

impl fmt::Display for FrontDisplayError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Could not access I2C-Device")
    }
}

//impl Context for ParseConfigError {}
impl Error for FrontDisplayError {}

struct FrontDisplay {}

impl FrontDisplay {
    pub fn load_font() {}

    pub fn println() {}

    fn print_char(c: char) {}
}
