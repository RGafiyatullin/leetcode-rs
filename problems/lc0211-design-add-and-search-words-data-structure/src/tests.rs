use crate::solution::WordDictionary;

const CASES: &[(&[&str], &[&[&str]], &[Option<bool>])] = &[(
    &["WordDictionary", "addWord", "addWord", "addWord", "search", "search", "search", "search"],
    &[&[], &["bad"], &["dad"], &["mad"], &["pad"], &["bad"], &[".ad"], &["b.."]],
    &[None, None, None, None, Some(false), Some(true), Some(true), Some(true)],
)];

#[test]
fn test_01() {
    let mut dict = WordDictionary::new();

    for &(fs, args, outs) in CASES {
        let this_case = fs.iter().zip(args).zip(outs);
        for ((f, a), out) in this_case {
            eprintln!("DICT: {:#?}", dict);
            eprintln!("  f: {:?}; a: {:?}", f, a);

            match (*f, *a) {
                ("WordDictionary", &[]) => dict = WordDictionary::new(),
                ("addWord", &[word]) => dict.add_word(word.to_owned()),
                ("search", &[word]) => assert_eq!(dict.search(word.to_owned()), out.unwrap()),

                unexpected => panic!("unexpected command: {:#?}", unexpected),
            }
        }
    }
}
