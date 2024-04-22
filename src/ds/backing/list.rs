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

struct TestNode<T> {
	val : T,
}

impl<T:Copy> NodeData for TestNode<T> {
	type TVal = T;

	fn get_val(&self) -> Self::TVal {
		self.val
	}

	fn set_val(&mut self, new_val: Self::TVal) {
		self.val = new_val;
	}
}

#[test]
fn add_rem_items() {
	let mut l: ListData<TestNode<i32>> = ListData::new();
	assert_eq!(l.len(), 0);
	assert_eq!(l.items.len(), 0);
	assert_eq!(l.free_list.len(), 0);
	assert_eq!(l.add_item(TestNode { val: 9 }), 0);
	assert_eq!(l.len(), 1);
	assert_eq!(l[0].val, 9);
	assert_eq!(l.add_item(TestNode { val: 8 }), 1);
	assert_eq!(l.len(), 2);
	assert_eq!(l[1].val, 8);
	l.rem_item(0);
	assert_eq!(l.len(), 1);
	assert_eq!(l.free_list.len(), 1);
	assert_eq!(l.free_list[0], 0);
	assert_eq!(l.add_item(TestNode { val: 7 }), 0);
	assert_eq!(l.len(), 2);
	assert_eq!(l[0].val, 7);
	assert_eq!(l.free_list.len(), 0);
}
