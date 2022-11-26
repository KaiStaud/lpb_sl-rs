extern crate int_enum;
extern crate iceoryx_rs;
extern crate clap;

use int_enum::IntEnum;
use clap::ValueEnum;
use iceoryx_rs::marker::ShmSend;

#[repr(u32)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,ValueEnum,IntEnum)]
pub enum DbAction {
    /// Add
    New = 110,
    /// Unregister
    Remove = 111,
    /// EnQueue
    Queue = 112,
}

#[repr(u32)]
#[derive(Debug,Copy,Clone,PartialEq,Eq,PartialOrd,Ord,ValueEnum,IntEnum)]
pub enum Mode {
    /// Teach some moves
    TorqueFree = 10,
    /// Run switfly
    Fast = 11,
    /// Crawl slowly but steadily
    Slow = 12,
}

