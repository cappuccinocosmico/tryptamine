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

            let mut next_head = tail.next.take();
            let mut build_reverse_ref = &mut next_head;
            while let Some(mut poped_node) = pop_and_advance_linked_node(&mut build_reverse_ref) {
                poped_node.next = next_head;
                next_head = Some(poped_node);
            }
            previous_head.next = next_head;
            // previous_head = tail
        }
    }
}
