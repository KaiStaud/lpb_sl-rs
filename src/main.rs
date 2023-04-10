extern crate nalgebra as na;
mod inverse_kinematics;
mod job_dispatcher;
mod serialization;
mod state_server;
use inverse_kinematics::inverse_kinematics::{coordinates_to_steps, simple_ik};
use job_dispatcher::*;
use na::Vector3;
use serialization::db_abstraction::connect_db;
use sqlx::SqlitePool;
use std::sync::mpsc::{self};
extern "C" {
    pub fn doubler(x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn create_shared_od();
    pub fn access_shared_od();
}

use crate::serialization::serde_helpers::deserialize_vector;
use clap::Parser;

enum DataAccess {
    Write,
    Read,
}

fn tracker_thread(rx: std::sync::mpsc::Receiver<Vec<i32>>) {
    let res = rx.recv();
    let x = res.unwrap();
    let mut rm = RingManager::new();

    let tc = TimedCoordinates {
        name: ("Test".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(x),
        rotation: Vector3::from_vec(Vec::from([1, 2, 3])),
    };
    _ = rm.push_to_ring1(vec![tc]);
    _ = rm.auto_refill_rings();

    let r3_content = rm.set_job_from_ring3().unwrap().clone();
    let mut t = Vec::new();
    t.push(r3_content.0.x as f64);
    t.push(r3_content.0.y as f64);
    t.push(r3_content.0.z as f64);
    let q = Vector3::from_vec(t);
    simple_ik(q);
    coordinates_to_steps(Vector3::new(0.0, 0.0, 0.0), q, 90);
}

async fn database_thread(
    rx: std::sync::mpsc::Receiver<(TimedCoordinates, DataAccess)>,
    tx: std::sync::mpsc::Sender<TimedCoordinates>,
) {
    connect_db();
    // TODO: Replace
    let pool = SqlitePool::connect("sqlite:todos.db").await;
    // with connect_db!
    // Wait for key
}

/// Start Control with config and vec-file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: String,
    #[arg(short, long)]
    file: String,
}

fn main() -> () {
    //let t2 = Vector3::new(5.0, 5.0, 5.0);
    //let v = inverse_kinematics::inverse_kinematics::simple_ik(t2);
    //   if let Err(report) = setup_encoder().await {}
    //    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    let args = Args::parse();

    println!("{} {}!", args.file, args.config);
    let data = std::fs::read_to_string(args.file).expect("Unable to read file");
    let r = deserialize_vector(&data);

    match r {
        Ok(card) => {
            println!("\n: {:?}", card.vectors[0]);
        }
        Err(err) => match err.current_context() {
            _parse_config_error => println!("\nConfig File corrupt"),
        },
    }

    let (tx, rx) = mpsc::channel();

    // Spawn tracker thread
    // Spawn CANopen thread

    // Spawn zbus thread

    // enqueuu vector into ringbuffer
    let l = deserialize_vector(&data).unwrap().vectors;
    tx.send(l).unwrap();
    //std::thread::spawn(move ||
    tracker_thread(rx);

    // wait for completion

    //exit
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
