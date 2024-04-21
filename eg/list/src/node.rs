pub trait NodeData {
	type TVal;

	fn get_val(&self) -> Self::TVal;
	fn set_val(&mut self, new_val: Self::TVal);
}

pub struct ListData<TNode: NodeData> {
	items: Vec<TNode>,
	pub head: Option<usize>,
	count: usize,
	free_list: Vec<usize>,
}

impl<TNode: NodeData> ListData<TNode> {
	pub fn new() -> Self {
		Self {
			items: Vec::new(), head: None, count: 0, free_list: Vec::new()
		}
	}

	pub fn len(&self) -> usize { self.count }

	pub fn add_item(&mut self, item : TNode) -> usize {
		self.count += 1;
		match self.free_list.pop() {
			Some(i) => {
				self.items[0].set_val(item.get_val());
				i
			},
			None => {
				self.items.push(item);
				self.items.len() - 1
			}
		}
	}

	pub fn rem_item(&mut self, i: usize) {
		self.free_list.push(i);
		self.count -= 1;
	}
}