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
        if k <= 1 {
            return head;
        }
        let k = k as usize;
        let mut head = head;
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy;

        while head.is_some() {
            // 1. Check if there are k nodes left
            let mut probe = &head;
            let mut count = 0;
            for _ in 0..k {
                if let Some(node) = probe {
                    probe = &node.next;
                    count += 1;
                } else {
                    break;
                }
            }

            if count == k {
                // 2. Reverse k nodes
                let mut group_head = head;
                let mut prev = None;
                for _ in 0..k {
                    if let Some(mut node) = group_head {
                        group_head = node.next.take();
                        node.next = prev;
                        prev = Some(node);
                    }
                }
                // `prev` is now the head of the reversed group.
                // `group_head` is the start of the rest of the list.

                // 3. Link the reversed group
                tail.as_mut().unwrap().next = prev;

                // 4. Move the tail to the end of the newly added group.
                while tail.as_ref().unwrap().next.is_some() {
                    tail = &mut tail.as_mut().unwrap().next;
                }
                head = group_head;
            } else {
                // Not enough nodes, append the rest and break.
                tail.as_mut().unwrap().next = head;
                break;
            }
        }

        dummy.unwrap().next
    }
}