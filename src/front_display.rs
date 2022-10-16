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

use lcd_pcf8574::{Pcf8574, ErrorHandling};

pub fn lcd_setup() -> Result<(), Box<dyn std::error::Error>> {
    let mut dev = Pcf8574::new(1, 0x27)?;
    dev.on_error(ErrorHandling::Panic);

    /*  .into_report()
    .change_context(FrontDisplayError)
    .attach_printable_lazy(||format("Display@/dev/i2c-1:0x27 not responding!"))?;
   */ 
    let mut display = lcd::Display::new(dev);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff);

    display.clear();
    display.home();
    display.print("Hello, World!");
    display.position(0, 1);
    display.print("This is line two");


    Ok(())
}