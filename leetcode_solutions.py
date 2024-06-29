# Definition for singly-linked list.
from typing import Optional

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
class Solution_SumLinkedList:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        def safe_next(node : ListNode):
            if node.next is not None:
                return node.next
            node.next = ListNode()
            return node.next
        def does_continue(node : ListNode):
            return node.val != 0 or node.next is not None
        if not does_continue(l1) and not does_continue(l2):
            return ListNode()
        base = 10
        borrow = 0
        root_node = ListNode()
        result_current = root_node
        l1_current = l1
        l2_current = l2
        while does_continue(l1_current) or does_continue(l2_current) or borrow!=0:
            result_current = safe_next(result_current)
            result = (borrow+l1_current.val+l2_current.val ) % base
            result_current.val = result
            borrow = (borrow+l1_current.val+l2_current.val)  //10
            # print(borrow)
            l1_current = safe_next(l1_current)
            l2_current = safe_next(l2_current)
        return safe_next(root_node)
