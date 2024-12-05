from typing import List


class Solution:
    def minSubArrayLen(self, target: int, nums: List[int]) -> int:
        start = 0
        size = 2**31-1
        sum = 0
        for end in range(start, len(nums)):
            sum += nums[end]
            while sum >= target:
                size = min(size, end-start+1)
                sum -= nums[start]
                start += 1

        if size == 2**31-1:
            size = 0
        return size
