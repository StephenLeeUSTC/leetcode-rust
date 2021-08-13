/**
 * [786] K-th Smallest Prime Fraction
 *
 * You are given a sorted integer array arr containing 1 and prime numbers, where all the integers of arr are unique. You are also given an integer k.
 * For every i and j where 0 <= i < j < arr.length, we consider the fraction arr[i] / arr[j].
 * Return the k^th smallest fraction considered. Return your answer as an array of integers of size 2, where answer[0] == arr[i] and answer[1] == arr[j].
 *  
 * Example 1:
 * 
 * Input: arr = [1,2,3,5], k = 3
 * Output: [2,5]
 * Explanation: The fractions to be considered in sorted order are:
 * 1/5, 1/3, 2/5, 1/2, 3/5, and 2/3.
 * The third fraction is 2/5.
 * 
 * Example 2:
 * 
 * Input: arr = [1,7], k = 1
 * Output: [1,7]
 * 
 *  
 * Constraints:
 * 
 * 	2 <= arr.length <= 1000
 * 	1 <= arr[i] <= 3 * 10^4
 * 	arr[0] == 1
 * 	arr[i] is a prime number for i > 0.
 * 	All the numbers of arr are unique and sorted in strictly increasing order.
 * 	1 <= k <= arr.length * (arr.length - 1) / 2
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/k-th-smallest-prime-fraction/
// discuss: https://leetcode.com/problems/k-th-smallest-prime-fraction/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut left = 0_f64;
        let mut right= 1_f64;
        let mut cand = 0_f64;

        while left <= right {
            cand = (left + right) / 2_f64;

            // println!("left is {}, right is {}, mid is {}", left, right, cand);
            // let index = Self::check(&arr, cand);

            let mut p = 0;
            let mut q = 0;
            let mut max_value = 0_f64;
            let mut count = 0;
            
            for i in 0..arr.len() {
                for j in (i + 1..arr.len()).rev() {
                    let tmp = arr[i] as f64 / arr[j] as f64;
                    if tmp > cand {
                        break;
                    } else {
                        if tmp > max_value {
                            p = i;
                            q = j;
                            max_value = tmp;
                        } 
                        count += 1;
                    }
                }
            }
            // 小于等于 k 的分数个数为 n，则 k 一定为第 n 小的分数;

            // println!("the number small than mid is {}", count);
            if count < k {
                left = cand;
            } else if count > k {
                right = cand;
            } else {
                return vec![arr[p], arr[q]];
            }
        }

        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_786_0() {
        // arr = [1,2,3,5], k = 3
        assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 2, 3, 5], 3), vec![2, 5]);
    }

    #[test]
    fn test_786_1() {
        // arr = [1,2,3,5], k = 3
        assert_eq!(Solution::kth_smallest_prime_fraction(vec![1, 5], 1), vec![1, 5]);
    }
}
