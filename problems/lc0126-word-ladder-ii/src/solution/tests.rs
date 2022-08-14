use super::*;

#[test]
fn check_hash_and_eq() {
    let key_1 = Bridge::new(b"abcdef", 3);
    let key_2 = Bridge::new(b"abcwef", 3);

    assert_eq!(key_1, key_2);

    let mut map = HashMap::new();
    map.insert(key_1, "one");
    assert_eq!(map.get(&key_2).copied(), Some("one"));
}

#[test]
fn vecdeque_extend_pushes_back() {
    let mut queue: VecDeque<_> = vec![1, 2, 3, 4].into();
    queue.extend([5, 6, 7, 8]);
    assert_eq!(
        queue.into_iter().collect::<Vec<_>>(),
        &[1, 2, 3, 4, 5, 6, 7, 8]
    );
}
