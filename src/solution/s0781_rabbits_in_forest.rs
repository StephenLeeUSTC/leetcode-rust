
/**
 * [781] Rabbits in Forest
 *
 * In a forest, each rabbit has some color. Some subset of rabbits (possibly all of them) tell you how many other rabbits have the same color as them. Those answers are placed in an array.
 * 
 * Return the minimum number of rabbits that could be in the forest.
 * 
 * 
 * Examples:
 * Input: answers = [1, 1, 2]
 * Output: 5
 * Explanation:
 * The two rabbits that answered "1" could both be the same color, say red.
 * The rabbit than answered "2" can't be red or the answers would be inconsistent.
 * Say the rabbit that answered "2" was blue.
 * Then there should be 2 other blue rabbits in the forest that didn't answer into the array.
 * The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn't.
 * 
 * Input: answers = [10, 10, 10]
 * Output: 11
 * 
 * Input: answers = []
 * Output: 0
 * 
 * 
 * Note:
 * 
 * <ol>
 * 	answers will have length at most 1000.
 * 	Each answers[i] will be an integer in the range [0, 999].
 * </ol>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rabbits-in-forest/
// discuss: https://leetcode.com/problems/rabbits-in-forest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut nn = HashMap::new();
        for num in answers.iter() {
            let counter = nn.entry(num).or_insert(0);
            *counter += 1;
        }
        let mut res = 0;
        for (k, v ) in nn {
            res += ((v + *k) / (*k + 1)) * (*k + 1);
        }
        res
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_781() {
        let input = vec![1, 1, 2];
        assert_eq!(Solution::num_rabbits(input), 5);

        let input2 = vec![10, 10, 10];
        assert_eq!(Solution::num_rabbits(input2), 11);

        let input3 = vec![];
        assert_eq!(Solution::num_rabbits(input3), 0);
    }
}

