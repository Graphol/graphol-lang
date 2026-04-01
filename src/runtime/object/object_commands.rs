use std::any::Any;
use std::rc::Rc;

use crate::ast::Expr;

use super::super::host::ExecutionHost;
use super::super::io::OutputMode;
use super::super::scope::ScopeRef;
use super::super::value::{ObjectRef, Value};
use super::new_node;
use super::{
    BlockSnapshot, GrapholObject, MessageKind, StdoutState, exec_object, message_kind, object_ref,
    receive_object,
};

pub fn new_block(id: usize, expressions: Rc<Vec<Expr>>, parent_scope: ScopeRef) -> ObjectRef {
    object_ref(BlockObject {
        id,
        expressions,
        parent_scope,
        is_sync: true,
        inbox: new_node(),
    })
}

pub fn new_input() -> ObjectRef {
    object_ref(InputCommand {
        value: String::new(),
    })
}

pub fn new_echo(stdout: StdoutState) -> ObjectRef {
    object_ref(EchoCommand { stdout })
}

pub fn new_stdout(stdout: StdoutState) -> ObjectRef {
    object_ref(StdoutCommand { stdout })
}

pub fn new_if() -> ObjectRef {
    object_ref(IfCommand {
        cond: None,
        execute_else: true,
        state: 0,
    })
}

pub fn new_while() -> ObjectRef {
    object_ref(WhileCommand {
        cond: None,
        state: 0,
    })
}

pub fn new_message_run() -> ObjectRef {
    object_ref(MessageObject {
        kind: MessageKind::Run,
    })
}

pub fn new_message_async() -> ObjectRef {
    object_ref(MessageObject {
        kind: MessageKind::Async,
    })
}

pub fn new_message_else() -> ObjectRef {
    object_ref(MessageObject {
        kind: MessageKind::Else,
    })
}

struct MessageObject {
    kind: MessageKind,
}

impl GrapholObject for MessageObject {
    fn receive(&mut self, _value: Value, _host: &mut dyn ExecutionHost) {}

    fn get_type(&self) -> &'static str {
        "message"
    }

    fn get_message(&self) -> Option<MessageKind> {
        Some(self.kind)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BlockObject {
    id: usize,
    expressions: Rc<Vec<Expr>>,
    parent_scope: ScopeRef,
    is_sync: bool,
    inbox: ObjectRef,
}

impl GrapholObject for BlockObject {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        match message_kind(&value) {
            Some(MessageKind::Run) => self.exec(host),
            Some(MessageKind::Async) => self.is_sync = false,
            _ => {
                self.inbox = new_node();
                receive_object(&self.inbox, value, host);
            }
        }
    }

    fn exec(&mut self, host: &mut dyn ExecutionHost) {
        host.call_block(BlockSnapshot {
            id: self.id,
            expressions: self.expressions.clone(),
            parent_scope: self.parent_scope.clone(),
            inbox: self.inbox.clone(),
            is_async: !self.is_sync,
        });
    }

    fn get_type(&self) -> &'static str {
        "block"
    }

    fn block_snapshot(&self) -> Option<BlockSnapshot> {
        Some(BlockSnapshot {
            id: self.id,
            expressions: self.expressions.clone(),
            parent_scope: self.parent_scope.clone(),
            inbox: self.inbox.clone(),
            is_async: !self.is_sync,
        })
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct InputCommand {
    value: String,
}

impl GrapholObject for InputCommand {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        self.value = host.read_input(&value.as_text());
    }

    fn tonumber(&self) -> f64 {
        self.value.parse::<f64>().unwrap_or(0.0)
    }

    fn tostring(&self) -> String {
        self.value.clone()
    }

    fn get_type(&self) -> &'static str {
        "input"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct EchoCommand {
    stdout: StdoutState,
}

impl GrapholObject for EchoCommand {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        host.emit_output(self.stdout.mode(), &value.as_text());
    }

    fn get_type(&self) -> &'static str {
        "command"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct StdoutCommand {
    stdout: StdoutState,
}

impl GrapholObject for StdoutCommand {
    fn receive(&mut self, value: Value, _host: &mut dyn ExecutionHost) {
        let mode = value.as_text().to_ascii_lowercase();
        match mode.as_str() {
            "console" => self.stdout.set_mode(OutputMode::Console),
            _ => self.stdout.set_mode(OutputMode::Alert),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct IfCommand {
    cond: Option<Value>,
    execute_else: bool,
    state: u8,
}

impl GrapholObject for IfCommand {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        if message_kind(&value) == Some(MessageKind::Else) {
            self.state = 3;
            return;
        }

        if self.state == 0 {
            self.cond = Some(value);
            self.state = 1;
            return;
        }

        if self.state == 1 {
            self.state = 0;
            if self.cond.as_ref().map(Value::as_bool).unwrap_or(false) {
                if let Value::Obj(obj) = value {
                    exec_object(&obj, host);
                }
                self.execute_else = false;
            }
            return;
        }

        if self.state == 3 {
            self.state = 0;
            if self.execute_else {
                if let Value::Obj(obj) = value {
                    exec_object(&obj, host);
                }
            }
        }
    }

    fn end(&mut self) {
        self.cond = None;
        self.execute_else = true;
        self.state = 0;
    }

    fn get_type(&self) -> &'static str {
        "command"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct WhileCommand {
    cond: Option<Value>,
    state: u8,
}

impl GrapholObject for WhileCommand {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        if self.state == 0 {
            self.cond = Some(value);
            self.state = 1;
            return;
        }

        self.state = 0;
        if self.cond.as_ref().map(Value::as_bool).unwrap_or(false) {
            if let Value::Obj(obj) = value {
                exec_object(&obj, host);
            }
        }
    }

    fn end(&mut self) {
        self.cond = None;
        self.state = 0;
    }

    fn get_type(&self) -> &'static str {
        "whileCommand"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
