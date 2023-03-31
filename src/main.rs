extern crate nalgebra as na;
mod encoder_interface;
mod font;
mod front_display;
mod inverse_kinematics;
mod job_dispatcher;
mod serialization;
mod state_server;
use na::Vector3;
use sqlx::sqlite::SqlitePool;

extern "C" {
    pub fn doubler(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn create_shared_od();
    pub fn access_shared_od();
}

use clap::Parser;

use crate::serialization::serde_helpers::{deserialize_vector, serialize_vector, SerdeVector};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
    #[arg(short, long)]
    file: String,
}

fn main() -> () {
    let t2 = Vector3::new(5.0, 5.0, 5.0);
    let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
    //   if let Err(report) = setup_encoder().await {}
    //    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let args = Args::parse();

    println!("{} {}!", args.file, args.config);
    let data = std::fs::read_to_string(args.file).expect("Unable to read file");
    let r = deserialize_vector(&data);

    match r {
        Ok(card) => {
            println!("\n: {?}", card.vectors[0]);
        }
        Err(err) => match err.current_context() {
            ParseConfigError => println!("\nConfig File corrupt"),
        },
    }
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
}
