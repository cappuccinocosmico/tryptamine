///
use std::collections::BTreeMap;
/// A simple binary search tree implementation with invert logic: left subtree holds greater values, right holds smaller.
pub struct BinaryTree<T: PartialOrd> {
    head: BinaryLeaf<T>,
}

struct BinaryNode<T> {
    is_red: bool,
    data: T,
    left: BinaryLeaf<T>,
    right: BinaryLeaf<T>,
}

type BinaryLeaf<T> = Option<Box<BinaryNode<T>>>;

fn new_leaf<T: PartialOrd>(data: T, is_red: bool) -> BinaryLeaf<T> {
    Some(Box::new(BinaryNode {
        is_red,
        data,
        left: None,
        right: None,
    }))
}

fn rotate_node<T>(parent_leaf: &mut BinaryLeaf<T>, is_left: bool) -> Result<(), String> {
    if is_left {
        // Left rotation
        let (mut parent_box, mut child_box) = match parent_leaf.take() {
            None => return Err("Parent is empty".into()),
            Some(mut p) => {
                let c = p.right.take().ok_or("Right child is empty")?;
                (p, c)
            }
        };
        parent_box.right = child_box.left.take();
        child_box.left = Some(parent_box);
        *parent_leaf = Some(child_box);
        Ok(())
    } else {
        // Right rotation
        let (mut parent_box, mut child_box) = match parent_leaf.take() {
            None => return Err("Parent is empty".into()),
            Some(mut p) => {
                let c = p.left.take().ok_or("Left child is empty")?;
                (p, c)
            }
        };
        parent_box.left = child_box.right.take();
        child_box.right = Some(parent_box);
        *parent_leaf = Some(child_box);
        Ok(())
    }
}

impl<T: PartialOrd + Clone> BinaryTree<T> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        Self { head: None }
    }

    /// Inserts a value. No balancing performed; first node is black by default.
    pub fn insert(&mut self, value: T) {
        fn rec<T: PartialOrd>(node: &mut BinaryNode<T>, v: T) {
            if node.data == v {
                return;
            }
            if node.data < v {
                // larger goes left
                match node.left.as_mut() {
                    None => node.left = new_leaf(v, true),
                    Some(n) => rec(n.as_mut(), v),
                }
            } else {
                // smaller goes right
                match node.right.as_mut() {
                    None => node.right = new_leaf(v, true),
                    Some(n) => rec(n.as_mut(), v),
                }
            }
        }
        match self.head.as_mut() {
            None => self.head = new_leaf(value, false),
            Some(root) => rec(root.as_mut(), value),
        }
    }

    /// Fetches a clone of the element if present.
    pub fn fetch(&self, value: &T) -> Option<T> {
        fn rec<T: PartialOrd + Clone>(node: &BinaryLeaf<T>, v: &T) -> Option<T> {
            match node {
                None => None,
                Some(n) => {
                    if n.data == *v {
                        Some(n.data.clone())
                    } else if n.data < *v {
                        // go left for greater
                        rec(&n.left, v)
                    } else {
                        // go right for smaller
                        rec(&n.right, v)
                    }
                }
            }
        }
        rec(&self.head, value)
    }

    /// Deletes an element, returning it if found.
    pub fn delete(&mut self, value: &T) -> Option<T> {
        fn rec<T: PartialOrd + Clone>(node: &mut BinaryLeaf<T>, v: &T) -> Option<T> {
            let mut removed = None;
            if let Some(mut boxed) = node.take() {
                if &boxed.data == v {
                    removed = Some(v.clone());
                    // Cases
                    let mut taken_left = boxed.left.take();
                    let mut taken_right = boxed.right.take();
                    match (&mut taken_left, &mut taken_right) {
                        (None, None) => *node = None,
                        (Some(_), None) => *node = taken_left,
                        (None, Some(_)) => *node = taken_right,
                        (Some(l), Some(_r)) => {
                            // both children: find successor = smallest in left subtree (since left has greater values)
                            let succ_data;
                            {
                                let mut cur = l;
                                while let Some(ref mut right) = cur.right {
                                    cur = right;
                                }
                                succ_data = cur.data.clone();
                            }
                            // delete successor
                            let _ = rec(&mut taken_left, &succ_data);
                            boxed.data = succ_data;
                            boxed.left = taken_left;
                            boxed.right = taken_right;
                            *node = Some(boxed);
                        }
                    }
                } else if &boxed.data < v {
                    // go left
                    let res = rec(&mut boxed.left, v);
                    *node = Some(boxed);
                    return res;
                } else {
                    // go right
                    let res = rec(&mut boxed.right, v);
                    *node = Some(boxed);
                    return res;
                }
            }
            removed
        }
        rec(&mut self.head, value)
    }
}

enum TreeRef<'a, T> {
    NotPushed(&'a BinaryNode<T>),
    Pushed(&'a BinaryNode<T>),
}

pub struct BinaryTreeIterator<'a, T> {
    tree_stack: Vec<TreeRef<'a, T>>,
}

impl<'a, T: PartialOrd + Clone> Iterator for BinaryTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(direction) = self.tree_stack.pop() {
            match direction {
                TreeRef::NotPushed(node) => {
                    match node.left.as_deref() {
                        None => {
                            self.tree_stack.push(TreeRef::Pushed(node));
                            return Some(&node.data);
                        }
                        Some(next_leftward) => {
                            self.tree_stack.push(TreeRef::NotPushed(node))
                            // Decend again until you get to the bottom leftmost node you havent
                            // visited yet
                        }
                    }
                }
                TreeRef::Pushed(node) => {
                    match node.right.as_deref() {
                        None => {
                            // Pop nodes off the vec until you get to a NotPushed node, then pop it
                            // off and mark it as pushed and return that value. If there are no non
                            // pushed values return nothing
                        }
                        Some(next_rightward) => {
                            self.tree_stack.push(TreeRef::NotPushed(next_rightward))
                            // Once at this point you can go back to treating the tree as a regular
                            // not pushed node and begin descending left until you hit a value,
                            // push that and then mark it as pushed
                        }
                    }
                }
            }
        }
        None
    }
}

impl<'a, T: PartialOrd + Clone> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = BinaryTreeIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut tree_stack = Vec::new();

        if let Some(root) = self.head.as_deref() {
            tree_stack.push(TreeRef::NotPushed(root));
        }

        BinaryTreeIterator { tree_stack }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_fetch() {
        let mut t = BinaryTree::new();
        t.insert(5);
        t.insert(3);
        t.insert(7);
        assert_eq!(t.fetch(&5), Some(5));
        assert_eq!(t.fetch(&3), Some(3));
        assert_eq!(t.fetch(&7), Some(7));
        assert_eq!(t.fetch(&8), None);
    }

    #[test]
    fn test_delete_leaf() {
        let mut t = BinaryTree::new();
        t.insert(10);
        assert_eq!(t.delete(&10), Some(10));
        assert_eq!(t.fetch(&10), None);
    }

    #[test]
    fn test_delete_one_child() {
        let mut t = BinaryTree::new();
        t.insert(10);
        t.insert(5);
        assert_eq!(t.delete(&10), Some(10));
        assert_eq!(t.fetch(&5), Some(5));
    }

    #[test]
    fn test_delete_two_children() {
        let mut t = BinaryTree::new();
        t.insert(10);
        t.insert(5);
        t.insert(15);
        t.insert(12);
        assert_eq!(t.delete(&10), Some(10));
        // successor of 10 is smallest in left (greater) subtree = 12
        assert_eq!(t.fetch(&12), Some(12));
        assert_eq!(t.fetch(&10), None);
    }
}
