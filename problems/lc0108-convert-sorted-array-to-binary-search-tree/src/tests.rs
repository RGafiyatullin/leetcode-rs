#[test]
fn slices() {
    let slice = &[0, 1, 2, 3, 4];
    let left = &slice[0..2];
    let right = &slice[2 + 1..];

    assert_eq!(left, &[0, 1]);
    assert_eq!(right, &[3, 4]);

    let slice = &[0, 1];
    let left = &slice[0..0];
    let right = &slice[0 + 1..];

    assert_eq!(left, &[]);
    assert_eq!(right, &[1]);

    let slice = &[0];
    let left = &slice[0..0];
    let right = &slice[0 + 1..];

    assert_eq!(left, &[]);
    assert_eq!(right, &[]);
}
