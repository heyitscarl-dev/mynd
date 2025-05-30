use error::ParsingError;
use node::Node;

use crate::lexer::tokenize;

pub mod node;
pub mod error;

#[cfg(test)]
mod test;

pub fn parse(source: impl Into<String>) -> Result<Vec<Node>, ParsingError> {
    let tokens = tokenize(source);
    let mut index = 0;

    let nodes = parse_nodes(&tokens, &mut index)?;

    // if there's a ']' left after parsing,
    // it's an unopened bracket. (i don't think this can
    // happen, but i'm not sure)

    if index < tokens.len() {
        if tokens[index] == ']' {
            return Err(ParsingError::UnopenedClosingBracket(index))
        }
    }

    Ok(nodes)
}

pub fn parse_nodes(tokens: &Vec<char>, index: &mut usize) -> Result<Vec<Node>, ParsingError> {
    let mut nodes = Vec::new();

    while let Some(&c) = tokens.get(*index) {
        if c == ']' {
            return Ok(nodes);
        }

        let node = parse_node(tokens, index);
        nodes.push(node?);
    }

    // If we hit EOF inside a loop, there's an unclosed '['

    if tokens.get(index.saturating_sub(1)) == Some(&'[') {
        return Err(ParsingError::UnclosedOpeningBracket(*index))
    }

    Ok(nodes)
}

fn parse_node(tokens: &Vec<char>, index: &mut usize) -> Result<Node, ParsingError> {
    let token = tokens[*index];
    
    let node = match token {
        '>' | '<' => parse_ptr(tokens, index),
        '+' | '-' => parse_dat(tokens, index),
        '.' => Node::Output,
        ',' => Node::Accept,
        '[' => {
            *index += 1;
            Node::Loop(parse_nodes(tokens, index)?)
        },

        // If there's a ']' outside a loop, it wasn't opened
        ']' => return Err(ParsingError::UnopenedClosingBracket(*index)),
        _ => unreachable!("invalid token: {}", token),
    };

    *index += 1;

    return Ok(node);
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

