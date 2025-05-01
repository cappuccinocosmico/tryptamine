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
