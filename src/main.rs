fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod single;
#[cfg(test)]
use std::time::Duration;

#[test]
fn test_single() {
    let mb = single::MessageBus::new(1);
    let mut workers: Vec<single::Worker> = Vec::new();
    for n in 1..=5 {
        workers.push(single::Worker::new(format!("Worker {n}"), &mb, false));
    }

    std::thread::sleep(Duration::from_secs(1));
    for i in 1..=10 {
        mb.send(("message: ".to_string(), i));
    }
    std::thread::sleep(Duration::from_secs(1));

    let mut sum: u32 = 0;
    for w in workers.iter() {
        sum += w.get_cnt();
    }
    assert_eq!(sum, 10);
}
