use std::collections::VecDeque;

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
impl Solution {
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let Some(mut head) = head else { return None };
        let previous_head = &mut head;
        loop {
            let mut rev_head = previous_head.next.take();
            let mut tail = match &mut rev_head {
                Some(val) => val,
                None => {
                    previous_head.next = rev_head;
                    return Some(head);
                }
            };
            for _ in 0..k {
                let Some(new_tail) = &mut tail.next else {
                    previous_head.next = rev_head;
                    return Some(head);
                };
                tail = new_tail;
            }
            let mut next_head = tail.next.take();
            while let Some(mut new_head) = rev_head.take() {
                rev_head = new_head.next.take();
                // let new_head_next = new_head.next.take();
                new_head.next = next_head;
            }
        }
    }
}
