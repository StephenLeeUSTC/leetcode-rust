
/**
 * [692] Top K Frequent Words
 *
 * Given a non-empty list of words, return the k most frequent elements.
 * Your answer should be sorted by frequency from highest to lowest. If two words have the same frequency, then the word with the lower alphabetical order comes first.
 * 
 * Example 1:<br />
 * 
 * Input: ["i", "love", "leetcode", "i", "love", "coding"], k = 2
 * Output: ["i", "love"]
 * Explanation: "i" and "love" are the two most frequent words.
 *     Note that "i" comes before "love" due to a lower alphabetical order.
 * 
 * 
 * 
 * Example 2:<br />
 * 
 * Input: ["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"], k = 4
 * Output: ["the", "is", "sunny", "day"]
 * Explanation: "the", "is", "sunny" and "day" are the four most frequent words,
 *     with the number of occurrence being 4, 3, 2 and 1 respectively.
 * 
 * 
 * 
 * Note:<br>
 * <ol>
 * You may assume k is always valid, 1 &le; k &le; number of unique elements.
 * Input words contain only lowercase letters.
 * </ol>
 * 
 * 
 * Follow up:<br />
 * <ol>
 * Try to solve it in O(n log k) time and O(n) extra space.
 * </ol>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/top-k-frequent-words/
// discuss: https://leetcode.com/problems/top-k-frequent-words/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Pair<'a>(&'a str, i32);

impl<'a> Ord for Pair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl<'a> PartialOrd for Pair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut m = HashMap::new();
        for word in words.iter() {
            let counter = m.entry(word.as_str()).or_insert(0);
            *counter += 1;
        }
        // println!("m is {:?}", m);

        let mut heap = BinaryHeap::new();
        
        let mut count = k as usize;
        // println!("count is {}", count);
        for (w, c) in m.iter() {
            heap.push(Pair(*w, *c));
            if heap.len() > count {
                let _ = heap.pop();
            }
        }

        let mut res = vec!["".to_string(); count];
        while count != 0 {
            let e = heap.pop();
            res[count - 1] = e.unwrap().0.to_string();
            count -= 1;
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_692() {
        assert_eq!(Solution::top_k_frequent(vec_string!("i", "love", "leetcode", "i", "love", "coding"), 2), vec_string!("i", "love"));
        assert_eq!(Solution::top_k_frequent(vec_string!("the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"), 4), vec_string!("the", "is", "sunny", "day"));
    }
}
