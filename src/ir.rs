use crate::ast::ReservedToken;

#[derive(Debug, Clone)]
pub struct ProgramIr {
    pub expressions: Vec<ExprIr>,
}

#[derive(Debug, Clone)]
pub struct ExprIr {
    pub nodes: Vec<NodeIr>,
}

#[derive(Debug, Clone)]
pub enum NodeIr {
    Identifier(String),
    StringLiteral(String),
    BlockLiteral(BlockIr),
    SubExpression(Box<ExprIr>),
    Reserved(ReservedToken),
}

#[derive(Debug, Clone)]
pub struct BlockIr {
    pub id: usize,
    pub expressions: Vec<ExprIr>,
}
