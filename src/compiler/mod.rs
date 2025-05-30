#![allow(unused)]

use crate::parser::{error::ParsingError, node::Node, parse};

pub const PREFIX: &str = r#"
.intel_syntax noprefix
.section .bss
    .lcomm dat, 30000

.section .text
.global _start

_start:
    lea rsi, [dat]
"#;

pub const SUFFIX: &str = r#"
    // sys_exit
    mov rax, 60
    mov rdi, 69
    syscall
"#;

pub fn compile(source: impl Into<String>) -> Result<String, ParsingError> {
    let nodes = parse(source)?;
    let mut output = String::new();
    let mut loops = 0;

    output.push_str(PREFIX);
    output.push_str(&compile_nodes(nodes, &mut loops));
    output.push_str(SUFFIX);
    Ok(output)
}

fn compile_nodes(nodes: Vec<Node>, loops: &mut usize) -> String {
    let mut output = String::new();
    
    for node in nodes {
        output.push_str(&compile_node(node, loops));
        output.push_str("\n");
    }

    output
}

fn compile_node(node: Node, loops: &mut usize) -> String {
    match node {
        Node::ModPtr(val) => if val > 0 {
            format!("add rsi, {}", val)
        } else {
            format!("sub rsi, {}", -val)
        },
        Node::ModDat(val) => if val > 0 {
            format!("add byte ptr [rsi], {}", val)
        } else {
            format!("sub byte ptr [rsi], {}", -val)
        },
        Node::Output => {
            format!("mov rax, 1\nmov rdi, 1\nmov rsi, rsi\nmov rdx, 1\nsyscall")
        },
        Node::Accept => {
            format!("mov rax, 0\nmov rdi, 0\nmov rsi, rsi\nmov rdx, 1\nsyscall")
        },
        Node::Loop(children) => {
            *loops += 1;
            let body = compile_nodes(children, loops);
            format!("l{}s:\ncmp byte ptr [rsi], 0\nje l{}e\n{}jmp l{}s\nl{}e:", *loops, *loops, body, *loops, *loops)
        }
    }.to_string()
}
