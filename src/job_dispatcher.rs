use error_stack::{IntoReport, ResultExt};
use nalgebra::Vector3;
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};
use std::{error::Error, fmt};
const RING1_SIZE: i32 = 10;
const RING2_SIZE: i32 = 10;
const RING3_SIZE: i32 = 10;
const RING_SIZES: [i32; 3] = [RING1_SIZE, RING2_SIZE, RING3_SIZE];

#[derive(Clone, Debug, PartialEq)]
pub struct TimedCoordinates {
    pub name: String,
    pub timestamp: u8,
    pub vector: Vector3<i32>,
    pub rotation: Vector3<i32>,
}
/*
 pub fn main() {
    let mut buffer = AllocRingBuffer::with_capacity(4);

    // First entry of the buffer is now 5.
    buffer.push(5);
    buffer.push(42);
    buffer.push(10);
//    buffer.push(20); Werden nicht alle buffer gefüllt,
// ist der letzte+1 (2+1) automatisch der erste buffer!

    println!("{:?}",buffer.get(3));
    println!("{:?}",buffer.get(2));
    println!("{:?}",buffer.get(1));
    println!("{:?}",buffer.get(0));
// buffer.get(0) ist der zuletzt gepushte: 20 10 42 5
// Oder mit negativen Index:
println!("Negative Indexe");
    println!("{:?}",buffer.get(-1));
    println!("{:?}",buffer.get(-2));
    println!("{:?}",buffer.get(-3));
    println!("{:?}",buffer.get(-4));

// buffer.get(-1) ist immer der zuletzt gepushte: 20 10 42 5
}
*/
#[derive(Debug)]
pub struct DataRingError;

impl fmt::Display for DataRingError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Could not read/write ringbuffer")
    }
}

impl Error for DataRingError {}
struct RingManager {
    r1: AllocRingBuffer<Vec<TimedCoordinates>>,
    r2: AllocRingBuffer<TimedCoordinates>,
    r3: AllocRingBuffer<(Vector3<i32>, Vector3<i32>)>,
    read_index: [i32; 3],
    write_index: [i32; 3],
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
        let tr3: AllocRingBuffer<(Vector3<i32>, Vector3<i32>)> = AllocRingBuffer::with_capacity(4);
        let ri_array: [i32; 3] = [0; 3];
        let wi_array: [i32; 3] = [0; 3];

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
            read_index: (ri_array),
            write_index: (wi_array),
        }
    }

    fn get_remaining_slots(&self, ring_no: usize) -> i32 {
        if self.write_index[ring_no] > self.read_index[ring_no] {
            self.read_index[ring_no] - 1 + RING_SIZES[ring_no] - self.write_index[ring_no]
        } else if self.read_index[ring_no] > self.write_index[ring_no] {
            self.read_index[ring_no] - self.write_index[ring_no]
        } else {
            0
        }
    }

    fn update_pointer(&mut self, read_pointer: bool, ring_no: i32) {
        if read_pointer {
            self.read_index[ring_no as usize] =
                (self.read_index[ring_no as usize] + 1) % RING_SIZES[ring_no as usize]
        } else {
            self.write_index[ring_no as usize] =
                (self.write_index[ring_no as usize] + 1) % RING_SIZES[ring_no as usize]
        }
    }

    fn refill_ring2(&mut self) -> i32 {
        let mut transferred_elements = 0;

        // Unpack vector from ring1 into ring2
        let v = self.r1.get(-1).cloned().unwrap().to_owned(); // Index needs to be r1_ri!
        let cv = v.clone();
        for i in 0..cv.len() {
            self.r2.push(cv[i].clone());
            transferred_elements += 1;
            self.update_pointer(false, 1);
        }
        transferred_elements
    }

    fn refill_ring3(&mut self) -> i32 {
        let mut transferred_elements = 0;
        let mut ix = -1;
        // While r3 has empty slots:
        for i in 0..2 {
            // Get from r2:
            let v = self.r2.get(ix.try_into().unwrap()).unwrap().clone(); // Index needs to be r1_ri!

            if self.r3_wi >= self.r3_ri && self.r3_ri <= RING3_SIZE {
                // Save to r3:
                self.r3.push((v.vector, v.rotation));
                self.update_pointer(false, 0);
                self.r3_wi += 1;

                transferred_elements += 1;
                println!("Got from R3:{:?},{:?}", ix, self.r3.get(-1));
                ix -= 1;
            }
        }
        transferred_elements
    }

    fn set_job_from_ring3() {}

    fn push_to_ring1(&mut self, i: Vec<TimedCoordinates>) -> i32 {
        self.r1.push(i);
        self.r1_wi += 1;
        self.write_index[0] += 1;
        let remaining_slot = self.r1.peek().unwrap();
        RING1_SIZE - self.r1_wi
    }
}

#[cfg(test)]
#[test]
fn test_push_into_empty_rings() {
    let mut rm = RingManager::new();
    let tc = TimedCoordinates {
        name: ("Test".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([11, 22, 33])),
        rotation: Vector3::from_vec(Vec::from([1, 2, 3])),
    };
    let tc2 = TimedCoordinates {
        name: ("Test2".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([44, 55, 66])),
        rotation: Vector3::from_vec(Vec::from([4, 5, 6])),
    };
    let ret = rm.push_to_ring1(vec![tc, tc2]);
    assert_eq!(RING1_SIZE - 1, ret);
}

#[test]
fn test_fill_r2() {
    let mut rm = RingManager::new();
    let tc = TimedCoordinates {
        name: ("Test".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([11, 22, 33])),
        rotation: Vector3::from_vec(Vec::from([1, 2, 3])),
    };
    let tc2 = TimedCoordinates {
        name: ("Test2".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([44, 55, 66])),
        rotation: Vector3::from_vec(Vec::from([4, 5, 6])),
    };
    _ = rm.push_to_ring1(vec![tc, tc2]);
    //let v = rm.r1.get(-1).cloned().unwrap();
    //println!("{:?}", v);
    let ret = rm.refill_ring2();
    let mut r2_content = rm.r2.get(-2).unwrap().clone();
    assert_eq!(
        r2_content,
        TimedCoordinates {
            name: ("Test".to_string()),
            timestamp: (10),
            vector: Vector3::from_vec(Vec::from([11, 22, 33])),
            rotation: Vector3::from_vec(Vec::from([1, 2, 3])),
        }
    );

    r2_content = rm.r2.get(-1).unwrap().clone();
    assert_eq!(
        r2_content,
        TimedCoordinates {
            name: ("Test2".to_string()),
            timestamp: (10),
            vector: Vector3::from_vec(Vec::from([44, 55, 66])),
            rotation: Vector3::from_vec(Vec::from([4, 5, 6])),
        }
    );
    assert_eq!(2, ret); // 2 TC transferred
}

#[test]
fn test_fill_r3() {
    let mut rm = RingManager::new();
    let tc = TimedCoordinates {
        name: ("Test".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([11, 22, 33])),
        rotation: Vector3::from_vec(Vec::from([1, 2, 3])),
    };
    let tc2 = TimedCoordinates {
        name: ("Test2".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([44, 55, 66])),
        rotation: Vector3::from_vec(Vec::from([4, 5, 6])),
    };
    _ = rm.push_to_ring1(vec![tc, tc2]);
    let _ = rm.refill_ring2();
    let _ = rm.r2.get(-2).unwrap().clone();

    let _ = rm.refill_ring3();

    let mut r3_content = rm.r3.get(-1).unwrap().clone();
    assert_eq!(
        r3_content,
        (
            Vector3::from_vec(Vec::from([11, 22, 33])),
            Vector3::from_vec(Vec::from([1, 2, 3]))
        )
    );
    r3_content = rm.r3.get(-2).unwrap().clone();
    assert_eq!(
        r3_content,
        (
            Vector3::from_vec(Vec::from([44, 55, 66])),
            Vector3::from_vec(Vec::from([4, 5, 6]))
        )
    );
}

#[test]
fn trickle_down_r1_to_queue() {
    let mut rm = RingManager::new();
    let tc = TimedCoordinates {
        name: ("Test".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([11, 22, 33])),
        rotation: Vector3::from_vec(Vec::from([1, 2, 3])),
    };
    let tc2 = TimedCoordinates {
        name: ("Test2".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([44, 55, 66])),
        rotation: Vector3::from_vec(Vec::from([4, 5, 6])),
    };
    _ = rm.push_to_ring1(vec![tc, tc2]);
}

#[test]
fn test_overpush_ring() {}

#[test]
fn test_overpull_ring() {}
