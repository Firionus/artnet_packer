use super::DMXData;
use std::time;

pub struct Suppressor {
    previous: DMXData,
    last_packed: time::Instant,
}

impl Suppressor {
    pub fn new(dmx: &DMXData) -> Self {
        Suppressor {
            previous: dmx.clone(),
            last_packed: time::Instant::now() - time::Duration::from_secs(20), //magic number 20 secs to trigger sending when should_pack is called
        }
    }
    pub fn should_pack(&mut self, new: &DMXData) -> bool {
        let keepalive = time::Duration::from_millis(800);
        //check if DMXData has changed
        if new != &self.previous {
            self.previous = new.clone();
            true
        } else {
            //check for keepalive deadline
            if time::Instant::now() > self.last_packed + keepalive {
                self.last_packed = time::Instant::now();
                true
            } else {
                false
            }
        }
    }
}
