/*
 * https://opendatastructures.org/ods-python/3_1_SLList_Singly_Linked_Li.html
 */

pub struct SlList<T> {
	items: Vec<Node<T>>,
	head: Option<usize>,
	tail: Option<usize>,
	free_list: Vec<usize>,
	n: usize,
}

impl <T: std::marker::Copy> IntoIterator for SlList<T> {
	type Item = T;
	type IntoIter = SlListIter1<T>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			curr_idx: self.head,
			list: self,
		}
	}
}
impl<'a,T: std::marker::Copy> IntoIterator for &'a SlList<T> {
	type Item = T;
	type IntoIter = SlListIter2<'a,T>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
			curr_idx: self.head,
			list: &self,
		}
	}
}
impl<'a,T: std::marker::Copy> IntoIterator for &'a mut SlList<T> {
	type Item = T;
	type IntoIter = SlListIter2<'a,T>;

	fn into_iter(self) -> Self::IntoIter {
		todo!()
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

pub struct SlListIter1<T> {
	curr_idx: Option<usize>,
	list: SlList<T>,
}
pub struct SlListIter2<'a, T> {
	curr_idx: Option<usize>,
	list: &'a SlList<T>,
}
impl<T: Copy> Iterator for SlListIter1<T> {
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
		return Some(item.val)
	}
}
impl<'a,T: Copy> Iterator for SlListIter2<'a,T> {
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
		return Some(item.val)
	}
}

#[derive(Clone)]
struct Node<T> {
	val: T,
	next: Option<usize>,
}

impl<T: Copy> SlList<T> {
	pub fn new() -> Self {
		Self {
			items: Vec::<Node<T>>::new(),
			head: None,
			tail: None,
			free_list: Vec::new(),
			n: 0,
		}
	}

	/// add a new value to the front of the list
	pub fn push(&mut self, item: T) {
		let n : Node<T> = Node {
			val: item,
			next: self.head,
		};
		let idx = match self.free_list.pop() {
			None => {
				self.items.push(n);
				self.items.len()-1
			},
			Some(idx) => {
				self.items[idx] = n;
				idx
			}
		};
		self.head = Some(idx);
		if self.n == 0 {
			self.tail = self.head;
		}
		self.n += 1;
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
		let n : Node<T> = Node {
			val: item,
			next: None,
		};
		let idx = match self.free_list.pop() {
			None => {
				self.items.push(n);
				self.items.len()-1
			},
			Some(idx) => {
				self.items[idx] = n;
				idx
			}
		};
		match self.tail {
			Some(tail_idx) => {
				self.items[tail_idx].next = Some(idx);
			},
			None => {}
		};
		self.tail = Some(idx);
		if self.n == 0 {
			self.head = self.tail;
		}
		self.n += 1;
	}
	/// fetch and remove a value from the back of the list
	pub fn remove_tail(&mut self) -> Option<T> {
		let tail_idx = match self.tail {
			None => return None,
			Some(i) => i
		};
		let result = self.items[tail_idx].val;
		if (self.head == self.tail) {
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
fn all_ops() {}