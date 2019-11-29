fn main() -> () {
    let stdin = std::io::stdin();
    let mut input = String::new();
    let mut pattern = String::new();
    stdin.read_line(&mut input).unwrap();
    stdin.read_line(&mut pattern).unwrap();
    let pattern = Pattern::from(pattern.trim());
    println!("pattern: {:?}", pattern);
    println!("{:?}", pattern.is_match(&input.trim()));
}

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let pattern = Pattern::from(&p);
        pattern.is_match(&s)
    }
}

trait CharMatch {
    fn is_match(&self, ch: char) -> bool;
}
impl CharMatch for Option<char> {
    fn is_match(&self, ch: char) -> bool {
        self.is_none() || *self == Some(ch)
    }
}

#[derive(Debug)]
enum Node {
    Char(Option<char>),
    Asterisk(Option<char>),
}

#[derive(Debug)]
struct Pattern {
    nodes: Vec<Node>,
}

impl Pattern {
    fn is_match(&self, input: &str) -> bool {
        let mut nodes = self.nodes.iter();
        let mut chars = input.chars();

        let mut state = State::Empty;

        loop {
            match state {
                State::Empty => {
                    if let Some(node) = nodes.next() {
                        state = State::ExpectChar(node);
                        continue;
                    } else {
                        return chars.next().is_none();
                    }
                }
                State::ExpectChar(node) => {
                    if let Some(char) = chars.next() {
                        match node {
                            Node::Char(expected) => {
                                if expected.is_match(char) {
                                    state = State::Empty;
                                    continue;
                                } else {
                                    return false;
                                }
                            }
                            Node::Asterisk(expected) => {
                                if expected.is_match(char) {
                                    state = State::ExpectChar(node);
                                    continue;
                                } else {
                                    state = State::ExpectNode(char);
                                    continue;
                                }
                            }
                        }
                    } else {
                        match node {
                            Node::Asterisk(_) => return nodes.next().is_none(),
                            Node::Char(_) => return false,
                        }
                    }
                }
                State::ExpectNode(char) => {
                    if let Some(node) = nodes.next() {
                        match node {
                            Node::Char(expected) => {
                                if expected.is_match(char) {
                                    state = State::Empty;
                                    continue;
                                } else {
                                    return false;
                                }
                            }
                            Node::Asterisk(expected) => {
                                if expected.is_match(char) {
                                    state = State::ExpectChar(node);
                                    continue;
                                } else {
                                    state = State::ExpectNode(char);
                                    continue;
                                }
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum State<'a> {
    Empty,
    ExpectChar(&'a Node),
    ExpectNode(char),
}

impl<S> From<S> for Pattern
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let s: &str = s.as_ref();
        let mut nodes: Vec<Node> = Vec::with_capacity(s.len());
        for char in s.chars() {
            if char == '*' {
                if let Some(Node::Char(ch)) = nodes.pop() {
                    nodes.push(Node::Asterisk(ch));
                } else {
                    panic!("Asterisk must follow a regular character")
                }
            } else if char == '.' {
                nodes.push(Node::Char(None));
            } else {
                nodes.push(Node::Char(Some(char)));
            }
        }
        Self { nodes }
    }
}
