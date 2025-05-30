use node::Node;

use crate::lexer::tokenize;

pub mod node;

#[cfg(test)]
mod test;

pub fn parse(source: impl Into<String>) -> Vec<Node> {
    let tokens = tokenize(source);
    let mut index = 0;

    parse_nodes(&tokens, &mut index)
}

pub fn parse_nodes(tokens: &Vec<char>, index: &mut usize) -> Vec<Node> {
    let mut nodes = Vec::new();

    while let Some(&c) = tokens.get(*index) {
        if c == ']' {
            return nodes;
        }

        let node = parse_node(tokens, index);
        nodes.push(node);
    }

    nodes
}

fn parse_node(tokens: &Vec<char>, index: &mut usize) -> Node {
    let token = tokens[*index];
    
    let node = match token {
        '>' | '<' => parse_ptr(tokens, index),
        '+' | '-' => parse_dat(tokens, index),
        '.' => Node::Output,
        ',' => Node::Accept,
        '[' => {
            *index += 1;
            Node::Loop(parse_nodes(tokens, index))
        },
        ']' => todo!(),
        _ => unreachable!("invalid token: {}", token),
    };

    *index += 1;

    return node;
}

fn parse_ptr(tokens: &Vec<char>, index: &mut usize) -> Node {
    let mut val = 0;

    while let Some(&c) = tokens.get(*index) {
        match c {
            '>' => val += 1,
            '<' => val -= 1,
            _ => break,
        }
        *index += 1;
    }
    
    *index -= 1;
    return Node::ModPtr(val);
}

fn parse_dat(tokens: &Vec<char>, index: &mut usize) -> Node {
    let mut val = 0;

    while let Some(&c) = tokens.get(*index) {
        match c {
            '+' => val += 1,
            '-' => val -= 1,
            _ => break,
        }
        *index += 1;
    }
    
    *index -= 1;
    return Node::ModDat(val);
}

