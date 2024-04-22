use super::node::NodeData;

pub struct ListData<TNode: NodeData> {
	// datastructure nodes stored in a vector
	items: Vec<Box<TNode>>,
	// index to the head node of the datastructure
	pub head: Option<usize>,
	// number of items currently stored
	count: usize,
	// indexes to elemets in the vector free to be reallocated
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
				self.items[i].set_val(item.get_val());
				i
			},
			None => {
				self.items.push(Box::new(item));
				self.items.len() - 1
			}
		}
	}

	pub fn rem_item(&mut self, i: usize) {
		self.free_list.push(i);
		self.count -= 1;
	}
}

impl<TNode: NodeData> std::ops::Index<usize> for ListData<TNode> {
	type Output = TNode;

	fn index(&self, index: usize) -> &Self::Output {
		&self.items[index]
	}
}

impl<TNode: NodeData> std::ops::IndexMut<usize> for ListData<TNode> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self.items[index]
	}
}