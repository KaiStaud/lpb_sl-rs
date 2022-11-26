extern crate iceoryx_rs;
use iceoryx_rs::marker::ShmSend;

#[repr(C)]
#[derive(Default)]
pub struct Counter {
    pub counter: u32,
    pub mode: u32,
    pub action: u32,
}

#[repr(C)]
#[derive(Default)]
pub struct Response {
    pub result: u32,
}

unsafe impl ShmSend for Response {}
unsafe impl ShmSend for Counter {}
