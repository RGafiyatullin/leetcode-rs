pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        assert!(n <= 8);
        assert!(n >= 1);

        let mut out = Default::default();
        generate(&mut out, LL::nil(), n as usize, 0);
        out
    }
}

use std::rc::Rc;

fn generate(out: &mut Vec<String>, prefix: Rc<LL>, to_open: usize, to_close: usize) {
    if let Some(to_close) = to_close.checked_sub(1) {
        generate(out, prefix.cons(Paren::Close), to_open, to_close);
    }
    if let Some(to_open) = to_open.checked_sub(1) {
        generate(out, prefix.cons(Paren::Open), to_open, to_close + 1);
    }

    if to_open == 0 && to_close == 0 {
        out.push(prefix.render());
    }
}

enum Paren {
    Open,
    Close,
}

enum LL {
    Nil,
    Cons(Paren, Rc<Self>),
}

impl LL {
    fn nil() -> Rc<Self> {
        Rc::new(Self::Nil)
    }
    fn cons(self: &Rc<Self>, paren: Paren) -> Rc<Self> {
        Rc::new(Self::Cons(paren, Rc::clone(self)))
    }
    fn render(&self) -> String {
        let mut out = String::new();
        self.render_impl(&mut out);
        out
    }
    fn render_impl(&self, acc: &mut String) {
        match self {
            Self::Nil => (),
            Self::Cons(Paren::Open, next) => {
                acc.push(')');
                next.render_impl(acc);
            },
            Self::Cons(Paren::Close, next) => {
                acc.push('(');
                next.render_impl(acc);
            },
        }
    }
}
