extern crate lpb_unsafe_lib;
extern crate nalgebra as na;

mod encoder_interface;
mod font;
mod front_display;
mod inverse_kinematics;
mod job_dispatcher;
mod ll_display;
mod serialization;
mod ssd1306_driver;
mod state_server;
use encoder_interface::setup_encoder;
use lpb_unsafe_lib::*;
use na::Vector3;
use sqlx::sqlite::SqlitePool;
use state_server::*;
use std::{env, path::Path};

use iceoryx_rs::Runtime;
use iceoryx_rs::SubscriberBuilder;

use std::error::Error;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // The `<StateA>` is implied here. We don't need to add type annotations!
    let in_state_a = StateMachine::new("Booting up...".into());

    // This is okay here. But later once we've changed state it won't work anymore.
    in_state_a.some_unrelated_value;
    println!("Starting Value: {}", in_state_a.state.start_value);

    // Transition to the new state. This consumes the old state.
    // Here we need type annotations (since not all StateMachines are linear in their state).
    let in_state_b = StateMachine::<PreOperational>::from(in_state_a);

    // This doesn't work! The value is moved when we transition!
    // in_state_a.some_unrelated_value;
    // Instead, we can use the existing value.
    in_state_b.some_unrelated_value;

    println!("Interm Value: {:?}", in_state_b.state.interm_value);

    // And our final state.
    let in_state_c = StateMachine::<Operational>::from(in_state_b);

    // This doesn't work either! The state doesn't even contain this value.
    // in_state_c.state.start_value;

    println!("Final state: {}", in_state_c.state.final_value);

    let t2 = Vector3::new(5.0, 5.0, 5.0);
    let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
    if let Err(report) = setup_encoder().await {}
    //let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    /*
       println!("Writing some pixels to screen...");
       let mut drv = Driver::new(128, 64, Path::new("/dev/i2c-2"));
       drv.init(Path::new("/dev/gpiochip2"), 3);

       for y in 0..128 {
           drv.draw_pixel(y, 0, true); //Store pixel at (x,y) location
           drv.draw_pixel(0, y, true); //Store pixel at (x,y) location
       }
       drv.refresh(Path::new("/dev/i2c-2"));
    */

    Runtime::init("cli_receiver");

    let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Counter>::new("lpb-sl", "cli", "transceiver")
            .queue_capacity(5)
            .create()?;

    let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);

    loop {
        if sample_receiver.has_data() {
            while let Some(sample) = sample_receiver.take() {
                println!("Receiving: {:?} {:?}", sample.mode, sample.action);
            }
        } else {
            thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
