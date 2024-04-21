use super::backing::{node::NodeData, list::ListData};

struct Node<T> {
	/// list items are stored in a vector with vector indexes used instead of pointers
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

/// This structure is for a single-linked-list implementation that does not use pointers
/// See datastructure implementation [here](https://opendatastructures.org/ods-python/3_1_SLList_Singly_Linked_Li.html)
pub struct LinkedList<T:Copy> {
	data: ListData<Node<T>>,
	tail: Option<usize>,
}

// region v List Iterators

/// This is the struct that keeps track of iterating a list
/// # Examples
/// 
/// ```
/// let l = SlList::<i32>::new();
/// for item in l {
///     println!(item);
/// }
/// ```
// pub struct SlListIter<T> {
// 	curr_idx: Option<usize>,
// 	list: SlList<T>,
// }
// /// Iterator for the list 
// impl<T: Copy> Iterator for SlListIter<T> {
// 	type Item = T;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		let idx = match self.curr_idx {
// 			None => {
// 				return None
// 			},
// 			Some(i) => {
// 				i
// 			}
// 		};
// 		let item = self.list.items[idx].clone();
// 		self.curr_idx = item.next;
// 		Some(item.val)
// 	}
// }
// /// Produces the iterator for the list
// impl <T: std::marker::Copy> IntoIterator for SlList<T> {
// 	type Item = T;
// 	type IntoIter = SlListIter<T>;

// 	fn into_iter(self) -> Self::IntoIter {
// 		Self::IntoIter {
// 			curr_idx: self.head,
// 			list: self,
// 		}
// 	}
// }
// region ^ List Iterators

/// Linked list methods
impl<T: Copy> LinkedList<T> {
	/// constructor
	pub fn new() -> Self {
		Self { data: ListData::new(), tail: None }
	}

	/// add a new value to the front of the list
	pub fn push(&mut self, item: T) {
		let n = Node { val: item, next: self.data.head };
		let idx = self.data.add_item(n);
		self.data.head = Some(idx);
		if self.len() == 1 {
			self.tail = self.data.head;
		}
	}
	/// fetch and remove a value from the front of the list
	pub fn pop(&mut self) -> Option<T> {
		match self.data.head {
			None => None,
			Some(idx) => {
				let n = self.data[idx].next;
				let v = self.data[idx].val;
				self.data.head = n;
				self.data.rem_item(idx);
				if self.len() == 0 {
					self.tail = None;
				}
				Some(v)
			}
		}
	}
	/// add a new value to the end of a list
	pub fn add_tail(&mut self, item: T) {
		let n = Node { val: item, next: None };
		let idx = self.data.add_item(n);
		if let Some(tail_idx) = self.tail {
			self.data[tail_idx].next = Some(idx);
		}
		self.tail = Some(idx);
		if self.len() == 1 {
			self.data.head = self.tail;
		}
	}
	/// fetch and remove a value from the back of the list
	pub fn remove_tail(&mut self) -> Option<T> {
		let tail_idx = match self.tail {
			None => return None,
			Some(i) => i
		};
		let result = self.data[tail_idx].val;
		self.data.rem_item(tail_idx);
		if self.data.head == self.tail {
			self.data.head = None;
			self.tail = None;
		} else {
			let mut new_tail: usize = self.data.head.unwrap();
			loop {
				let nxt_idx = match self.data[new_tail].next {
					None => panic!("list linkage is incorrect"),
					Some(i) => i
				};
				if nxt_idx == tail_idx {
					self.data[new_tail].next = None;
					break;
				}
				new_tail = nxt_idx;
			}
			self.tail = Some(new_tail);
		}
		Some(result)
	}

	pub fn len(&self) -> usize { self.data.len() }
	pub fn is_empty(&self) -> bool { self.len() == 0 }

}

#[test]
fn empty() {
	let list = LinkedList::<i32>::new();
	assert_eq!(list.len(), 0);
	assert_eq!(list.data.head, None);
	assert_eq!(list.tail, None);
	// for _ in list {
	// 	assert!(false);
	// }
}

#[test]
fn push() {
	let mut list = LinkedList::<i32>::new();
	list.push(1);
	assert_eq!(list.data.len(), 1);
	assert_eq!(list.data[0].val, 1);
	assert_eq!(list.data[0].next, None);
	assert_eq!(list.data.head, Some(0));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.len(), 1);

	list.push(2);
	assert_eq!(list.data.len(), 2);
	assert_eq!(list.data[0].val, 1);
	assert_eq!(list.data[0].next, None);
	assert_eq!(list.data[1].val, 2);
	assert_eq!(list.data[1].next, Some(0));
	assert_eq!(list.data.head, Some(1));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.len(), 2);
	let mut i = 2;
	// for x in list {
	// 	assert_eq!(x, i);
	// 	i -= 1;
	// }
}

#[test]
fn pop() {
	let mut list = LinkedList::<i32>::new();
	list.push(1);
	list.push(2);
	let val = list.pop();
	assert_eq!(val, Some(2));
	assert_eq!(list.len(), 1);
	assert_eq!(list.data.head, Some(0));
	assert_eq!(list.tail, Some(0));

	let val = list.pop();
	assert_eq!(val, Some(1));
	assert_eq!(list.len(), 0);
	assert_eq!(list.data.head, None);
	assert_eq!(list.tail, None);

	let val = list.pop();
	assert_eq!(val, None);
}

#[test]
fn push_n_pop() {
	let mut list = LinkedList::<i32>::new();
	list.push(1);
	list.push(2);
	let val = list.pop();
	assert_eq!(val, Some(2));
	list.push(3);

	assert_eq!(list.data[0].val, 1);
	assert_eq!(list.data[0].next, None);
	assert_eq!(list.data[1].val, 3);
	assert_eq!(list.data[1].next, Some(0));
	assert_eq!(list.data.head, Some(1));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.len(), 2);

	let expected = vec![3,1];
	let mut i = 0;
	// for x in list {
	// 	assert_eq!(x, expected[i]);
	// 	i += 1;
	// }
}

#[test]
fn add_tail() {
	let mut list = LinkedList::<i32>::new();
	list.add_tail(1);
	assert_eq!(list.len(), 1);
	assert_eq!(list.data.head, Some(0));
	assert_eq!(list.tail, Some(0));
	list.push(2);
	assert_eq!(list.len(), 2);
	assert_eq!(list.data.head, Some(1));
	assert_eq!(list.tail, Some(0));
	list.add_tail(3);
	assert_eq!(list.len(), 3);
	assert_eq!(list.data.head, Some(1));
	assert_eq!(list.tail, Some(2));
}

// #[test]
// fn rem_tail() {
// 	let mut list = LinkedList::<i32>::new();
// 	list.add_tail(1);
// 	list.add_tail(2);
// 	list.add_tail(3);
// 	let mut i = 1;
// 	for a in list.clone() {
// 		assert_eq!(a, i);
// 		i += 1;
// 	}
// 	let x = list.remove_tail().unwrap();
// 	assert_eq!(x, 3);
// 	let x = list.remove_tail().unwrap();
// 	assert_eq!(x, 2);
// 	let x = list.remove_tail().unwrap();
// 	assert_eq!(x, 1);
// 	assert!(list.is_empty());
// }

#[test]
fn all_ops() {
	let mut list = LinkedList::<i32>::new();
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
	// 	assert_eq!(x, i);
	// 	i += 1;
	// }
}
