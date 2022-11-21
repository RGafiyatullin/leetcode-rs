pub struct Solution;

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let tokens = tokenize(s.as_str());

        let tokens = tokens.collect::<Vec<_>>();
        // eprintln!("TOKENS: {:?}", tokens);

        let mut output = shunting_yard(tokens);
        // eprintln!("OUTPUT: {:?}", output);

        let value = rpn_eval(&mut output).expect("Expr error");

        value
    }
}

const UNARY_MINUS_TOKEN: &str = "_";
const INFIX_MINUS_TOKEN: &str = "-";
const INFIX_PLUS_TOKEN: &str = "+";
const LEFT_PAREN: &str = "(";
const RIGHT_PAREN: &str = ")";

fn precedence(op: &str) -> usize {
    match op {
        LEFT_PAREN => 0,
        UNARY_MINUS_TOKEN | INFIX_PLUS_TOKEN | INFIX_MINUS_TOKEN => 2,
        _ => 3,
    }
}
fn arity(op: &str) -> usize {
    match op {
        LEFT_PAREN | RIGHT_PAREN => panic!("Paren in output"),
        UNARY_MINUS_TOKEN => 1,
        INFIX_PLUS_TOKEN | INFIX_MINUS_TOKEN => 2,
        _ => 0,
    }
}

fn rpn_eval(rpn: &mut Vec<&str>) -> Result<i32, String> {
    let op = rpn.pop().ok_or("unexpected end of expression").map_err(ToOwned::to_owned)?;
    let arity = arity(op);
    if arity == 0 {
        op.parse::<i32>().map_err(|e| e.to_string())
    } else {
        let mut args = Vec::with_capacity(arity);
        for _ in 0..arity {
            args.push(rpn_eval(rpn)?);
        }
        match op {
            UNARY_MINUS_TOKEN => Ok(-args[0]),
            INFIX_MINUS_TOKEN => Ok(args[1] - args[0]),
            INFIX_PLUS_TOKEN => Ok(args[1] + args[0]),
            invalid => Err(format!("Invalid operator: {:?}", invalid)),
        }
    }
}

fn op_push<'a>(op: &'a str, opstack: &mut Vec<&'a str>, output: &mut Vec<&'a str>) {
    while opstack.last().copied().map(precedence).into_iter().any(|p| p >= precedence(op)) {
        if let Some(op) = opstack.pop() {
            output.push(op);
        }
    }

    opstack.push(op);
}

fn shunting_yard<'a>(tokens: impl IntoIterator<Item = &'a str>) -> Vec<&'a str> {
    let mut output = vec![];
    let mut opstack = vec![];

    let mut minus_is_unary = true;

    for token in tokens {
        // eprintln!("output:  {:?}", output);
        // eprintln!("opstack: {:?}", opstack);
        // eprintln!("<- {:?}", token);

        minus_is_unary = match token {
            LEFT_PAREN => {
                opstack.push(LEFT_PAREN);
                true
            },
            INFIX_PLUS_TOKEN => {
                op_push(token, &mut opstack, &mut output);
                false
            },
            INFIX_MINUS_TOKEN if minus_is_unary => {
                op_push(UNARY_MINUS_TOKEN, &mut opstack, &mut output);
                false
            },
            INFIX_MINUS_TOKEN => {
                op_push(token, &mut opstack, &mut output);
                false
            },
            RIGHT_PAREN => {
                while let Some(op) = opstack.pop() {
                    if op == LEFT_PAREN {
                        break
                    } else {
                        output.push(op)
                    }
                }
                false
            },
            _ => {
                // op_push(token, &mut opstack, &mut output);
                output.push(token);
                false
            },
        }
    }

    while let Some(op) = opstack.pop() {
        // if op != LEFT_PAREN {
        output.push(op);
        // }
    }

    output
}

fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = &'a str> + 'a {
    enum Yield<R> {
        None,
        One(R),
        Two(R, R),
    }
    impl<R> Iterator for Yield<R> {
        type Item = R;
        fn next(&mut self) -> Option<Self::Item> {
            let (out, this) = match std::mem::replace(self, Self::None) {
                Self::None => (None, Self::None),
                Self::One(r) => (Some(r), Self::None),
                Self::Two(r1, r2) => (Some(r1), Self::One(r2)),
            };
            *self = this;
            out
        }
    }

    input
        .as_bytes()
        .iter()
        .copied()
        .chain([' ' as u8])
        .enumerate()
        .scan(Option::<usize>::None, |state, (idx, ch)| {
            let (next_state, out) = match (state.take(), ch as char) {
                (None, ' ') => (None, Yield::None),
                (Some(start_pos), ' ') => (None, Yield::One(start_pos..idx)),
                (Some(start_pos), d) if d.is_ascii_digit() => (Some(start_pos), Yield::None),
                (Some(start_pos), _c) => (None, Yield::Two(start_pos..idx, idx..idx + 1)),
                (None, d) if d.is_ascii_digit() => (Some(idx), Yield::None),
                (None, _c) => (None, Yield::One(idx..idx + 1)),
            };
            *state = next_state;

            Some(out)
        })
        .flatten()
        .map(move |range| &input[range])
}

#[test]
fn test_tokenize() {
    const CASES: &[(&str, &[&str])] = &[
        ("", &[]),
        ("1", &["1"]),
        ("11", &["11"]),
        ("()", &["(", ")"]),
        ("(1)", &["(", "1", ")"]),
        ("-1", &["-", "1"]),
        ("1 1 1", &["1", "1", "1"]),
    ];

    for &(input, expected) in CASES {
        let output = tokenize(input).collect::<Vec<_>>();
        assert_eq!(output, expected);
    }
}
