mod single;
mod broadcast;
mod ds;

use std::time::Duration;

fn main() {
    println!("Running channels broadcast");
    let mut workers = broadcast::WorkerManager::new();
    for n in 1..=5 {
        workers.add(broadcast::Worker::new(format!("Worker {n}"), false));
    }
    std::thread::sleep(Duration::from_secs(1));
    for i in 1..=10 {
        workers.send(("this is the msg".to_string(), i));
    }
    std::thread::sleep(Duration::from_secs(1));
    let _success = workers.chk_msg_counts(10);

    println!("Running channels single");
    let mb = single::MessageBus::new(1);
    let mut workers: Vec<single::Worker> = Vec::new();
    for n in 1..=5 {
        workers.push(single::Worker::new(format!("Worker {n}"), &mb, false));
    }
    std::thread::sleep(Duration::from_secs(1));
    for i in 1..=10 {
        mb.send(("this is the send message".to_string(), i));
    }
    std::thread::sleep(Duration::from_secs(1));

    let mut sum: u32 = 0;
    for w in workers.iter() {
        sum += w.get_cnt();
    }
    assert_eq!(sum, 10);
    for w in workers {
        w.stop();
    }

    println!("Running Single Linked List");
	let mut list = ds::linked_list::LinkedList::<i32>::new();
	assert!(list.is_empty());
	list.push(2);
	list.push(1);
	list.push(40);
	list.add_tail(3);
	list.add_tail(4);
	list.add_tail(45);
	let x = list.pop();
	assert_eq!(x.unwrap(), 40);
	let x = list.remove_tail();
	assert_eq!(x.unwrap(), 45);
	assert_eq!(list.len(), 4);
	let mut i = 1;
	// for x in list {
	// 	println!("item {i} = {x}");
	// 	i += 1;
	// }

    println!("Running Binary Tree");
	let mut tree = ds::b_tree::BTree::<i32>::new();
	assert!(tree.is_empty());
    tree.insert(10);
    assert_eq!(tree.count(), 1);
}
