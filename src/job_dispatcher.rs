use crate::serialization::serialization::TimedCoordinates;
use error_stack::{IntoReport, ResultExt};
use na::Vector3;
use ringbuffer::{AllocRingBuffer, RingBuffer, RingBufferExt, RingBufferWrite};
use std::{error::Error, fmt};
const RING1_SIZE: i32 = 10;
const RING2_SIZE: i32 = 10;
const RING3_SIZE: i32 = 10;

#[derive(Debug)]
pub struct DataRingError;

impl fmt::Display for DataRingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Could not read/write ringbuffer")
    }
}

//impl Context for DataRingError {}
impl Error for DataRingError {}
struct SubRing<T> {
    generic_ring: AllocRingBuffer<T>,
    readpointer: i32,
    writepointer: i32,
}

struct RingManager {
    r1: AllocRingBuffer<Vec<TimedCoordinates>>,
    r2: AllocRingBuffer<TimedCoordinates>,
    r3: AllocRingBuffer<(Vec<i32>, Vec<i32>)>,
    r1_ri: i32,
    r1_wi: i32,
    r2_ri: i32,
    r2_wi: i32,
    r3_ri: i32,
    r3_wi: i32,
}
impl RingManager {
    fn new() -> RingManager {
        let tr1: AllocRingBuffer<Vec<TimedCoordinates>> = AllocRingBuffer::with_capacity(4);
        let tr2: AllocRingBuffer<TimedCoordinates> = AllocRingBuffer::with_capacity(4);
        let tr3: AllocRingBuffer<(Vec<i32>, Vec<i32>)> = AllocRingBuffer::with_capacity(4);

        RingManager {
            r1: (tr1),
            r2: (tr2),
            r3: (tr3),
            r1_ri: (0),
            r1_wi: (0),
            r2_ri: (0),
            r2_wi: (0),
            r3_ri: (0),
            r3_wi: (0),
        }
    }

    fn get_remaining_slots(&self) -> i32 {
        0
    }

    fn get_element_at_ri<T>(&self, sr: SubRing<T>) -> error_stack::Result<T, DataRingError> {
        //Ok(sr.generic_ring.get(1))
        Ok(4)
    }

    fn push_to_ring1(&mut self, i: Vec<TimedCoordinates>) -> i32 {
        self.r1.push(i);
        self.r1_wi += 1;
        let remaining_slot = self.r1.peek().unwrap();
        RING1_SIZE - self.r1_wi
    }

    fn get_from_ring3(&mut self) -> Option<(Vector3<i32>, Vector3<i32>)> {
        //fn get_from_ring3(&mut self)->Option<(Vec<i32>,Vec<i32>)>{
        let tt = self.r3.get(-1).unwrap().clone();
        let t = self.r3.get(-1);
        let mut x = Vec::new();
        let mut y = Vec::new();
        match t {
            Some(v1) => {
                // Automatically pull until all slots are filled:
                for i in 1..RING1_SIZE {
                    let t2 = self.r2.get(-1);
                    // Unwrap to get vals:
                    let val = t2.unwrap();
                    // Construct tuple
                    x = val.vector.clone();
                    y = val.rotation.clone();
                    /*                 let mut tv = Vec::new();
                    let tr = Vec::new();
                    for i in val.vector {
                        tv.push(i);
                    }
                    for i in val.rotation  {
                        tr.push(i);
                    } */
                    // Push tuple into r3
                    self.r3.push((x, y));
                }

                // Automatically pull until r2 slots are filled up:
                let u = self.r1.get(-1).cloned();
                // Construct TimedCoordinates
                let tc = u.unwrap();
                // Push into r2
                for u2 in tc {
                    self.r2.push(u2);
                }

                // Return the last slot of r3
                let xa = Vector3::from_vec(tt.0);
                let xb = Vector3::from_vec(tt.1);
                Some((xa, xb));
                None
            }
            _ => None,
        }
    }
}
