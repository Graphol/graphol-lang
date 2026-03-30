use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::ast::{
    ArithmeticOp, BlockLiteral, BooleanOp, Expr, LogicOp, NodeExpr, Program, ReservedToken,
};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub position: usize,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at position {}", self.message, self.position)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_program(source: &str) -> Result<Program, ParseError> {
    let mut parser = Parser::new(source);
    parser.parse_program()
}

struct Parser<'a> {
    chars: Vec<char>,
    pos: usize,
    block_counter: usize,
    _source: &'a str,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            block_counter: 0,
            _source: source,
        }
    }

    fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut expressions = Vec::new();
        while self.pos < self.chars.len() {
            self.consume_noise();
            if self.pos >= self.chars.len() {
                break;
            }

            if self.peek_char() == Some('}') {
                return Err(self.error("Unexpected '}'"));
            }

            let expr = self.parse_expression()?;
            if !expr.nodes.is_empty() {
                expressions.push(expr);
            }

            if self.pos < self.chars.len() {
                self.pos += 1;
            }
        }

        Ok(Program {
            expressions: Rc::new(expressions),
        })
    }

    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        let mut nodes = Vec::new();

        self.consume_spaces();
        while self.pos < self.chars.len() {
            let current = self.chars[self.pos];
            if matches!(current, '\n' | '\r' | ')' | '}') {
                break;
            }

            let node = match current {
                '{' => {
                    self.pos += 1;
                    NodeExpr::BlockLiteral(self.parse_block()?)
                }
                '(' => {
                    self.pos += 1;
                    let subexpr = self.parse_expression()?;
                    if self.peek_char() != Some(')') {
                        return Err(self.error("Expected ')'"));
                    }
                    NodeExpr::SubExpression(Box::new(subexpr))
                }
                '"' => NodeExpr::StringLiteral(self.parse_string()?),
                _ => self.parse_reserved_or_identifier()?,
            };

            nodes.push(node);
            self.pos += 1;
            self.consume_spaces();
        }

        Ok(Expr { nodes })
    }

    fn parse_block(&mut self) -> Result<BlockLiteral, ParseError> {
        let mut expressions = Vec::new();
        let block_id = self.next_block_id();

        loop {
            self.consume_noise();
            if self.pos >= self.chars.len() {
                return Err(self.error("Unclosed block"));
            }
            if self.chars[self.pos] == '}' {
                break;
            }

            let expr = self.parse_expression()?;
            if !expr.nodes.is_empty() {
                expressions.push(expr);
            }

            if self.peek_char() == Some('}') {
                break;
            }

            self.pos += 1;
            if self.pos >= self.chars.len() {
                return Err(self.error("Unclosed block"));
            }
        }

        Ok(BlockLiteral {
            id: block_id,
            expressions: Rc::new(expressions),
        })
    }

    fn parse_string(&mut self) -> Result<String, ParseError> {
        let mut out = String::new();
        self.pos += 1;

        while self.pos < self.chars.len() {
            let current = self.chars[self.pos];
            if current == '"' {
                return Ok(out);
            }

            if current == '\\' {
                self.pos += 1;
                if self.pos >= self.chars.len() {
                    return Err(self.error("Invalid string escape"));
                }
                out.push(self.chars[self.pos]);
            } else {
                out.push(current);
            }
            self.pos += 1;
        }

        Err(self.error("Unclosed string literal"))
    }

    fn parse_reserved_or_identifier(&mut self) -> Result<NodeExpr, ParseError> {
        if let Some(two_char) = self.peek_two_chars() {
            let reserved = match two_char.as_str() {
                "!=" => Some(ReservedToken::Logic(LogicOp::Ne)),
                "<=" => Some(ReservedToken::Logic(LogicOp::Le)),
                ">=" => Some(ReservedToken::Logic(LogicOp::Ge)),
                "x|" => Some(ReservedToken::Boolean(BooleanOp::Xor)),
                _ => None,
            };
            if let Some(token) = reserved {
                self.pos += 1;
                return Ok(NodeExpr::Reserved(token));
            }
        }

        if let Some(current) = self.peek_char() {
            let reserved = match current {
                '+' => Some(ReservedToken::Arithmetic(ArithmeticOp::Add)),
                '-' => Some(ReservedToken::Arithmetic(ArithmeticOp::Sub)),
                '*' => Some(ReservedToken::Arithmetic(ArithmeticOp::Mul)),
                '/' => Some(ReservedToken::Arithmetic(ArithmeticOp::Div)),
                '^' => Some(ReservedToken::Arithmetic(ArithmeticOp::Xor)),
                '&' => Some(ReservedToken::Boolean(BooleanOp::And)),
                '|' => Some(ReservedToken::Boolean(BooleanOp::Or)),
                '!' => Some(ReservedToken::Boolean(BooleanOp::Not)),
                '>' => Some(ReservedToken::Logic(LogicOp::Gt)),
                '<' => Some(ReservedToken::Logic(LogicOp::Lt)),
                '=' => Some(ReservedToken::Logic(LogicOp::Eq)),
                _ => None,
            };

            if let Some(token) = reserved {
                return Ok(NodeExpr::Reserved(token));
            }
        }

        let start = self.pos;
        while self.pos < self.chars.len() && !self.is_name_terminator(self.chars[self.pos]) {
            self.pos += 1;
        }

        if start == self.pos {
            return Err(self.error("Unexpected token"));
        }

        let name: String = self.chars[start..self.pos].iter().collect();
        self.pos -= 1;
        Ok(NodeExpr::Identifier(name))
    }

    fn is_name_terminator(&self, c: char) -> bool {
        matches!(
            c,
            '\n' | '\r' | ' ' | '+' | '-' | '*' | '/' | '^' | ')' | '(' | '{' | '}'
        )
    }

    fn consume_noise(&mut self) {
        while self.pos < self.chars.len() && matches!(self.chars[self.pos], '\n' | '\r' | ' ') {
            self.pos += 1;
        }
    }

    fn consume_spaces(&mut self) {
        while self.pos < self.chars.len() && self.chars[self.pos] == ' ' {
            self.pos += 1;
        }
    }

    fn next_block_id(&mut self) -> usize {
        self.block_counter += 1;
        self.block_counter
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn peek_two_chars(&self) -> Option<String> {
        if self.pos + 1 >= self.chars.len() {
            return None;
        }
        Some(format!(
            "{}{}",
            self.chars[self.pos],
            self.chars[self.pos + 1]
        ))
    }

    fn error(&self, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            position: self.pos,
        }
    }
}
