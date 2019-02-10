extern crate garden;

mod common;

#[test]
fn default_expression_finds_garden() {
    let config = common::garden_config();
    let result = garden::query::resolve_trees(&config, "cola");
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].garden, Some(0));
    assert_eq!(result[0].tree, 0);
    assert_eq!(result[1].garden, Some(0));
    assert_eq!(result[1].tree, 1);
}

#[test]
fn tree_expression_wildcard() {
    let config = common::garden_config();
    let result = garden::query::resolve_trees(&config, "@c*");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].garden, None);
    assert_eq!(result[0].tree, 1);
}