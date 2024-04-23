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

pub struct BinaryTree<T:Copy> {
	data: ListData<Node<T>>,
	root: Option<usize>,
}


impl<T:Copy+PartialEq+PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            data: ListData::new(),
            root: None,
        }
    }
    /// insert item in tree (duplicates not allowed) true if inserted, false if not
    pub fn insert(&mut self, item: T) -> bool {
        match self.root {
            Some(i) => {
                self.add_tree(i, item)
            },
            None => {
                // first item becomes the tree root
                let n = Node { val:item, left:None, right:None };
                let idx = self.data.add_item(n);
                self.root = Some(idx);
                true
            }
        }
    }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn is_empty(&self) -> bool { self.data.len() == 0 }

    /// insert value into tree unless its a duplicate
    fn add_tree(&mut self, parent_idx: usize, item: T) -> bool {
        let n = Node { val:item, left:None, right:None };
        if self.data[parent_idx].val == item {
            false // no duplicates allowed
        } else if item < self.data[parent_idx].val {
            match self.data[parent_idx].left {
                None => {
                    // insert new node
                    let idx = self.data.add_item(n);
                    self.data[parent_idx].left = Some(idx);
                    true
                },
                Some(i) => self.add_tree(i, item) // keep walking
            }
        } else {
            match self.data[parent_idx].right {
                None => {
                    // insert new node
                    let idx = self.data.add_item(n);
                    self.data[parent_idx].right = Some(idx);
                    true
                },
                Some(i) => self.add_tree(i, item) // keep walking
            }
        }
    }
}

pub struct BinaryTreeIter<T:Copy> {
    curr_node_idx: Option<usize>,
	walk_stack: Vec<usize>,
    tree: BinaryTree<T>,
}
impl <T: std::marker::Copy+std::fmt::Debug> IntoIterator for BinaryTree<T> {
	type Item = T;
	type IntoIter = BinaryTreeIter<T>;

	fn into_iter(self) -> Self::IntoIter {
		Self::IntoIter {
            curr_node_idx: self.root,
			walk_stack: Vec::new(),
			tree: self,
		}
	}
}
impl<T: Copy+std::fmt::Debug> Iterator for BinaryTreeIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // loop result is the next value in the iterator
        loop {
            // look at the current node
            self.curr_node_idx = match self.curr_node_idx {
                Some(curr_idx) => {
                    // check the left branch of the current node
                    match self.tree.data[curr_idx].left {
                        Some(i) => {
                            // descend down the left branch
                            self.walk_stack.push(curr_idx); // remember the current node to come back to after walking the left branch
                            println!("go down left branch: curr = {i}; stk = {:?}", self.walk_stack);
                            Some(i) // current is now the left child
                        }
                        _ => {
                            // no left branch prep to output current value
                            let val = self.tree.data[curr_idx].val;
                            // see if there is a right branch
                            self.curr_node_idx = self.tree.data[curr_idx].right;
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
                            let val = self.tree.data[i].val;
                            self.curr_node_idx = self.tree.data[i].right;
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
fn build_test_tree() -> BinaryTree<i32> {
    let mut tree = BinaryTree::<i32>::new();
    let vals = vec![7,3,11,1,5,9,13,4,6,8,12,14];
    for x in vals {
        assert!(tree.insert(x));
    }
    tree
}

#[test]
fn empty() {
    let tree = BinaryTree::<i32>::new();
    assert_eq!(tree.root, None);
    assert_eq!(tree.len(), 0);
    assert!(tree.is_empty());
    let mut vals = Vec::new();
    vals.extend(tree);
    assert!(vals.is_empty());
}

#[test]
fn insert() {
    let mut tree = BinaryTree::<i32>::new();
    assert!(tree.insert(5));
    assert!(tree.insert(3));
    assert!(tree.insert(8));
    assert!(!tree.insert(3));
    assert_eq!(tree.root.unwrap(), 0);
    assert_eq!(tree.len(), 3);
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
