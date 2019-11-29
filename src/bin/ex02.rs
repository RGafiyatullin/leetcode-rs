fn main() -> () {
    let stdin = std::io::stdin();

    let mut input = String::new();
    let mut pattern = String::new();

    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut pattern).unwrap();

    let input = input.trim();
    let pattern = pattern.trim();

    println!(
        "{:?}",
        Solution::is_match(input.to_owned(), pattern.to_owned())
    );
}

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        use ex02::*;

        let pattern: Pattern = p.parse().expect("failed to parse the pattern");
        pattern.is_match(&s)
    }
}

mod ex02 {
    pub use pattern::Pattern;
    pub use str_view::StrView;

    mod str_view {
        pub struct StrView<'a> {
            slice: &'a str,
            pos: usize,
        }
        impl<'a> StrView<'a> {
            pub fn from_str_slice(slice: &'a str) -> StrView<'a> {
                Self { slice, pos: 0 }
            }
        }
    }

    mod pattern {
        #[derive(Debug)]
        enum Char {
            Any,
            Exact(char),
        }
        impl Char {
            fn is_match(&self, input: char) -> bool {
                match *self {
                    Self::Any => true,
                    Self::Exact(expected) => expected == input,
                }
            }
        }

        #[derive(Debug)]
        enum Quantifier {
            One,
            Any,
        }

        #[derive(Debug)]
        struct Node(Char, Quantifier);

        #[derive(Debug)]
        pub struct Pattern(Vec<Node>);

        impl std::str::FromStr for Pattern {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut acc = Vec::<Node>::with_capacity(s.len());

                for ch in s.chars() {
                    match ch {
                        '.' => acc.push(Node(Char::Any, Quantifier::One)),
                        '*' => match acc.pop() {
                            None => return Err("asterisk cannot start an expression".to_owned()),
                            Some(Node(ch, Quantifier::Any)) => {
                                return Err("cannot have two asterisks in a row".to_owned())
                            }
                            Some(Node(ch, Quantifier::One)) => acc.push(Node(ch, Quantifier::Any)),
                        },
                        ch => acc.push(Node(Char::Exact(ch), Quantifier::One)),
                    }
                }

                Ok(Self(acc))
            }
        }

        impl Pattern {
            pub fn is_match(&self, input: &str) -> bool {
                unimplemented!()
            }
        }
    }
}
