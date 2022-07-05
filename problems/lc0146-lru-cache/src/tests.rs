use super::solution::LRUCache;
use super::Command;

const CASES: &[&[(Command, Option<i32>)]] = &[
    &[
        (Command::Init(1), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(-1)),
        (Command::Put(1, 1), None),
        (Command::Get(1), Some(1)),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(-1)),
        (Command::Put(2, 2), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(2)),
        (Command::Get(3), Some(-1)),
        (Command::Put(3, 3), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(3)),
        (Command::Put(3, 3), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(3)),
    ],
    &[
        (Command::Init(2), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(-1)),
        (Command::Get(4), Some(-1)),
        (Command::Put(1, 1), None),
        (Command::Get(1), Some(1)),
        (Command::Get(2), Some(-1)),
        (Command::Get(3), Some(-1)),
        (Command::Get(4), Some(-1)),
        (Command::Put(2, 2), None),
        (Command::Get(1), Some(1)),
        (Command::Get(2), Some(2)),
        (Command::Get(3), Some(-1)),
        (Command::Get(4), Some(-1)),
        (Command::Put(3, 3), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(2)),
        (Command::Get(3), Some(3)),
        (Command::Get(4), Some(-1)),
        (Command::Get(2), Some(2)),
        (Command::Put(4, 4), None),
        (Command::Get(1), Some(-1)),
        (Command::Get(2), Some(2)),
        (Command::Get(3), Some(-1)),
        (Command::Get(4), Some(4)),
    ],
];

#[test]
fn test_all_cases() {
    for commands in CASES {
        let mut cache = LRUCache::new(1);
        for (command, expected) in *commands {
            let command = *command;
            let expected = *expected;

            let actual = command.apply(&mut cache);
            assert_eq!(actual, expected, "case: {:?}", command);
        }
    }
}
