from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        def create_combined_sorted(list1, list2):
            if min(len(list1), len(list2)) == 0:
                return list1 + list2
            if list1[0] <= list2[0]:
                return [list1[0]] + create_combined_sorted(list1[1:], list2)
            else:
                return [list2[0]] + create_combined_sorted(list1, list2[1:])

        combined = create_combined_sorted(nums1, nums2)
        if len(combined) % 2 == 1:
            return combined[len(combined) // 2]
        else:
            return (combined[len(combined) // 2] + combined[len(combined) // 2 - 1]) / 2
