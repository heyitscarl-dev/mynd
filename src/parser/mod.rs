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

    while *index < tokens.len() {
        if tokens[*index] == ']' {
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
        '.' => {
            *index += 1;
            Node::Output
        },
        ',' => {
            *index += 1;
            Node::Accept
        },
        '[' => {
            *index += 1;    // skip '['
            *level += 1;

            let body = parse_nodes(tokens, index, level)?;

            *index += 1;    // skip ']'
            Node::Loop(body)
        },
        ']' => unreachable!("invalid close loop operator"),
        _ => unreachable!("invalid token: {}", token),
    };

    Ok(node)
}

fn parse_ptr(tokens: &Vec<char>, index: &mut usize) -> Node {
    let mut val = 0;

    while *index < tokens.len() {
        match tokens[*index] {
            '>' => val += 1,
            '<' => val -= 1,
            _ => break,
        }
        *index += 1;
    }
    
    Node::ModPtr(val)
}

fn parse_dat(tokens: &Vec<char>, index: &mut usize) -> Node {
    let mut val = 0;

    while *index < tokens.len() {
        match tokens[*index] {
            '+' => val += 1,
            '-' => val -= 1,
            _ => break,
        }
        *index += 1;
    }
    
    Node::ModDat(val)
}

