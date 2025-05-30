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
    let mut level = 0;

    let nodes = parse_nodes(&tokens, &mut index, &mut level)?;

    if level != 0 {
        return Err(ParsingError::UnclosedOpeningBracket)
    }

    Ok(nodes)
}

pub fn parse_nodes(tokens: &Vec<char>, index: &mut usize, level: &mut usize) -> Result<Vec<Node>, ParsingError> {
    let mut nodes = Vec::new();

    while let Some(&c) = tokens.get(*index) {
        if c == ']' {
            if *level == 0 {
                return Err(ParsingError::UnopenedClosingBracket);
            }
            *level -= 1;
            return Ok(nodes);
        }

        let node = parse_node(tokens, index, level);
        nodes.push(node?);
    }

    Ok(nodes)
}

fn parse_node(tokens: &Vec<char>, index: &mut usize, level: &mut usize) -> Result<Node, ParsingError> {
    let token = tokens[*index];
    
    let node = match token {
        '>' | '<' => parse_ptr(tokens, index),
        '+' | '-' => parse_dat(tokens, index),
        '.' => Node::Output,
        ',' => Node::Accept,
        '[' => {
            *index += 1;
            *level += 1;

            Node::Loop(parse_nodes(tokens, index, level)?)
        },
        ']' => unreachable!("invalid close loop operator"),
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

