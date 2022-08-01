pub struct Solution;
#[cfg(test)]
mod tests;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack: Vec<P> = vec![];
        let mut output: P = Default::default();

        for ch in s.chars() {
            match ch {
                '(' => {
                    stack.push(Default::default());
                }
                ')' => {
                    let last = stack.pop().expect("Unbalanced parenthesis!");
                    if let Some(prev) = stack.last_mut() {
                        prev.push(last);
                    } else {
                        output.push(last)
                    }
                }

                illegal => panic!("Illegal char: {:?}", illegal),
            }
        }

        if output.is_empty() {
            "".to_owned()
        } else {
            let mut s = String::new();
            for item in output.into_vec() {
                item.render(&mut s);
            }
            s
        }
    }
}

#[derive(Debug, Clone)]
struct P(Vec<Self>);

impl Default for P {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl P {
    fn push(&mut self, other: Self) {
        self.0.push(other);
    }
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn render(&self, out_string: &mut String) {
        for sub in &self.0 {
            out_string.push('(');
            sub.render(out_string);
            out_string.push(')');
        }
    }

    fn into_vec(self) -> Vec<Self> {
        self.0
    }
}
