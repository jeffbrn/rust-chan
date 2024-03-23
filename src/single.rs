use std::{sync::{Arc, Mutex, atomic::AtomicBool}, thread::JoinHandle, time::Duration};

use crossbeam_channel::{Receiver, Sender};

/*
 * Single channel all works subscribe only 1 worker receives the message
 */

/// Holds channel objects for the main thread to communicate with workers
pub struct MessageBus {
    tx: Sender<(String, usize)>,
    rx: Receiver<(String, usize)>
}

impl MessageBus {
    pub fn new(capacity: u8) -> Self {
        // channels are a fixed size and will not queue msgs beyond capacity
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
                std::thread::yield_now(); // free up thread to give workers a chance to catchup
            }
        }
        retval
    }
 
    fn get_recvr(&self) -> Receiver<(String, usize)> {
        self.rx.clone()
    }
}

/// This is a unit that receives messages to do work
pub struct Worker {
    rec_cnt: Arc<Mutex<u32>>, // counter of messages received
    handle: JoinHandle<()>, // worker thread handle
    interrupt: Arc<AtomicBool>, // signal to thread to exit
}
 
impl Worker {
    pub fn new(nm : String, mb: &MessageBus, do_delay: bool) -> Self {
        let recvr = mb.get_recvr();
        let ctr = Arc::new(Mutex::new(0));
        let intr = Arc::new(AtomicBool::new(false));
        let intr_ = intr.clone();
        let retval = Self {
            rec_cnt: ctr.clone(),
            handle: std::thread::spawn(move || {
                println!("Creating worker: {:?}", nm);
                let chk_stop = intr.clone();
                loop {
                    let message = recvr.recv_timeout(Duration::from_millis(500));
                    match message {
                        Ok(msg) => {
                            println!("  worker '{}' | received msg # {} : {}", nm, msg.1, msg.0);
                            {
                                // increment rec counter
                                let mut n = ctr.lock().unwrap();
                                *n += 1;
                            }
                        },
                        Err(_) => {
                            // recv timeout
                            if chk_stop.load(std::sync::atomic::Ordering::SeqCst) {
                                // thread signalled to stop
                                break;
                            }
                            continue;
                        }
                    }
                    if do_delay {
                        // pretend to do lengthy work
                        std::thread::sleep(Duration::from_secs(2));
                    }
                }
            }),
            interrupt: intr_
        };
        std::thread::yield_now();
        retval
    }

    /// get the number of msgs recvd by worker
    pub fn get_cnt(&self) -> u32 { *self.rec_cnt.lock().unwrap() }

    /// signal the worker to stop and wait until it does
    pub fn stop(self) {
        self.interrupt.store(true, std::sync::atomic::Ordering::SeqCst);
        self.handle.join().expect("Failed to join thread");
    }
}
