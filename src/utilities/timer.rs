use std::time::{Duration, Instant, SystemTime};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Timer {
    start_time: SystemTime
}


impl Timer {

    pub fn new() -> Self {
        Timer {
            start_time: SystemTime::now()
        }
    }

    pub fn get_time(&self) -> f32 {
        let current_time = SystemTime::now();
        let elapsed_time =
            current_time
                .duration_since(self.start_time)
                .expect("| ERROR::TIMER: Could not get elapsed time");
        (elapsed_time.as_secs() * 1000) as f32
            + (elapsed_time.subsec_nanos() as f32 / 1_000_000 as f32)
    }
}


