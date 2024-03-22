use std::{sync::{Arc, Mutex}, time::Duration};

use crossbeam_channel::{Sender, Receiver};

/*
 * Single channel all works subscribe only 1 worker receives the message
 */

pub struct MessageBus {
    tx: Sender<(String, usize)>,
    rx: Receiver<(String, usize)>
}

impl MessageBus {
    pub fn new(capacity: u8) -> Self {
        let (t, r) = crossbeam_channel::bounded(capacity as usize);
        Self {tx: t, rx: r}
    }
 
    pub fn send(&self, msg: (String, usize)) {
        let _res = self.tx.send((msg.0, msg.1)).unwrap();
    }
 
    fn get_recvr(&self) -> Receiver<(String, usize)> {
        self.rx.clone()
    }
}
 
pub struct Worker {
    name: String,
    inputs: Receiver<(String, usize)>,
    rec_cnt: Arc<Mutex<u32>>,
    delay: bool
}
 
impl Worker {
    pub fn new(nm : String, mb: &MessageBus, do_delay: bool) -> Self {
        let retval = Self {
            name: nm.clone().to_string(),
            inputs: mb.get_recvr(),
            rec_cnt: Arc::new(Mutex::new(0)),
            delay: do_delay
        };
        retval.run();
        retval
    }

    pub fn run(&self) {
        let ctr = Arc::clone(&self.rec_cnt);
        let name1 = self.name.clone();
        let x = self.inputs.clone();
        let do_delay = self.delay;
        std::thread::spawn(move || {
            println!("Creating worker: {:?}", name1);
            loop {
                let message = x.recv();
                let msg = message.unwrap();
                println!("  worker '{}' | received msg # {} : {}", name1, msg.1, msg.0);
                {
                    let mut n = ctr.lock().unwrap();
                    *n += 1;
                }
                if do_delay {
                    std::thread::sleep(Duration::from_secs(2));
                }
            }
        });
    }
 
    pub fn get_cnt(&self) -> u32 { *self.rec_cnt.lock().unwrap() }

}
