/**
 * [1006] Clumsy Factorial
 *
 * Normally, the factorial of a positive integer n is the product of all positive integers less than or equal to n.  For example, factorial(10) = 10 * 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1.
 * 
 * We instead make a clumsy factorial: using the integers in decreasing order, we swap out the multiply operations for a fixed rotation of operations: multiply (*), divide (/), add (+) and subtract (-) in this order.
 * 
 * For example, clumsy(10) = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1.  However, these operations are still applied using the usual order of operations of arithmetic: we do all multiplication and division steps before any addition or subtraction steps, and multiplication and division steps are processed left to right.
 * 
 * Additionally, the division that we use is floor division such that 10 * 9 / 8 equals 11.  This guarantees the result is an integer.
 * 
 * <font face="sans-serif, Arial, Verdana, Trebuchet MS">Implement the </font>clumsy function as defined above: given an integer N, it returns the clumsy factorial of N.
 * 
 *  
 * 
 * Example 1:
 * 
 * 
 * Input: 4
 * Output: 7
 * Explanation: 7 = 4 * 3 / 2 + 1
 * 
 * 
 * Example 2:
 * 
 * 
 * Input: <span id="example-input-1-1">10
 * </span>Output: <span id="example-output-1">12
 * </span>Explanation: 12 = 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1
 * 
 * 
 *  
 * 
 * Note:
 * 
 * <ol>
 * 	1 <= N <= 10000
 * 	-2^31 <= answer <= 2^31 - 1  (The answer is guaranteed to fit within a 32-bit integer.)
 * </ol>
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/clumsy-factorial/
// discuss: https://leetcode.com/problems/clumsy-factorial/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here
use std::collections::HashMap;

impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        if n == 1 || n == 2 {
            return n;
        }
        let ops = vec!['*', '/', '+', '-'];
        let mut weights = HashMap::new();
        weights.insert('*', 10);
        weights.insert('/', 10);
        weights.insert('+', 1);
        weights.insert('-', 1);

        let mut op_stack: Vec<char> = Vec::new();
        let mut num_stack: Vec<i32> = Vec::new();

        let mut index = 0;

        for i in (1..n + 1).rev() {
            num_stack.push(i);
            println!("num_stack is {:?}", num_stack);
            if i == 1 {
                break;
            }

            let now = ops[index];
            let weight = weights[&now];
            while !op_stack.is_empty() && weight <= weights[op_stack.last().unwrap()] {
                let rhs = num_stack.pop().unwrap();
                let lhs = num_stack.pop().unwrap();
                let op = op_stack.pop().unwrap();
                let mut res = 0;
                match op {
                    '*' => res = lhs * rhs,
                    '/' => res = lhs / rhs,
                    '+' => res = lhs + rhs,
                    '-' => res = lhs - rhs,
                    _ => panic!("what fuck op"),
                }
                num_stack.push(res);
                println!("calculate {} {} {}, res is {}", lhs, op, rhs, res);
            }
            op_stack.push(now);
            index = (index + 1) % ops.len();
        }

        while !op_stack.is_empty() {
            let rhs = num_stack.pop().unwrap();
            let lhs = num_stack.pop().unwrap();
            let op = op_stack.pop().unwrap();
            let mut res = 0;
            match op {
                '*' => res = lhs * rhs,
                '/' => res = lhs / rhs,
                '+' => res = lhs + rhs,
                '-' => res = lhs - rhs,
                _ => panic!("what fuck op"),
            }
            num_stack.push(res);
            println!("calculate {} {} {}, res is {}", lhs, op, rhs, res);
        }

        num_stack[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1006() {
        // assert_eq!(Solution::clumsy(4), 4 * 3 / 2 + 1);
        // assert_eq!(Solution::clumsy(3), 3 * 2 / 1);
        // assert_eq!(Solution::clumsy(4), 5 * 4 / 3 + 2 - 1);
        assert_eq!(Solution::clumsy(10), 10 * 9 / 8 + 7 - 6 * 5 / 4 + 3 - 2 * 1);
    }
}
