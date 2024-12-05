from typing import List
class Solution:
    def twoSum(self, numbers: List[int], target: int) -> List[int]:
        i,j=0,len(numbers)-1
        while i<j:
            tmp =numbers[i]+numbers[j]
            if tmp==target:
                break
            elif tmp>target:
                j-=1
            else:
                i+=1
        return [i+1,j+1]
