use crate::solution::Codec;

pub use utils::tree::{self, TreeNode};

const CASES: &[&[Option<i32>]] = &[
    &[Some(1), Some(2), Some(3), None, None, Some(4), Some(5)],
    &[],
];

#[test]
fn run_all_cases() {
    for &bft_in in CASES.iter() {
        let tree_in = tree::tree_from_bft(bft_in);
        let codec = Codec::new();
        let serialized = codec.serialize(tree_in);
        eprintln!("SERIALIZED: {:?}", serialized);
        let tree_out = codec.deserialize(serialized);
        let bft_out = tree::tree_to_bft(tree_out);
        assert_eq!(bft_in, bft_out);
    }
}
