class Solution(object):
    def merge(self, nums1, m, nums2, n):
        """
        :type nums1: List[int]
        :type m: int
        :type nums2: List[int]
        :type n: int
        :rtype: None Do not return anything, modify nums1 in-place instead.
        """
        pm = m-1
        pn=n-1 
        p = m+n-1
        while pm>=0 and pn>=0:
            if nums1[pm]>nums2[pn]: 
                nums1[p] = nums1[pm]
                pm -=1
            else:
                nums1[p] = nums2[pn]
                pn-=1
            p-=1
        

        for i in range (0,pn+1):
            nums1[i] = nums2[i]




nums1 = [1,2,3,0,0,0]
m = 3
nums2 = [4,5,6]
n = 3
s =Solution()
s.merge(nums1,m,nums2,n)
print(nums1)