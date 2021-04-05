/**
 * [88] Merge Sorted Array
 *
 * Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one sorted array.
 * The number of elements initialized in nums1 and nums2 are m and n respectively. You may assume that nums1 has a size equal to m + n such that it has enough space to hold additional elements from nums2.
 *  
 * Example 1:
 * Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
 * Output: [1,2,2,3,5,6]
 * Example 2:
 * Input: nums1 = [1], m = 1, nums2 = [], n = 0
 * Output: [1]
 *  
 * Constraints:
 * 
 * 	nums1.length == m + n
 * 	nums2.length == n
 * 	0 <= m, n <= 200
 * 	1 <= m + n <= 200
 * 	-10^9 <= nums1[i], nums2[i] <= 10^9
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/merge-sorted-array/
// discuss: https://leetcode.com/problems/merge-sorted-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let t_num = nums1.to_vec();
        let mut i1 : usize = 0;
        let mut i2 : usize = 0;
        let mut i0 : usize= 0;
        let mm = m as usize;
        let nn = n as usize;

        while i0 < nums1.len() {
            if i1 >= mm  && i2 < nn {
                nums1[i0] = nums2[i2];
                i2 += 1;
            } else if i2 >= nn && i1 < mm {
                nums1[i0] = t_num[i1];
                i1 += 1;
            } else if i2 >= nn && i1 >= mm {
                return;
            } else {
                if t_num[i1] <= nums2[i2] {
                    nums1[i0] = t_num[i1];
                    i1 += 1;
                } else {
                    nums1[i0] = nums2[i2];
                    i2 += 1;
                }
            }
            i0 += 1;
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_88() {
        let mut nums1 : Vec<i32> = vec![1,2,3,0,0,0];
        let m = 3;
        let mut nums2 : Vec<i32> = vec![2,5,6];
        let n = 3;
        Solution::merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1,2,2,3,5,6]);
    }
}
