struct Node<T> {
	val: T,
	left: Option<usize>,
	right: Option<usize>,
}

pub struct Tree<T> {
	items: Vec<Node<T>>,
	head: Option<usize>,
	count: usize,
	free_list: Vec<usize>,
}

impl<T> Tree<T> {
	pub fn new() -> Tree<T>{
		Self {
			items: Vec::new(), head: None, count: 0, free_list: Vec::new()
		}
	}
	pub fn len(&self) -> usize { self.count }

	pub fn add_item(&mut self, item : T) -> usize {
		self.count += 1;
		match self.free_list.pop() {
			Some(i) => {
				self.items[i].val = item;
				i
			},
			None => {
				self.items.push(Node { val: item, left: None, right: None});
				self.items.len() - 1
			}
		}
	}

	pub fn rem_item(&mut self, i: usize) {
		self.free_list.push(i);
		self.count -= 1;
	}
}
