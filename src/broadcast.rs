use crossbeam_channel::{Sender, Receiver};
use std::{sync::{Arc, Mutex, atomic::AtomicBool}, thread::JoinHandle, time::Duration};

pub struct Worker {
    tx: Sender<(String, usize)>,
    rec_cnt: Arc<Mutex<u32>>, // counter of messages received
    handle: JoinHandle<()>, // worker thread handle
    interrupt: Arc<AtomicBool>, // signal to thread to exit
}

impl Worker {
    pub fn new(nm : String, do_delay: bool) -> Self {
        let (t, r) = crossbeam_channel::bounded(1);
        let ctr = Arc::new(Mutex::new(0));
        let intr = Arc::new(AtomicBool::new(false));
        let intr_ = intr.clone();
        let retval = Self {
            tx: t,
            rec_cnt: ctr.clone(),
            interrupt: intr_,
            handle: std::thread::spawn(move || {
                println!("Creating worker: {:?}", nm);
                let chk_stop = intr.clone();
                loop {
                    let message = r.recv_timeout(Duration::from_millis(500));
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
        };

        retval
    }

    pub fn send(&self, msg: (String,usize)) -> bool {
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

    /// get the number of msgs recvd by worker
    pub fn get_cnt(&self) -> u32 { *self.rec_cnt.lock().unwrap() }

    /// signal the worker to stop and wait until it does
    pub fn stop(self) {
        self.interrupt.store(true, std::sync::atomic::Ordering::SeqCst);
        self.handle.join().expect("Failed to join thread");
    }
}

pub struct WorkerManager {
    workers: Vec<Worker>,
}

impl WorkerManager {
    fn new() -> Self {
        Self {
            workers: Vec::new()
        }
    }

    fn add(&mut self, wrk: Worker) {
        self.workers.push(wrk);
    }

    fn send(&self, msg: (String,usize)) -> bool {
        let mut result = true;
        for wrk in self.workers.iter() {
            result = result && wrk.send(msg.clone());
        }
        result
    }

    #[cfg(test)]
    fn chk_msg_counts(&self, expected: u32) -> bool {
        for wrk in self.workers.iter() {
            if wrk.get_cnt() != expected {
                return false;
            }
        }
        return true;
    }
}

impl Drop for WorkerManager {
    fn drop(&mut self) {
        while self.workers.len() > 0 {
            let w = self.workers.remove(0);
            println!("Stopping worker");
            w.stop();
        }
    }
}

#[test]
fn test_broadcast() {
    println!("Testing broadcast");
    let mut workers = WorkerManager::new();
    for n in 1..=5 {
        workers.add(Worker::new(format!("Worker {n}"), false));
    }
    std::thread::sleep(Duration::from_secs(1));
    for i in 1..=10 {
        assert_eq!(workers.send(("this is the msg".to_string(), i)), true);
    }
    std::thread::sleep(Duration::from_secs(1));
    assert_eq!(workers.chk_msg_counts(10), true);
}
