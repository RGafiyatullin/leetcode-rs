pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = vec![];

        for t in tokens.iter() {
            if let Ok(value) = <i32 as std::str::FromStr>::from_str(t) {
                stack.push(value);
            } else {
                match t.as_str() {
                    "+" => pop_apply_push(&mut stack, std::ops::Add::add),
                    "-" => pop_apply_push(&mut stack, std::ops::Sub::sub),
                    "*" => pop_apply_push(&mut stack, std::ops::Mul::mul),
                    "/" => pop_apply_push(&mut stack, std::ops::Div::div),

                    unsupported => panic!("unsupported operator {:?}", unsupported),
                }
            }
        }

        assert_eq!(stack.len(), 1, "bad stack: {:?}", stack);
        stack.pop().expect("come on!")
    }
}

fn pop_apply_push(stack: &mut Vec<i32>, op: impl Fn(i32, i32) -> i32) {
    let right = stack.pop().expect("stack underrun");
    let left = stack.pop().expect("stack underrun");
    stack.push(op(left, right));
}
