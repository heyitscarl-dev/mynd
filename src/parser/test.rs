use super::{error::ParsingError, node::Node, parse};

#[test]
fn test_moddat_sequence() {
    let source = "+++---";
    let nodes_exp = vec![Node::ModDat(0)];
    let nodes_ret = parse(source).unwrap();

    assert_eq!(nodes_exp, nodes_ret);
}

#[test]
fn test_modptr_sequence() {
    let source = ">><<><><<";
    let nodes_exp = vec![Node::ModPtr(-1)];
    let nodes_ret = parse(source).unwrap();

    assert_eq!(nodes_exp, nodes_ret);
}

#[test]
fn test_simple_loop() {
    let source = "[->+<.,]";
    let nodes_exp = vec![Node::Loop(vec![
        Node::ModDat(-1),
        Node::ModPtr(1),
        Node::ModDat(1),
        Node::ModPtr(-1),
        Node::Output,
        Node::Accept
    ])];
    let nodes_ret = parse(source).unwrap();

    assert_eq!(nodes_exp, nodes_ret);
}

#[test]
fn test_nested_loop() {
    let source = "[+[-<]]";
    let nodes_exp = vec![Node::Loop(vec![
        Node::ModDat(1),
        Node::Loop(vec![
            Node::ModDat(-1),
            Node::ModPtr(-1)
        ])
    ])];
    let nodes_ret = parse(source).unwrap();

    assert_eq!(nodes_exp, nodes_ret);
}

#[test]
fn test_mixed() {
    let source = "+++[>--+<-]----";
    let nodes_exp = vec![
        Node::ModDat(3),
        Node::Loop(vec![
            Node::ModPtr(1),
            Node::ModDat(-1),
            Node::ModPtr(-1),
            Node::ModDat(-1),
        ]),
        Node::ModDat(-4)
    ];
    let nodes_ret = parse(source).unwrap();

    assert_eq!(nodes_exp, nodes_ret);
}

#[test]
fn test_unmatched_opening_bracket() {
    let source = "[+[-]";
    let result_exp = Err(ParsingError::UnclosedOpeningBracket(4));
    let result_ret = parse(source);

    assert_eq!(result_exp, result_ret);
}
