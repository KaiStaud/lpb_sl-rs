extern crate i2cdev;
use gpio_cdev::{Chip, LineRequestFlags};

use crate::ll_display::DotDisplay;
use error_stack::{IntoReport, ResultExt};
use i2cdev::core::{I2CMessage, I2CTransfer};
use i2cdev::linux::{LinuxI2CBus, LinuxI2CMessage};
use std::{error::Error, fmt, path::Path, thread::sleep, time::Duration};
#[derive(Debug)]
pub struct Ssd1306DriverError;

impl fmt::Display for Ssd1306DriverError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("I2C Device shows unusual behaviour")
    }
}

impl Error for Ssd1306DriverError {}

pub struct Driver {
    width: i32,
    height: i32,
    frame_buffer: [u8; 1025],
    bus: LinuxI2CBus,
}

impl Driver {
    pub fn new(width: i32, height: i32, path: &Path) -> Driver {
        let fb: [u8; 1025] = [0; 1025];

        let mut bus = LinuxI2CBus::new(path).unwrap();

        Driver {
            frame_buffer: (fb),
            width: (width),
            height: (height),
            bus: (bus),
        }
    }

    pub fn init(
        &mut self,
        gpiochip: &Path,
        gpioline: u32,
    ) -> error_stack::Result<(), Ssd1306DriverError> {
        let mut chip = Chip::new(gpiochip)
            .into_report()
            .change_context(Ssd1306DriverError)
            .attach_printable_lazy(|| {
                format!(
                    "Failed to intialize GPIO {}:{}",
                    gpiochip.display(),
                    gpioline
                )
            })?;

        let handle = chip
            .get_line(gpioline)
            .into_report()
            .change_context(Ssd1306DriverError)
            .attach_printable_lazy(|| format!("Unable to get handle to line {}!", gpioline))?
            .request(LineRequestFlags::OUTPUT, 0, "driveoutput")
            .into_report()
            .change_context(Ssd1306DriverError)
            .attach_printable_lazy(|| format!("Setting State of Reset-Pin failed!"))?;

        let mut chip = Chip::new(Path::new("/dev/gpiochip2")).expect("this should work");

        let handle = chip
            .get_line(3)
            .expect("huh")
            .request(LineRequestFlags::OUTPUT, 0, "driveoutput")
            .unwrap();

        // Toggle RST to initialize display:
        handle.set_value(0).expect("Error while initial set of pin");
        sleep(Duration::from_millis(10));
        handle.set_value(1).expect("Error while initial set of pin");
        sleep(Duration::from_millis(2));
        handle.set_value(0).expect("Error while resetting pin");
        sleep(Duration::from_millis(10));
        handle
            .set_value(1)
            .expect("Error while setting final pin state");
        sleep(Duration::from_millis(10));

        let mut msgs = [LinuxI2CMessage::write(&[
            0x00, 0xAE, 0xD5, 0x80, 0xA8, 0x3F, 0xD3, 0x00, 0x40, 0x8D, 0x14, 0xA1, 0xC0, 0xDA,
            0x12, 0x81, 0xcf, 0xd9, 0xf1, 0xdb, 0x40, 0xa4, 0xa6, 0x20, 0x00, 0xaf,
        ])
        .with_address(0x3d)];
        // Send the messages to the kernel to process
        match self.bus.transfer(&mut msgs) {
            Ok(rc) => println!("Successful transfer call: {} messages processed", rc),
            Err(_e) => {
                println!("Error reading/writing {}", _e);
            }
        }
        std::result::Result::Ok(())
    }
}

impl DotDisplay for Driver {
    fn draw_pixel(&mut self, x: i32, y: i32, _on: bool) {
        if (x < 0) || (x >= self.width) || (y < 0) || (y >= self.height) {
        } else {
            self.frame_buffer[(x + (y / 8) * self.width) as usize] |= 1 << (y % 8);
            //Store pixel in array
        }
    }

    fn print_fb(&mut self) {}
    fn refresh(&mut self, _path: &std::path::Path) {
        let mut values: [u8; 4] = [0; 4];
        values[0] = 0x00; //Command stream
        values[1] = 0x00; //Set lower column start address for page addressing mode
        values[2] = 0x10; //Set higher column start address for page addressing mode
        values[3] = 0x40; //Set display start line

        let mut msgs = [LinuxI2CMessage::write(&values).with_address(0x3d)];

        match self.bus.transfer(&mut msgs) {
            Ok(rc) => (),
            Err(_e) => {
                println!("Error reading/writing {}", _e);
                return;
            }
        }
        let mut chunk = 0;
        let mut pixels: [u8; 17] = [0; 17];
        pixels[0] = 0x40;
        // Bytes 5-16 are the framebuffer-data
        let mut q = 0;
        while q < 1024 {
            chunk += 1;

            // Write each 64 pixel rows seperately:
            // Each byte represents eight horizontally aligned pixels: 8*16 = 64
            for _w in 0..16 {
                q += 1;
                pixels[((q % 16) + 1) as usize] = self.frame_buffer[q as usize];
            }
            //println!("Chunk Nr.:{}={:?}",chunk,pixels);
            // increment only above:
            q += 1;
            q -= 1;

            // Perform a block-transfer on i2c bus:
            let mut data = [LinuxI2CMessage::write(&pixels).with_address(0x3d)];

            match self.bus.transfer(&mut data) {
                Ok(rc) => (),
                Err(_e) => {
                    println!("Error reading/writing {}", _e);
                    return;
                }
            }
        }
    }
}
