extern crate nalgebra as na;
mod cli;
mod encoder_interface;
mod front_display;
mod inverse_kinematics;
mod job_dispatcher;
mod serialization;
mod ssd1306_driver;
mod state_server;

use encoder_interface::setup_encoder;
use front_display::lcd_setup;
use na::Vector3;
use sqlx::sqlite::SqlitePool;
use ssd1306_driver::Driver;
use state_server::*;
use std::{env, path::Path};
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
    lcd_setup();
    if let Err(report) = setup_encoder().await {}
    //let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    println!("Writing some pixels to screen...");
    let mut drv = Driver::new(128, 64, Path::new("/dev/i2c-2"));
    drv.init(Path::new("/dev/gpiochip2"), 3);

    for y in 0..128 {
        drv.draw_pixel(y, 0, true); //Store pixel at (x,y) location
        drv.draw_pixel(0, y, true); //Store pixel at (x,y) location
    }
    drv.refresh(Path::new("/dev/i2c-2")); /*
                                              match args.cmd {
                                                  Some(Command::Add { description }) => {
                                                      println!("Adding new todo with description '{}'", &description);
                                                      let todo_id = add_todo(&pool, "Test".to_string(),"1".to_string(),"2".to_string()).await?;
                                                      println!("Added new todo with id {}", todo_id);
                                                  }

                                                  None => {
                                                      println!("Printing list of all todos");
                                                      list_todos(&pool).await?;
                                                  }
                                              }
                                          */
    Ok(())
}
