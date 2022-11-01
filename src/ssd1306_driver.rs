/*
Self-build driver for SSD1306
Implements traits from interface front_display

Display content is written to framebuffer, which update-call sets pixels.

todo: configurable reset-pin
todo: expose interface to serial interface i2c / spi
todo: init process
todo: low-level pixel write
todo: custom fonts
optional: pub as self-contained library

*/
// Based on tutorial:https://www.instructables.com/Getting-Started-With-OLED-Displays/
use gpio_cdev::{Chip, LineRequestFlags};
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

pub struct ssd1306_driver {
    dev: LinuxI2CDevice,
    frame_buffer: [u8; 1023],
}

impl ssd1306_driver {
    pub fn new(dev: u8, address: u8) -> ssd1306_driver {
        Ok(ssd1306_driver {
            dev: LinuxI2CDevice::new(format!("/dev/i2c-{}", bus), address)?,
        })
    }

    pub fn init(&self,gpiochip:i32,gpioline:i32) {
        let mut chip = Chip::new(self.)?;

        let _handle =
            chip.get_line(self.gpioline)?
                .request(LineRequestFlags::OUTPUT, 1, "driveoutput")?;
        // Toggle RST to initialize display:
        // Wait 100ms

        // Set Low

        // Wait 100ms

        // Set High
    }

    pub fn draw_pixel(&self, x: i32, y: i32) -> bool {}

    pub fn refresh(&self) {}
}
