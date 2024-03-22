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
        assert_eq!(mb.send(("this is the send message".to_string(), i)), true);
    }
    std::thread::sleep(Duration::from_secs(1));

    let mut sum: u32 = 0;
    for w in workers.iter() {
        sum += w.get_cnt();
    }
    assert_eq!(sum, 10);
}

#[test]
fn test_delay() {
    let mb = single::MessageBus::new(1);
    let wrk = single::Worker::new("Worker 1".to_string(), &mb, true);
    std::thread::sleep(Duration::from_secs(1));
    assert_eq!(mb.send(("this is the send message".to_string(), 1)), true);
    assert_eq!(mb.send(("this is the send message".to_string(), 2)), true);
    assert_eq!(mb.send(("this is the send message".to_string(), 3)), false);
    assert_eq!(mb.send(("this is the send message".to_string(), 4)), false);
    assert_eq!(mb.send(("this is the send message".to_string(), 5)), false);
    assert_eq!(mb.send(("this is the send message".to_string(), 6)), false);
    assert_eq!(mb.send(("this is the send message".to_string(), 7)), false);
    std::thread::sleep(Duration::from_secs(3));
    assert_eq!(wrk.get_cnt(), 2);
}
