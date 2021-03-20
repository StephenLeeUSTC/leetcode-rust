/**
 * [150] Evaluate Reverse Polish Notation
 *
 * Evaluate the value of an arithmetic expression in <a href="http://en.wikipedia.org/wiki/Reverse_Polish_notation" target="_blank">Reverse Polish Notation</a>.
 * Valid operators are +, -, *, and /. Each operand may be an integer or another expression.
 * Note that division between two integers should truncate toward zero.
 * It is guaranteed that the given RPN expression is always valid. That means the expression would always evaluate to a result, and there will not be any division by zero operation.
 *  
 * Example 1:
 * 
 * Input: tokens = ["2","1","+","3","*"]
 * Output: 9
 * Explanation: ((2 + 1) * 3) = 9
 * 
 * Example 2:
 * 
 * Input: tokens = ["4","13","5","/","+"]
 * Output: 6
 * Explanation: (4 + (13 / 5)) = 6
 * 
 * Example 3:
 * 
 * Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
 * Output: 22
 * Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
 * = ((10 * (6 / (12 * -11))) + 17) + 5
 * = ((10 * (6 / -132)) + 17) + 5
 * = ((10 * 0) + 17) + 5
 * = (0 + 17) + 5
 * = 17 + 5
 * = 22
 * 
 *  
 * Constraints:
 * 
 * 	1 <= tokens.length <= 10^4
 * 	tokens[i] is either an operator: "+", "-", "*", or "/", or an integer in the range [-200, 200].
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/evaluate-reverse-polish-notation/
// discuss: https://leetcode.com/problems/evaluate-reverse-polish-notation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut res: i32;
        let mut stack : Vec<i32> = vec![];
        for token in tokens.iter() {
            match token.parse::<i32>() {
                Ok(num) => {
                    stack.push(num);
                }
                Err(_) => {
                    match &token[..] {
                        "+" => {
                            let a = stack.pop().unwrap();
                            let b = stack.pop().unwrap();
                            stack.push(b + a);
                        }
                        "-" => {
                            let a = stack.pop().unwrap();
                            let b = stack.pop().unwrap();
                            stack.push(b - a);
                        }
                        "*" => {
                            let a = stack.pop().unwrap();
                            let b = stack.pop().unwrap();
                            stack.push(b * a);
                        }
                        "/" => {
                            let a = stack.pop().unwrap();
                            let b = stack.pop().unwrap();
                            stack.push(b / a);
                        }
                        _ => panic!("What fuck token"),
                    } 
                }
            }
        }
        stack[0]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_150() {
        // Input: tokens = ["2","1","+","3","*"]
        // Output: 9
        let input_a = vec_string!("2","1","+","3","*");
        assert_eq!(Solution::eval_rpn(input_a), 9);

        let input_b = vec_string!("10","6","9","3","+","-11","*","/","*","17","+","5","+");
        assert_eq!(Solution::eval_rpn(input_b), 22);
    }
}
