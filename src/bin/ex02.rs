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
        let pattern: Pattern = p.parse().expect("failed to parse the pattern");
        pattern.is_match(&s)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CharMatch {
    Any,
    Exact(char),
}
impl CharMatch {
    pub fn is_match(&self, input: char) -> bool {
        match *self {
            CharMatch::Any => true,
            CharMatch::Exact(expected) => expected == input,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Quantifier {
    One,
    Any,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatternNode(pub CharMatch, pub Quantifier);

impl PatternNode {
    pub fn char(&self) -> &CharMatch {
        &self.0
    }
    pub fn quantifier(&self) -> &Quantifier {
        &self.1
    }
}

#[derive(Debug)]
struct Pattern(pub Vec<PatternNode>);

impl std::str::FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut acc = Vec::<PatternNode>::with_capacity(s.len());

        for ch in s.chars() {
            match ch {
                '.' => acc.push(PatternNode(CharMatch::Any, Quantifier::One)),
                '*' => match acc.pop() {
                    None => {
                        return Err("asterisk cannot start an expression".to_owned())
                    }
                    Some(PatternNode(_ch, Quantifier::Any)) => {
                        return Err("cannot have two asterisks in a row".to_owned())
                    }
                    Some(PatternNode(ch, Quantifier::One)) => {
                        acc.push(PatternNode(ch, Quantifier::Any))
                    }
                },
                ch => acc.push(PatternNode(CharMatch::Exact(ch), Quantifier::One)),
            }
        }

        Ok(Pattern(acc))
    }
}

impl Pattern {
    pub fn is_match(&self, input: &str) -> bool {
        let chars = input.chars().collect::<Vec<_>>();
        let chars = Cursor::from(&chars[..]);
        let nodes = Cursor::from(&self.0[..]);
        let mut forks = Vec::<Fork>::with_capacity(self.0.len());
        let mut state = State::Match(nodes, chars);

        loop {
            // println!("state: {:?}; forks: {:?}", state, forks);
            state = match state {
                State::Unwind => {
                    if let Some(Fork {
                        nodes,
                        chars,
                        char_match,
                    }) = forks.pop()
                    {
                        if let (Some(char), next_chars) = chars.next() {
                            if char_match.is_match(*char) {
                                let fork = Fork {
                                    nodes: nodes.clone(),
                                    chars: next_chars.clone(),
                                    char_match,
                                };
                                forks.push(fork);
                                State::Match(nodes, next_chars)
                            } else {
                                State::Unwind
                            }
                        } else {
                            State::Unwind
                        }
                    } else {
                        return false;
                    }
                }

                State::Match(nodes, chars) => match nodes.next() {
                    (None, _) => {
                        if chars.has_next() {
                            State::Unwind
                        } else {
                            return true;
                        }
                    }

                    (Some(node), next_nodes) => match node.quantifier() {
                        Quantifier::One => match chars.next() {
                            (None, _) => State::Unwind,
                            (Some(char), next_chars) => {
                                if node.char().is_match(*char) {
                                    State::Match(next_nodes, next_chars)
                                } else {
                                    State::Unwind
                                }
                            }
                        },

                        Quantifier::Any => {
                            let fork = Fork {
                                nodes: next_nodes.clone(),
                                chars: chars.clone(),
                                char_match: node.char().clone(),
                            };
                            forks.push(fork);
                            State::Match(next_nodes, chars)
                        }
                    },
                },
            }
        }
    }
}

#[derive(Debug)]
enum State<'n, 'c> {
    Match(Cursor<'n, PatternNode>, Cursor<'c, char>),
    Unwind,
}

#[derive(Debug)]
struct Fork<'n, 'c> {
    nodes: Cursor<'n, PatternNode>,
    chars: Cursor<'c, char>,
    char_match: CharMatch,
}

#[derive(Debug, Clone)]
enum Cursor<'s, I> {
    Before(&'s [I]),
    Inside(&'s [I], usize),
    After(&'s [I]),
}
impl<'s, I> From<&'s [I]> for Cursor<'s, I> {
    fn from(slice: &'s [I]) -> Self {
        Cursor::Before(slice)
    }
}
impl<'s, I> Cursor<'s, I> {
    pub fn next(&self) -> (Option<&'s I>, Self) {
        match *self {
            Cursor::Before(slice) if slice.is_empty() => (None, Cursor::After(slice)),
            Cursor::Before(slice) => (Some(&slice[0]), Cursor::Inside(slice, 0)),
            Cursor::Inside(slice, pos) if pos + 1 >= slice.len() => {
                (None, Cursor::After(slice))
            }
            Cursor::Inside(slice, pos) => {
                (Some(&slice[pos + 1]), Cursor::Inside(slice, pos + 1))
            }
            Cursor::After(slice) => (None, Cursor::After(slice)),
        }
    }
    pub fn has_next(&self) -> bool {
        match *self {
            Cursor::After(_) => false,
            Cursor::Inside(slice, pos) => pos + 1 < slice.len(),
            Cursor::Before(slice) => !slice.is_empty(),
        }
    }
}

#[test]
fn tests() {
    let tests = vec![
        ("", "", true),
        ("ab", "", false),
        ("ab", "ab", true),
        (".", "a", true),
        (".", "", false),
        (".", "ab", false),
        ("..*.", "a", false),
        ("..*.", "ab", true),
        ("..*.", "abc", true),
        ("..*.", "abcd", true),
        ("ab.*cd", "abcd", true),
        ("ab.*cd", "ab..cd", true),
        ("ab.*c", "abcd", false),
        (".*a.*b.*c.*d.*", "abcd", true),
        (".*a.*b.*c.*d.*", "_a_b_c_d_", true),
    ];
    for (pattern, input, expected) in tests {
        let pattern: Pattern = pattern.parse().unwrap();
        assert_eq!(pattern.is_match(input), expected);
    }
}
