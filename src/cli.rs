extern crate lpb_lib;
extern crate lpb_unsafe_lib;

use int_enum::IntEnum;

use iceoryx_rs::{PublisherBuilder,SubscriberBuilder};
use iceoryx_rs::Runtime;

use std::error::Error;
use std::thread;
use std::time::Duration;
use lpb_lib::*;
use lpb_unsafe_lib::*;
use clap::{Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
        #[arg(short, long)]
        name: Vec<String>,
        #[arg(value_enum,default_value_t=Mode::TorqueFree)]
        mode: Mode,
        #[arg(value_enum)]
        action: Option<DbAction>,
}
/*
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum,IntEnum)]
enum Mode {
        /// Teach some moves
            TorqueFree = 10,            
        /// Run swiftly
             Fast = 11,
        /// Crawl slowly but steadily
             Slow = 12,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum,IntEnum)]
enum DbAction {
    /// Add
        New = 110,
    /// Unregister
        Remove = 111,
    /// EnQueue
        Queue = 112,
}
*/
fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();

        println!("name: {:?}",cli.name);
       
        let mut t_action = 0;
        match cli.action{
        Some(DbAction::New) => { t_action = DbAction::New.int_value();  println!("Adding to db");}
        Some(DbAction::Remove) =>{t_action = DbAction::Remove.int_value(); println!("Removing from db");}
        Some(DbAction::Queue) =>{t_action = DbAction::Queue.int_value(); println!("Adding to waitlist");}
        None => {println!("");}
        }

       let mut t_mode = 0;
        match cli.mode{
         Mode::Fast => {t_mode=  Mode::Fast.int_value();println!("Hare");}
         Mode::Slow => {t_mode = Mode::Slow.int_value(); println!("Tortoise")}
         Mode::TorqueFree => {t_mode = Mode::TorqueFree.int_value(); println!("Drives w/ Power");}
         //None => {println!("");}             
        }
    
        Runtime::init("publisher");

// Create Publishers:
    let publisher = PublisherBuilder::<Counter>::new("all", "glory", "hypnotoad").create()?;
    
    let mut counter = 0;
        Runtime::init("response_waiter");
        let (subscriber, sample_receive_token) =
        SubscriberBuilder::<Response>::new("subscriber", "response", "hypnotoad").queue_capacity(5).create()?;
        let sample_receiver = subscriber.get_sample_receiver(sample_receive_token);
                loop {

                        if let Some(_sample) = sample_receiver.take() 
                        {
                         //println!("Receiving Response: {}", sample.result);
                        }
                        else
                        {
                         thread::sleep(Duration::from_millis(100));
                        }
                         let mut sample = publisher.loan()?;
                          sample.counter = counter;
                          sample.mode =  t_mode;
                          sample.action = t_action;
                          publisher.publish(sample);
                          //println!("Send praise hypnotoad: {}", counter);
                          counter += 1;
                          thread::sleep(Duration::from_millis(1000));
                          }
}
