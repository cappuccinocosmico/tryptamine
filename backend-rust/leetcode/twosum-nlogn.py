from typing import List


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        indexes = {}
        for i in range(0, len(nums)):
            val = nums[i]
            if indexes.get(val) is None:
                indexes[val] = [i]
            else:
                indexes[val] = indexes[val] + [i]

        def quicksort(to_sort: List[int]) -> List[int]:
            if len(to_sort) <= 1:
                return to_sort
            pivot = to_sort[0]
            before_piv = []
            after_piv = []
            for i in to_sort[1:]:
                if i <= pivot:
                    before_piv.append(i)
                else:
                    after_piv.append(i)
            return quicksort(before_piv) + [pivot] + quicksort(after_piv)

        sort_nums = quicksort(nums)
        print(sort_nums)
        begin_index = 0
        end_index = len(sort_nums) - 1

        def lookup_final_index(val1: int, val2: int):
            if val1 == val2:
                return (indexes[val1])[:2]
            print(indexes[val1])
            # (index[val2])[0]]

            return [(indexes[val1])[0], (indexes[val2])[0]]

        while begin_index <= end_index:
            print(begin_index)
            print(end_index)
            test = sort_nums[begin_index] + sort_nums[end_index]
            if test == target:
                return lookup_final_index(sort_nums[begin_index], sort_nums[end_index])
            if test < target:
                begin_index += 1
            if test > target:
                end_index -= 1
        raise Exception("Not found")
