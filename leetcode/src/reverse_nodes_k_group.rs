// Given the head of a linked list, reverse the nodes of the list k at a time, and return the modified list.
//
// k is a positive integer and is less than or equal to the length of the linked list. If the number of nodes is not a multiple of k then left-out nodes, in the end, should remain as it is.
//
// You may not alter the values in the list's nodes, only nodes themselves may be changed.
//
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

fn pop_and_advance_linked_node(node: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut owned_node = node.take()?;
    let next_node = owned_node.next.take();
    *node = next_node;
    Some(owned_node)
}

fn advance_k_in_linked_list(
    begin_node: &mut Option<Box<ListNode>>,
    k: i32,
) -> Option<&mut Box<ListNode>> {
    let mut begin_ref = begin_node;
    for _ in 0..k {
        begin_ref = &mut begin_ref.as_mut()?.next
    }
    begin_ref.as_mut()
}

/// Takes in two linked lists, appends the first one onto the second, and returns an owned complete
/// list, and a reference to the cut point in the list. Modifies the root append node to be the
/// root of the new list with the appended items. And for convienence returns a reference to the
/// point in the list where the cut was made.
fn append_reverse_onto_linked_list(
    mut to_append: Option<Box<ListNode>>,
    root_append: &mut Option<Box<ListNode>>,
) -> &mut Option<Box<ListNode>> {
    let Some(mut first_append) = pop_and_advance_linked_node(&mut to_append) else {
        return root_append;
    };
    first_append.next = root_append.take();
    *root_append = Some(first_append);
    let cut_pointer = &raw mut root_append.as_mut().unwrap().next;
    while let Some(mut next_append) = pop_and_advance_linked_node(&mut to_append) {
        next_append.next = root_append.take();
        *root_append = Some(next_append);
    }
    unsafe {
        let return_ref = &mut (*cut_pointer);
        return return_ref;
    }
}
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head?;
        let mut previous_head = &mut head;
        loop {
            let mut rev_head = previous_head.next.take();
            let tail = match advance_k_in_linked_list(&mut rev_head, k) {
                Some(val) => val,
                None => {
                    previous_head.next = rev_head;
                    return Some(head);
                }
            };
            let mut append_root = tail.next.take();
            let tail_pointer = append_reverse_onto_linked_list(rev_head, &mut append_root);
            previous_head.next = append_root;
            previous_head = match tail_pointer {
                Some(val) => val,
                None => return Some(head),
            };
        }
    }
}
