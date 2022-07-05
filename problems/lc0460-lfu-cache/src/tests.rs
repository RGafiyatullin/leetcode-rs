use super::Command;
use super::LFUCache;

const CASES: &[&[(Command, Option<i32>)]] = &[
    /*
    Input
    ["LFUCache", "put", "put", "get", "put", "get", "get", "put", "get", "get", "get"]
    [[2], [1, 1], [2, 2], [1], [3, 3], [2], [3], [4, 4], [1], [3], [4]]
    Output
    [null, null, null, 1, null, -1, 3, null, -1, 3, 4]

    */
    &[
        (Command::Init(2), None),
        (Command::Put(1, 1), None),
        (Command::Put(2, 2), None),
        (Command::Get(1), Some(1)),
        (Command::Put(3, 3), None),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(3)),
        (Command::Put(4, 4), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(3), Some(3)),
        (Command::Get(4), Some(4)),
    ],
    &[
        (Command::Init(3), None),
        (Command::Put(2, 2), None),
        (Command::Put(1, 1), None),
        (Command::Get(2), Some(2)),
        (Command::Get(1), Some(1)),
        (Command::Get(2), Some(2)),
        (Command::Put(3, 3), None),
        (Command::Put(4, 4), None),
        (Command::Get(3), Some(-1)),
        (Command::Get(2), Some(2)),
        (Command::Get(1), Some(1)),
        (Command::Get(4), Some(4)),
    ],
];

#[test]
fn test_all_cases() {
    for commands in CASES {
        let mut cache = LFUCache::new(1);
        for (command, expected) in *commands {
            let command = *command;
            let expected = *expected;

            eprintln!("> {:?}", command);
            let actual = command.apply(&mut cache);
            eprintln!("| {:#?}", cache);

            assert_eq!(actual, expected, "case: {:?}", command);
        }
    }
}

#[test]
fn test_one_case() {
    let mut cache = LFUCache::new(1);
    for (command, expected) in CASES[1] {
        let command = *command;
        let expected = *expected;

        eprintln!("> {:#?}", command);
        let actual = command.apply(&mut cache);
        eprintln!("| {:#?}", cache);

        assert_eq!(actual, expected, "case: {:?}", command);
    }
}
