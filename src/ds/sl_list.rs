/// This structure is for a single-linked-list implementation that does not use pointers
/// See datastructure implementation [here](https://opendatastructures.org/ods-python/3_1_SLList_Singly_Linked_Li.html)
pub struct SlList<T> {
	/// list items are stored in a vector with vector indexes used instead of pointers
	items: Vec<Node<T>>,
	/// index of the head of the list
	head: Option<usize>,
	/// index of the tail of the list
	tail: Option<usize>,
	/// index of elements that are free on the list
	free_list: Vec<usize>,
	/// number of elements in the list
	n: usize,
}

// region v List Iterators

/// This is the struct that keeps track of iterating a list without borrowing it (move)
/// # Examples
/// 
/// ```
/// let l = SlList::<i32>::new();
/// for item in l {
///     println!(item);
/// }
/// ```
pub struct SlListIter<T> {
	curr_idx: Option<usize>,
	list: SlList<T>,
}
/// Iterator for move list 
impl<T: Copy> Iterator for SlListIter<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let idx = match self.curr_idx {
			None => {
				return None
			},
			Some(i) => {
				i
			}
		};
		let item = self.list.items[idx].clone();
		self.curr_idx = item.next;
		Some(item.val)
	}
}
/// Produces the iterator for moved list
impl <T: std::marker::Copy> IntoIterator for SlList<T> {
	type Item = T;
	type IntoIter = SlListIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			curr_idx: self.head,
			list: self,
		}
	}
}
// region ^ List Iterators

impl<T: Copy> Default for SlList<T> {
	fn default() -> Self {
		Self { items: Default::default(), head: Default::default(), tail: Default::default(), free_list: Default::default(), n: Default::default() }
	}
}
impl<T:Copy> Clone for SlList<T> {
	fn clone(&self) -> Self {
		if self.is_empty() {
			return Self::new();
		}
		let mut retval = Self::new();
		let mut curr_idx = self.head.unwrap();
		loop {
			let mut n = self.items[curr_idx].clone();
			let at_end = n.next.is_none();
			if !at_end {
				curr_idx = n.next.unwrap();
				n.next = Some(retval.items.len()+1);
			}
			retval.items.push(n);
			if at_end {
				break;
			}
		}
		retval.n = retval.items.len();
		retval.head = Some(0);
		retval.tail = Some(retval.n-1);
		retval
	}
}

/// This struct wraps the value to be stored in the list and add a pointer to the next item
#[derive(Clone)]
struct Node<T> {
	val: T,
	next: Option<usize>,
}

/// Linked list methods
impl<T: Copy> SlList<T> {
	/// constructor
	pub fn new() -> Self {
		Self::default()
	}

	/// add a new value to the front of the list
	pub fn push(&mut self, item: T) {
		let idx = self.add_item(item, self.head);
		self.head = Some(idx);
		if self.n == 1 {
			self.tail = self.head;
		}
	}
	/// fetch and remove a value from the front of the list
	pub fn pop(&mut self) -> Option<T> {
		match self.head {
			None => None,
			Some(idx) => {
				let retval = self.items[idx].clone();
				self.head = retval.next;
				self.free_list.push(idx);
				self.n -= 1;
				if self.n == 0 {
					self.tail = None;
				}
				Some(retval.val)
			}
		}
	}
	/// add a new value to the end of a list
	pub fn add_tail(&mut self, item: T) {
		let idx = self.add_item(item, None);
		if let Some(tail_idx) = self.tail {
			self.items[tail_idx].next = Some(idx);
		}
		self.tail = Some(idx);
		if self.n == 1 {
			self.head = self.tail;
		}
	}
	/// fetch and remove a value from the back of the list
	pub fn remove_tail(&mut self) -> Option<T> {
		let tail_idx = match self.tail {
			None => return None,
			Some(i) => i
		};
		let result = self.items[tail_idx].val;
		if self.head == self.tail {
			self.head = None;
			self.tail = None;
			self.n = 0;
		} else {
			let mut new_tail: usize = self.head.unwrap();
			loop {
				let nxt_idx = match self.items[new_tail].next {
					None => panic!("list linkage is incorrect"),
					Some(i) => i
				};
				if nxt_idx == tail_idx {
					self.items[new_tail].next = None;
					break;
				}
				new_tail = nxt_idx;
			}
			self.tail = Some(new_tail);
			self.n -= 1;
		}
		self.free_list.push(tail_idx);
		Some(result)
	}

	pub fn len(&self) -> usize { self.n }
	pub fn is_empty(&self) -> bool { self.n == 0 }

	fn add_item(&mut self, item: T, nxt: Option<usize>) -> usize {
		let n : Node<T> = Node {
			val: item,
			next: nxt,
		};
		self.n += 1;
		match self.free_list.pop() {
			None => {
				self.items.push(n);
				self.items.len()-1
			},
			Some(idx) => {
				self.items[idx] = n;
				idx
			}
		}
	}
}

#[test]
fn empty() {
	let list = SlList::<i32>::new();
	assert_eq!(list.len(), 0);
	assert_eq!(list.head, None);
	assert_eq!(list.tail, None);
	for _ in list {
		assert!(false);
	}
}

#[test]
fn push() {
	let mut list = SlList::<i32>::new();
	list.push(1);
	assert_eq!(list.items.len(), 1);
	assert_eq!(list.items[0].val, 1);
	assert_eq!(list.items[0].next, None);
	assert_eq!(list.head, Some(0));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.len(), 1);

	list.push(2);
	assert_eq!(list.items.len(), 2);
	assert_eq!(list.items[0].val, 1);
	assert_eq!(list.items[0].next, None);
	assert_eq!(list.items[1].val, 2);
	assert_eq!(list.items[1].next, Some(0));
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.len(), 2);
	let mut i = 2;
	for x in list {
		assert_eq!(x, i);
		i -= 1;
	}
}

#[test]
fn pop() {
	let mut list = SlList::<i32>::new();
	list.push(1);
	list.push(2);
	let val = list.pop();
	assert_eq!(val, Some(2));
	assert_eq!(list.len(), 1);
	assert_eq!(list.head, Some(0));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.free_list, vec![1]);

	let val = list.pop();
	assert_eq!(val, Some(1));
	assert_eq!(list.len(), 0);
	assert_eq!(list.head, None);
	assert_eq!(list.tail, None);
	assert_eq!(list.free_list, vec![1,0]);

	let val = list.pop();
	assert_eq!(val, None);
}

#[test]
fn push_n_pop() {
	let mut list = SlList::<i32>::new();
	list.push(1);
	list.push(2);
	let val = list.pop();
	assert_eq!(val, Some(2));
	list.push(3);

	assert_eq!(list.items[0].val, 1);
	assert_eq!(list.items[0].next, None);
	assert_eq!(list.items[1].val, 3);
	assert_eq!(list.items[1].next, Some(0));
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(0));
	assert_eq!(list.len(), 2);

	let expected = vec![3,1];
	let mut i = 0;
	for x in list {
		assert_eq!(x, expected[i]);
		i += 1;
	}
}

#[test]
fn add_tail() {
	let mut list = SlList::<i32>::new();
	list.add_tail(1);
	assert_eq!(list.len(), 1);
	assert_eq!(list.head, Some(0));
	assert_eq!(list.tail, Some(0));
	list.push(2);
	assert_eq!(list.len(), 2);
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(0));
	list.add_tail(3);
	assert_eq!(list.len(), 3);
	assert_eq!(list.head, Some(1));
	assert_eq!(list.tail, Some(2));
}

#[test]
fn rem_tail() {
	let mut list = SlList::<i32>::new();
	list.add_tail(1);
	list.add_tail(2);
	list.add_tail(3);
	let mut i = 1;
	for a in list.clone() {
		assert_eq!(a, i);
		i += 1;
	}
	let x = list.remove_tail().unwrap();
	assert_eq!(x, 3);
	let x = list.remove_tail().unwrap();
	assert_eq!(x, 2);
	let x = list.remove_tail().unwrap();
	assert_eq!(x, 1);
	assert!(list.is_empty());
}

#[test]
fn all_ops() {
	let mut list = SlList::<i32>::new();
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
	for x in list {
		assert_eq!(x, i);
		i += 1;
	}
}
