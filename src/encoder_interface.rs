use error_stack::{IntoReport, ResultExt};
use std::{fmt,error::Error};
use futures::stream::StreamExt;
use gpio_cdev::{Chip, AsyncLineEventHandle,LineRequestFlags,EventRequestFlags};

#[derive(Debug)]
pub struct GPIOConfigError;

impl fmt::Display for GPIOConfigError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Unable to setup GPIO")
    }
}

impl Error for GPIOConfigError {}


pub async fn setup_encoder() -> error_stack::Result<String, GPIOConfigError>{
    let mut chip = Chip::new("/dev/gpiochip0")
    .into_report()
    .change_context(GPIOConfigError)
    .attach_printable_lazy(|| format!("Error configuring /dev/gpiochip0"))?;


    let line = chip.get_line(19)
    .into_report()
    .change_context(GPIOConfigError)
    .attach_printable_lazy(|| format!("Error configuring /dev/gpiochip0 Pin:19"))?;

    let handle = line.events(
        LineRequestFlags::INPUT,
        EventRequestFlags::BOTH_EDGES,
        "gpioevents",
    ).into_report()
    .change_context(GPIOConfigError)
    .attach_printable_lazy(||format!("Unable to create handle for 0:16!"))?;

    let mut events = AsyncLineEventHandle::new(handle) 
    .into_report()
    .change_context(GPIOConfigError)
    .attach_printable_lazy(|| format!("Unable to attach Async-Fn to Button"))?;

    while let Some(event) = events.next().await {
        let event = event
        .into_report()
        .change_context(GPIOConfigError);
        println!("GPIO Event: {:?}", event);
    }

    Ok("GPIO 0:16 created!".to_string())
}