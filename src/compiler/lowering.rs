use crate::ast::{BlockLiteral, Expr, NodeExpr, Program};
use crate::ir::{BlockIr, ExprIr, NodeIr, ProgramIr};

pub fn lower_program(program: &Program) -> ProgramIr {
    ProgramIr {
        expressions: program.expressions.iter().map(lower_expr).collect(),
    }
}

fn lower_expr(expr: &Expr) -> ExprIr {
    ExprIr {
        nodes: expr.nodes.iter().map(lower_node).collect(),
    }
}

fn lower_node(node: &NodeExpr) -> NodeIr {
    match node {
        NodeExpr::Identifier(name) => NodeIr::Identifier(name.clone()),
        NodeExpr::StringLiteral(text) => NodeIr::StringLiteral(text.clone()),
        NodeExpr::BlockLiteral(block) => NodeIr::BlockLiteral(lower_block(block)),
        NodeExpr::SubExpression(expr) => NodeIr::SubExpression(Box::new(lower_expr(expr))),
        NodeExpr::Reserved(token) => NodeIr::Reserved(*token),
    }
}

fn lower_block(block: &BlockLiteral) -> BlockIr {
    BlockIr {
        id: block.id,
        expressions: block.expressions.iter().map(lower_expr).collect(),
    }
}
