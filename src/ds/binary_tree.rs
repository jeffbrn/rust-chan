use std::{fmt::Debug, usize};
use super::backing::{node::NodeData, list::ListData};

struct Node<T> {
    val: T,
    left: Option<usize>,
    right: Option<usize>,
}
impl<T: std::fmt::Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node").field("val", &self.val).field("left", &self.left).field("right", &self.right).finish()
    }
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

#[derive(Debug)]
pub struct BinaryTree<T> {
	/// tree items are stored in a vector with vector indexes used instead of pointers
    items: Vec<Node<T>>,
    /// index of the root node
    root: Option<usize>,
	/// index of elements that are free on the list
	free_list: Vec<usize>,
    /// number of items in tree
    n: usize,
}
impl<T> Default for BTree<T> {
    fn default() -> Self {
        Self { items: Default::default(), root: Default::default(), free_list: Default::default(), n: Default::default() }
    }
}


impl<T:Copy+PartialEq+PartialOrd> BTree<T> {
    pub fn new() -> Self {
        Self::default()
    }
    /// insert item in tree (duplicates not allowed) true if inserted, false if not
    pub fn insert(&mut self, item: T) -> bool {
        match self.root {
            Some(i) => {
                self.add_tree(i, item)
            },
            None => {
                // first item becomes the tree root
                let idx = self.add_item(item);
                self.root = Some(idx);
                true
            }
        }
    }
    pub fn count(&self) -> usize { self.n }
    pub fn is_empty(&self) -> bool { self.n == 0 }

    /// insert value into tree unless its a duplicate
    fn add_tree(&mut self, parent_idx: usize, item: T) -> bool {
        if self.items[parent_idx].val == item {
            false // no duplicates allowed
        } else if item < self.items[parent_idx].val {
            match self.items[parent_idx].left {
                None => {
                    // insert new node
                    let idx = self.add_item(item);
                    self.items[parent_idx].left = Some(idx);
                    true
                },
                Some(i) => self.add_tree(i, item) // keep walking
            }
        } else {
            match self.items[parent_idx].right {
                None => {
                    // insert new node
                    let idx = self.add_item(item);
                    self.items[parent_idx].right = Some(idx);
                    true
                },
                Some(i) => self.add_tree(i, item) // keep walking
            }
        }
    }

    /// allocate new node in datastructure
    fn add_item(&mut self, item: T) -> usize {
        let n = Node {
            val: item,
            left: None, right: None,
        };
        self.n += 1;
        match self.free_list.pop() {
            None => {
                // free node list is empty so add to end of array
                self.items.push(n);
                self.items.len()-1
            },
            Some(i) => {
                // use free node
                self.items[i] = n;
                i
            }
        }
    }

    // fn splice(&mut self, idx: usize) {
    //     let u = self.items[idx].clone();
    //     let s = if u.left.is_none() { u.right } else { u.left };
    //     if idx == 
    // }
}

pub struct BTreeIter<T> {
    curr_node_idx: Option<usize>,
	walk_stack: Vec<usize>,
    tree: BTree<T>,
}
impl <T: std::marker::Copy+std::fmt::Debug> IntoIterator for BTree<T> {
	type Item = T;
	type IntoIter = BTreeIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
            curr_node_idx: self.root,
			walk_stack: Vec::new(),
			tree: self,
		}
	}
}
impl<T: Copy+std::fmt::Debug> Iterator for BTreeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // loop result is the next value in the iterator
        loop {
            // look at the current node
            self.curr_node_idx = match self.curr_node_idx {
                Some(curr_idx) => {
                    // check the left branch of the current node
                    match self.tree.items[curr_idx].left {
                        Some(i) => {
                            // descend down the left branch
                            self.walk_stack.push(curr_idx); // remember the current node to come back to after walking the left branch
                            println!("go down left branch: curr = {i}; stk = {:?}", self.walk_stack);
                            Some(i) // current is now the left child
                        }
                        _ => {
                            // no left branch prep to output current value
                            let val = self.tree.items[curr_idx].val;
                            // see if there is a right branch
                            self.curr_node_idx = self.tree.items[curr_idx].right;
                            println!("go down right branch: curr = {:?}; stk = {:?}", self.curr_node_idx, self.walk_stack);
                            println!("next = {val:?}");
                            break Some(val); // get the iterator to return the current nodes value
                        }
                    }
                },
                _ => {
                    // no current node so check the descent stack
                    match self.walk_stack.pop() {
                        Some(i) => {
                            // prep to output the current value before trying to descend right branch
                            let val = self.tree.items[i].val;
                            self.curr_node_idx = self.tree.items[i].right;
                            println!("going back up the tree: curr = {:?}; stk = {:?}", self.curr_node_idx, self.walk_stack);
                            println!("next = {val:?}");
                            break Some(val);
                        },
                        _ => break None, // walked the whole tree
                    }
                }
            }
        }
    }
}

#[cfg(test)]
fn build_test_tree() -> BTree<i32> {
    let mut tree = BTree::<i32>::new();
    let vals = vec![7,3,11,1,5,9,13,4,6,8,12,14];
    for x in vals {
        assert!(tree.insert(x));
    }
    tree
}

#[test]
fn empty() {
    let tree = BTree::<i32>::new();
    assert_eq!(tree.root, None);
    assert_eq!(tree.count(), 0);
    assert!(tree.is_empty());
    let mut vals = Vec::new();
    vals.extend(tree);
    assert!(vals.is_empty());
}

#[test]
fn insert() {
    let mut tree = BTree::<i32>::new();
    assert!(tree.insert(5));
    assert!(tree.insert(3));
    assert!(tree.insert(8));
    assert!(!tree.insert(3));
    assert_eq!(tree.root.unwrap(), 0);
    assert_eq!(tree.count(), 3);
    let mut vals = Vec::new();
    vals.extend(tree);
    assert_eq!(vals, vec![3,5,8]);
}

#[test]
fn test_tree() {
    let tree = build_test_tree();
    let mut vals = Vec::new();
    vals.extend(tree);
    assert_eq!(vals, vec![1,3,4,5,6,7,8,9,11,12,13,14]);
}
