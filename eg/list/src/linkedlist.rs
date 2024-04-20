struct Node<T> {
	val: T,
	next: Option<usize>,
}

pub struct List<T> {
	items: Vec<Node<T>>,
	head: Option<usize>,
	count: usize,
	free_list: Vec<usize>,
}

impl<T> List<T> {
	pub fn new() -> List<T>{
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
				self.items.push(Node { val: item, next: None});
				self.items.len() - 1
			}
		}
	}

	pub fn rem_item(&mut self, i: usize) {
		self.free_list.push(i);
		self.count -= 1;
	}
}
