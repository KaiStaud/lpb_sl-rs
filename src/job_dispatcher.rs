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

#[derive(Debug)]

pub struct DataRingHelperError;

impl fmt::Display for DataRingHelperError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("Error catched by ring-helper function")
    }
}

impl Error for DataRingHelperError {}

pub struct RingManager {
    r1: AllocRingBuffer<Vec<TimedCoordinates>>,
    r2: AllocRingBuffer<TimedCoordinates>,
    r3: AllocRingBuffer<(Vector3<i32>, Vector3<i32>)>,
    read_index: [i32; 3],
    write_index: [i32; 3],
}
impl RingManager {
    pub fn new() -> RingManager {
        let tr1: AllocRingBuffer<Vec<TimedCoordinates>> = AllocRingBuffer::with_capacity(4);
        let tr2: AllocRingBuffer<TimedCoordinates> = AllocRingBuffer::with_capacity(4);
        let tr3: AllocRingBuffer<(Vector3<i32>, Vector3<i32>)> = AllocRingBuffer::with_capacity(4);
        let ri_array: [i32; 3] = [0; 3];
        let wi_array: [i32; 3] = [0; 3];

        RingManager {
            r1: (tr1),
            r2: (tr2),
            r3: (tr3),
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
            RING_SIZES[ring_no]
        }
    }

    // Get the first and last index with valid data
    // Ring Data will wrap around when overpushed or underpushed!
    fn get_valid_buffer_ranger(
        &self,
        ring_no: usize,
        current_buffer: i32,
        requested_length: i32,
    ) -> Result<(i32, i32), DataRingError> {
        let tuple = (current_buffer, current_buffer - requested_length);
        if tuple.1 > RING_SIZES[ring_no] && self.get_remaining_slots(ring_no) < requested_length {
            // We would overwrite previous set data
            Err(DataRingError)
        } else {
            Ok(tuple)
        }
    }

    fn get_open_requests(&self, ring_no: usize) -> i32 {
        // Write-pointer needs to lead Read-Pointer, ( or there is no work to do)
        if self.write_index[ring_no] > self.read_index[ring_no] {
            self.write_index[ring_no] - self.read_index[ring_no]
        } else {
            0
        }
    }

    fn update_pointer(&mut self, read_pointer: bool, ring_no: i32) -> i32 {
        if read_pointer {
            self.read_index[ring_no as usize] =
                (self.read_index[ring_no as usize] + 1) % RING_SIZES[ring_no as usize]
        } else {
            self.write_index[ring_no as usize] =
                (self.write_index[ring_no as usize] + 1) % RING_SIZES[ring_no as usize]
        }
        self.read_index[ring_no as usize]
    }

    fn translate_read_pointer(&mut self, rp: i32) -> isize {
        // Option<i32>
        (-rp - 1).try_into().unwrap()
    }

    fn refill_ring2(&mut self) -> i32 {
        // Wrap into Result<i32>!
        let mut transferred_elements = 1;

        // Is there enough space in Ring 2 to save another Vector when unpacked?
        let mut remaining_space = self.get_remaining_slots(1);
        let open_requests = self.get_open_requests(0);
        let mut transferred_requests = 0;

        let mut read_pointer = self.translate_read_pointer(self.read_index[0]);
        // Exit when theres enough storage space, but no pending requests
        // Also exit when not enough storage space is available
        while remaining_space > 0 && transferred_requests < open_requests {
            let requested_space = self.r1.get(read_pointer).cloned().unwrap().to_owned().len();
            if remaining_space >= requested_space.try_into().unwrap() {
                //println!("Theres enough space {}/{} left!",requested_space,remaining_space);
                // Unpack vector from ring1 into ring2
                read_pointer = self.translate_read_pointer(self.read_index[0]);
                let v = self.r1.get(read_pointer).cloned().unwrap().to_owned(); // Index needs to be r1_ri!

                let cv = v.clone();
                //println!("{}->{:?}",read_pointer,cv);
                // Are we overwriting unused data?

                for i in 0..cv.len() {
                    self.r2.push(cv[i].clone());
                    transferred_elements += 1;
                    self.update_pointer(false, 1);
                }
                transferred_elements += 1;
            } else {
                //println!("Not enough space {}/{}",requested_space,remaining_space);
                transferred_elements = 0;
                // exit while loop!
                break;
            }
            // Recalculate remaining space and update ring1's read-index
            remaining_space = self.get_remaining_slots(1);
            _ = self.update_pointer(true, 0) as isize;
            transferred_requests += 1;
            // Print updated pointer:
            //println!("Updated read-pointer to {} write-pointer to {} transferred {} requests",self.read_index[0],self.write_index[0],transferred_requests);
        }
        // Jump to beginning of while-loop!
        transferred_elements
    }

    fn refill_ring3(&mut self) -> i32 {
        let mut transferred_elements = 0;
        let remaining_space = self.get_remaining_slots(2);
        let mut read_pointer = self.translate_read_pointer(self.read_index[1]);
        let open_requests = self.get_open_requests(1);
        // While r3 has empty slots:
        while remaining_space > 0 && open_requests > transferred_elements {
            println!("{}/{}", transferred_elements, open_requests);
            let v = self.r2.get(read_pointer).unwrap().clone(); // Index needs to be r1_ri!
            println!("Reading R1 #{}:{:?}", read_pointer, v);
            self.r3.push((v.vector, v.rotation));
            self.update_pointer(true, 1);
            transferred_elements += 1;
            read_pointer = self.translate_read_pointer(self.read_index[1]); //-=1;
        }

        transferred_elements
    }

    pub fn set_job_from_ring3(&mut self) -> Option<(Vector3<i32>, Vector3<i32>)> {
        if self.read_index[2] < -10 {
            self.read_index[2] = -1;
        } else {
            self.read_index[2] -= 1;
        }
        self.r3.get(self.read_index[2].try_into().unwrap()).cloned()
    }

    pub fn auto_refill_rings(&mut self) -> Result<(), DataRingError> {
        _ = self.refill_ring2();
        _ = self.refill_ring3();
        Ok(())
    }

    pub fn push_to_ring1(&mut self, i: Vec<TimedCoordinates>) -> i32 {
        self.r1.push(i);
        self.write_index[0] += 1;
        RING1_SIZE - self.write_index[0] // Todo: Should return get available space!
    }
}

#[cfg(test)]
#[test]
fn test_ringbuff_utils() {
    let mut rm = RingManager::new();
    let r1 = rm.get_valid_buffer_ranger(0, -1, 2);
    assert_eq!(-3, r1.unwrap().1);

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
    let tc3 = TimedCoordinates {
        name: ("Test".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([77, 88, 99])),
        rotation: Vector3::from_vec(Vec::from([7, 8, 9])),
    };
    let tc4 = TimedCoordinates {
        name: ("Test2".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([12, 23, 45])),
        rotation: Vector3::from_vec(Vec::from([22, 33, 44])),
    };
    _ = rm.push_to_ring1(vec![tc, tc2]);
    _ = rm.push_to_ring1(vec![tc3, tc4]);

    assert_eq!(2, rm.get_open_requests(0));

    assert_eq!(-1, rm.translate_read_pointer(rm.read_index[0]));
    let mut v = rm.update_pointer(true, 0);
    assert_eq!(-2, rm.translate_read_pointer(v));
    v = rm.update_pointer(true, 0);
    assert_eq!(-3, rm.translate_read_pointer(v));
    v = rm.update_pointer(true, 0);
    assert_eq!(-4, rm.translate_read_pointer(v));

    //let err = rm.get_valid_buffer_ranger(0, -1, 10);
    //assert_eq!(DataRingHelperError>,)
}

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
        name: ("Test1".to_string()),
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
    let tc3 = TimedCoordinates {
        name: ("Test3".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([77, 88, 99])),
        rotation: Vector3::from_vec(Vec::from([7, 8, 9])),
    };
    let tc4 = TimedCoordinates {
        name: ("Test4".to_string()),
        timestamp: (10),
        vector: Vector3::from_vec(Vec::from([12, 23, 45])),
        rotation: Vector3::from_vec(Vec::from([22, 33, 44])),
    };
    _ = rm.push_to_ring1(vec![tc, tc2]);
    _ = rm.push_to_ring1(vec![tc3, tc4]);

    for i in -2..0 {
        println!("#{:?}{:?}", i, rm.r1.get(i));
    }

    let ret = rm.refill_ring2();

    let mut r2_content = rm.r2.get(-2).unwrap().clone();
    assert_eq!(
        r2_content,
        TimedCoordinates {
            name: ("Test1".to_string()),
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
    let _ = rm.refill_ring3();

    let r3_content = rm.set_job_from_ring3().unwrap().clone();
    let r32_content = rm.set_job_from_ring3().unwrap().clone();

    assert_eq!(
        r32_content,
        (
            Vector3::from_vec(Vec::from([44, 55, 66])),
            Vector3::from_vec(Vec::from([4, 5, 6]))
        )
    );
    assert_eq!(
        r3_content,
        (
            Vector3::from_vec(Vec::from([11, 22, 33])),
            Vector3::from_vec(Vec::from([1, 2, 3]))
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

    _ = rm.auto_refill_rings();

    let r3_content = rm.set_job_from_ring3().unwrap().clone();
    let r32_content = rm.set_job_from_ring3().unwrap().clone();

    assert_eq!(
        r32_content,
        (
            Vector3::from_vec(Vec::from([44, 55, 66])),
            Vector3::from_vec(Vec::from([4, 5, 6]))
        )
    );
    assert_eq!(
        r3_content,
        (
            Vector3::from_vec(Vec::from([11, 22, 33])),
            Vector3::from_vec(Vec::from([1, 2, 3]))
        )
    );
}

#[test]
fn test_overpush_ring() {
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
    let v = vec![tc, tc2];
    // Fill ring1 with 10 elements!
    for i in 0..9 {
        let x = v.clone();
        _ = rm.push_to_ring1(x);
    }
    // Eleventh element will overwrite, resulting in return value = 0
    assert_eq!(rm.push_to_ring1(v.clone()), 0);
}

#[test]
fn test_overpull_ring() {}
