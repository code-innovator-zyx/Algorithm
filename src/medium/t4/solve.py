from re import T
from typing import List
class Solution:
    def maxArea(self, height: List[int]) -> int:
        max_area,min_h,weight =0,0,0
        i,j= 0,len(height)-1
        while i<j:
            weight =j-i
            if height[i]<height[j]:
                min_h=height[i]
                i+=1
            else:
                min_h=height[j]
                j-=1
            tmp = weight*min_h
            max_area = max_area if tmp<max_area else tmp
        return max_area
