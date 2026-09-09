//! Tests for ONX Dynamic Sharding per docs/specification/sharding.md.

use onx_data_structures::{ShardIdent, WorkchainIdent};
use onx_sharding::ShardTreeNode;

#[test]
fn test_shard_tree_leaf_split() {
    let root_shard = ShardIdent::root(WorkchainIdent::BASIC);
    let root_node = ShardTreeNode::new_leaf(root_shard);

    let split_node = root_node.split_leaf().unwrap();

    match split_node {
        ShardTreeNode::Internal { shard, left, right } => {
            assert_eq!(shard, root_shard);
            match (*left, *right) {
                (ShardTreeNode::Leaf { shard: s0, .. }, ShardTreeNode::Leaf { shard: s1, .. }) => {
                    assert_eq!(s0.prefix_len().unwrap(), 1);
                    assert_eq!(s1.prefix_len().unwrap(), 1);
                }
                _ => panic!("Expected child leaf nodes"),
            }
        }
        _ => panic!("Expected internal node after split"),
    }
}

#[test]
fn test_load_based_trigger_conditions() {
    let byte_limit = 1_000_000u64;
    let gas_limit = 10_000_000u64;

    // 75% load -> should split
    assert!(ShardTreeNode::should_split(
        750_000, byte_limit, 7_500_000, gas_limit
    ));

    // 70% load -> should not split
    assert!(!ShardTreeNode::should_split(
        700_000, byte_limit, 7_000_000, gas_limit
    ));

    // 20% load -> should merge
    assert!(ShardTreeNode::should_merge(
        200_000, byte_limit, 2_000_000, gas_limit
    ));

    // 25% load -> should not merge
    assert!(!ShardTreeNode::should_merge(
        250_000, byte_limit, 2_500_000, gas_limit
    ));
}
