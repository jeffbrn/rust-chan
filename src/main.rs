mod ds;

fn main() {
    println!("Hello, world!");
    let mut list = ds::sl_list::SlList::<i32>::new();
	list.add_tail(1);
	list.add_tail(2);
	list.add_tail(3);
	let x = list.remove_tail().unwrap();
	let x = list.remove_tail().unwrap();
	let x = list.remove_tail().unwrap();
}

mod single;
mod broadcast;
