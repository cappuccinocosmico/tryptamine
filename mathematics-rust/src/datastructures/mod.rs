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

fn new_leaf<T: PartialOrd>(val: T) -> BinaryLeaf<T> {
    return Some(Box::new(BinaryNode {
        is_red: false,
        data: val,
        left: None,
        right: None,
    }));
}

fn rotate_node<T>(parent_leaf: &mut BinaryLeaf<T>, is_left_rotation: bool) -> Result<(), String> {
    if is_left_rotation {
        // Left Rotation Code
        let (mut stolen_parent, mut stolen_child) = match parent_leaf {
            None => return Err("Parent Node is Empty".to_string()),
            Some(parent) => {
                if parent.right.is_none() {
                    return Err("Child Right Node is empty".to_string());
                };
                let stolen_child = parent.right.take().unwrap();
                let stolen_parent = parent_leaf.take().unwrap();
                (stolen_parent, stolen_child)
            }
        };
        let transfer_child = stolen_child.left.take();
        stolen_parent.right = transfer_child;
        stolen_child.left = Some(stolen_parent);
        *parent_leaf = Some(stolen_child);
        return Ok(());
    }
    // Right Rotation Code
    let (mut stolen_parent, mut stolen_child) = match parent_leaf {
        None => return Err("Parent Node is Empty".to_string()),
        Some(parent) => {
            if parent.left.is_none() {
                return Err("Child Left Node is empty".to_string());
            };
            let stolen_child = parent.left.take().unwrap();
            let stolen_parent = parent_leaf.take().unwrap();
            (stolen_parent, stolen_child)
        }
    };
    let transfer_child = stolen_child.right.take();
    stolen_parent.left = transfer_child;
    stolen_child.right = Some(stolen_parent);
    *parent_leaf = Some(stolen_child);
    return Ok(());
}

impl<T: PartialOrd + Clone> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree { head: None }
    }
    fn insert(&mut self, insert: T) {
        match &mut self.head {
            None => {
                self.head = new_leaf(insert);
                return;
            }
            Some(val) => {
                return recursive_insert(val.as_mut(), insert);
            }
        }
        fn recursive_insert<T: PartialOrd>(head: &mut BinaryNode<T>, insert: T) {
            if head.data == insert {
                return;
            }
            if head.data < insert {
                match &mut head.left {
                    None => {
                        head.left = new_leaf(insert);
                        return;
                    }
                    Some(val) => {
                        return recursive_insert(val.as_mut(), insert);
                    }
                }
            }
            match &mut head.right {
                None => {
                    head.right = new_leaf(insert);
                    return;
                }
                Some(val) => {
                    return recursive_insert(val.as_mut(), insert);
                }
            }
        }
    }
    fn fetch(&self, element: T) -> Option<T> {
        return fetch_recursive(&self.head, element);
        fn fetch_recursive<T: PartialOrd + Clone>(node: &BinaryLeaf<T>, element: T) -> Option<T> {
            match &node {
                None => return None,
                Some(node) => {
                    if node.data == element {
                        return Some(node.data.clone());
                    };
                    if node.data < element {
                        return fetch_recursive(&node.left, element);
                    };
                    return fetch_recursive(&node.right, element);
                }
            }
        }
    }
}

// Test module for BinaryTree
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_fetch() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        // Fetching from empty tree returns None
        assert_eq!(tree.fetch(0), None);
    }

    #[test]
    fn test_single_insertion() {
        let mut tree = BinaryTree::new();
        tree.insert(42);
        assert_eq!(tree.fetch(42), Some(42));
        // Nonexistent element still returns None
        assert_eq!(tree.fetch(7), None);
    }

    #[test]
    fn test_multiple_insertions_ordered() {
        let mut tree = BinaryTree::new();
        for i in 1..11 {
            tree.insert(i);
        }
        // All inserted elements should be found
        for i in 1..11 {
            assert_eq!(tree.fetch(i), Some(i));
        }
        // Out of range values not inserted
        assert_eq!(tree.fetch(0), None);
        assert_eq!(tree.fetch(11), None);
    }

    #[test]
    fn test_multiple_insertions_random() {
        let mut tree = BinaryTree::new();
        let values = vec![5, 3, 8, 1, 4, 7, 9];
        for &v in &values {
            tree.insert(v);
        }
        // All inserted elements should be found
        for &v in &values {
            assert_eq!(tree.fetch(v), Some(v));
        }
        // Test duplicates: inserting again shouldn't break fetch
        tree.insert(3);
        tree.insert(8);
        assert_eq!(tree.fetch(3), Some(3));
        assert_eq!(tree.fetch(8), Some(8));
    }
}
