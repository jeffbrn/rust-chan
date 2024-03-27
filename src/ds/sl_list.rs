/*
 * https://opendatastructures.org/ods-python/3_1_SLList_Singly_Linked_Li.html
 */

struct SlList<T> {
	items: Vec<Node<T>>,
	head: Option<usize>,
	tail: Option<usize>,
}

struct Node<T> {
	val: T,
	next: Option<usize>,
}

impl<T: Copy> SlList<T> {
	fn new() -> Self {
		Self {
			items: Vec::<Node<T>>::new(),
			head: None,
			tail: None,
		}
	}

	fn push(&mut self, item: T) {
		let n : Node<T> = Node {
			val: item,
			next: self.head,
		};
		self.head = Some(self.items.len());
		self.items.push(n);
		if self.items.len() == 1 {
			self.tail = Some(0);
		}
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

	list.push(2);
	assert_eq!(list.items.len(), 2);
	assert_eq!(list.items[0].val, 1);
	assert_eq!(list.items[0].next, Some(1));
	assert_eq!(list.head, Some(0));
	assert_eq!(list.tail, Some(0));
}
