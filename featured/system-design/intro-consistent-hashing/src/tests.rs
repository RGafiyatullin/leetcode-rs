use crate::consistent_hashing::*;

#[test]
fn test_01() {
    // "ConsistentHashing",[6] => null
    // "getNodeForKey",[10] => 1,
    // "getNodeForKey",[20] => 1,
    // "getNodeForKey",[30] => 1,
    // "getNodeForKey",[40] => 1,
    // "getNodeForKey",[50] => 1,
    // "getNodeForKey",[10] => 1,
    // "removeNode",[2] => 1,
    // "getNodeForKey",[40] => 1,
    // "getNodeForKey",[30] => 1,
    // "addNode",[] => [7,6],
    // "getNodeForKey",[10] => 1,
    // "getNodeForKey",[20] => 1,
    // "getNodeForKey",[30] => 1,
    // "getKeysInNode"[1] => [10,20,30,40,50]

    let mut ch = ConsistentHashing::new(6);
    let node_for_10_1 = ch.get_node_for_key(10);
    eprintln!("node_for_10: {:?}", node_for_10_1);
    let node_for_20_1 = ch.get_node_for_key(20);
    eprintln!("node_for_20: {:?}", node_for_20_1);
    let node_for_30_1 = ch.get_node_for_key(30);
    eprintln!("node_for_30: {:?}", node_for_30_1);
    let node_for_40_1 = ch.get_node_for_key(40);
    eprintln!("node_for_40: {:?}", node_for_40_1);
    let node_for_50_1 = ch.get_node_for_key(50);
    eprintln!("node_for_50: {:?}", node_for_50_1);

    assert_eq!(node_for_10_1, ch.get_node_for_key(10));

    let node_2_heir = ch.remove_node(2);
    eprintln!("node_2_heir: {:?}", node_2_heir);
    let node_for_40_2 = ch.get_node_for_key(40);
    eprintln!("node_for_40_2: {:?}", node_for_40_2);
    let node_for_30_2 = ch.get_node_for_key(30);
    eprintln!("node_for_30_2: {:?}", node_for_30_2);
    let added = ch.add_node();
    assert_eq!(added.len(), 2);
    let node_id_6 = added[0];
    eprintln!("node_id_6: {:?}", node_id_6);
    let companion_of_6 = added[1];
    eprintln!("companion_of_6: {:?}", companion_of_6);

    let node_for_10_2 = ch.get_node_for_key(10);
    eprintln!("node_for_10_2: {:?}", node_for_10_2);
    let node_for_20_2 = ch.get_node_for_key(20);
    eprintln!("node_for_20_2: {:?}", node_for_20_2);
    let node_for_30_3 = ch.get_node_for_key(30);
    eprintln!("node_for_30_3: {:?}", node_for_30_3);
    let keys_in_node_1 = ch.get_keys_in_node(1);
    eprintln!("keys_in_node_1: {:?}", keys_in_node_1);
}
