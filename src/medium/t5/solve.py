from typing import List


class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        n = len(nums)
        result = []
        nums.sort()
        for i in range(0, n-2):
            first = nums[i]
            if i != 0 and nums[i] == nums[i-1]:
                continue
            if first + nums[i+1]+nums[i+2] > 0:
                return result
            if first + nums[n-1] + nums[n-2] < 0:
                continue
            j, k = i+1, n - 1
            while j < k:
                sum = first+nums[j]+nums[k]
                if sum > 0:
                    k -= 1
                elif sum < 0:
                    j += 1
                else:
                    result.append([first, nums[j], nums[k]])
                    while j < k and nums[j+1] == nums[j]:
                        j += 1
                    while j < k and nums[k-1] == nums[k]:
                        k -= 1
                    j += 1
                    k -= 1
        return result


nums = [-1, 0, 1, 2, -1, -4]
s = Solution()
s.threeSum(nums)
