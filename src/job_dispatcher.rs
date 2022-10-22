use ringbuffer::{AllocRingBuffer, RingBuffer};

use crate::serialization;
use serialization::serialization::TimedCoordinates;
struct RingManager {
    buffer: AllocRingBuffer<TimedCoordinates>,
    lock_stat: bool,
    following_rm: Box<RingManager>,
}

impl RingManager {
    fn link_to(&self, next_rm: RingManager) {
        //self.following_rm = next_rm.;
    }

    fn open(&self) {}

    fn notify_next(&self) -> bool {
        return true;
    }

    fn add_debug_ring(&self) -> bool {
        return true;
    }

    fn request_previous(&self) {}
}
