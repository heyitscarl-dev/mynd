#[derive(Debug, PartialEq)]
pub enum Node {
    ModPtr(i8),
    ModDat(i8),

    Accept,
    Output,

    Loop(Vec<Node>)
}
