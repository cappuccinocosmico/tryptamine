pub struct BinaryTree<T: PartialOrd> {
    head: BinaryLeaf<T>,
}

struct BinaryTreeNode<T> {
    is_red: bool,
    data: T,
    left: BinaryLeaf<T>,
    right: BinaryLeaf<T>,
}
type BinaryLeaf<T> = Option<Box<BinaryTreeNode<T>>>;

fn new_leaf<T: PartialOrd>(val: T) -> BinaryLeaf<T> {
    return Some(Box::new(BinaryTreeNode {
        is_red: false,
        data: val,
        left: None,
        right: None,
    }));
}

impl<T: PartialOrd> BinaryTree<T> {
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
        fn recursive_insert<T: PartialOrd>(head: &mut BinaryTreeNode<T>, insert: T) {
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
}
