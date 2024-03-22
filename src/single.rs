use std::{sync::{Arc, Mutex, atomic::AtomicBool}, thread::JoinHandle, time::Duration};

use crossbeam_channel::{Receiver, Sender};

/*
 * Single channel all works subscribe only 1 worker receives the message
 */

/// 
 pub struct MessageBus {
    tx: Sender<(String, usize)>,
    rx: Receiver<(String, usize)>
}

impl MessageBus {
    pub fn new(capacity: u8) -> Self {
        let (t, r) = crossbeam_channel::bounded(capacity as usize);
        Self {tx: t, rx: r}
    }
 
    pub fn send(&self, msg: (String, usize)) -> bool {
        println!("  main        | sending msg # {} : {}", msg.1, msg.0);
        let result = self.tx.send_timeout((msg.0, msg.1), Duration::from_millis(100));
        let mut retval = true;
        match result {
            Ok(_) => {},
            Err(error) => {
                println!("Send error: {}", error.to_string());
                retval = false;
                std::thread::yield_now();
            }
        }
        retval
    }
 
    fn get_recvr(&self) -> Receiver<(String, usize)> {
        self.rx.clone()
    }
}
 
pub struct Worker {
    name: String,
    inputs: Receiver<(String, usize)>,
    rec_cnt: Arc<Mutex<u32>>,
    delay: bool,
    handle: JoinHandle<()>,
    interrupt: Arc<AtomicBool>,
}
 
impl Worker {
    pub fn new(nm : String, mb: &MessageBus, do_delay: bool) -> Self {
        let recvr = mb.get_recvr();
        let ctr = Arc::new(Mutex::new(0));
        let intr = Arc::new(AtomicBool::new(false));
        let intr_ = intr.clone();
        let retval = Self {
            name: nm.clone(),
            inputs: mb.get_recvr(),
            rec_cnt: ctr.clone(),
            delay: do_delay,
            handle: std::thread::spawn(move || {
                println!("Creating worker: {:?}", nm);
                let chk_stop = intr.clone();
                loop {
                    let message = recvr.recv_timeout(Duration::from_millis(500));
                    match message {
                        Ok(msg) => {
                            println!("  worker '{}' | received msg # {} : {}", nm, msg.1, msg.0);
                            {
                                let mut n = ctr.lock().unwrap();
                                *n += 1;
                            }
                        },
                        Err(_) => {
                            if chk_stop.load(std::sync::atomic::Ordering::SeqCst) {
                                break;
                            }
                            continue;
                        }
                    }
                    if do_delay {
                        std::thread::sleep(Duration::from_secs(2));
                    }
                }
            }),
            interrupt: intr_
        };
        std::thread::yield_now();
        retval
    }

    pub fn get_cnt(&self) -> u32 { *self.rec_cnt.lock().unwrap() }

    pub fn stop(self) {
        self.interrupt.store(true, std::sync::atomic::Ordering::SeqCst);
        self.handle.join().expect("Failed to join thread");
    }
}
