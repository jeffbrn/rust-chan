use super::node::{NodeData, ListData};

struct Node<T> {
	val: T,
	next: Option<usize>,
}

impl<T:Copy> NodeData for Node<T> {
	type TVal = T;

	fn get_val(&self) -> Self::TVal {
		self.val
	}

	fn set_val(&mut self, new_val: Self::TVal) {
		self.val = new_val;
	}
}

pub struct List<T:Copy> {
	data: ListData<Node<T>>,
}

impl<T:Copy> List<T> {
	pub fn new() -> List<T>{
		Self {
			data: ListData::new()
		}
	}
	pub fn len(&self) -> usize { self.data.len() }

	pub fn add(&mut self, val : T) -> usize {
		let item = Node { val, next: self.data.head };
		self.data.add_item(item)
	}

}
