use std::any::Any;

use crate::ast::ArithmeticOp;
use crate::runtime::host::ExecutionHost;
use crate::runtime::object::{GrapholObject, object_ref};
use crate::runtime::value::{ObjectRef, ScalarValue, Value};

pub fn new_number(initial: Option<f64>) -> ObjectRef {
    object_ref(NumberStrategy::new(initial))
}

pub fn new_operator(op: ArithmeticOp) -> ObjectRef {
    object_ref(OperatorStrategy::new(op))
}

struct NumberStrategy {
    value: f64,
    strategy: ObjectRef,
}

impl NumberStrategy {
    fn new(initial: Option<f64>) -> Self {
        let strategy = new_operator(ArithmeticOp::Add);
        let value = initial.unwrap_or(0.0);
        if value != 0.0 {
            strategy
                .borrow_mut()
                .receive(Value::Number(value), &mut NoopHost);
        }
        Self { value, strategy }
    }
}

impl GrapholObject for NumberStrategy {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        if let Value::Text(op) = &value {
            if matches!(op.as_str(), "+" | "-" | "*" | "/") {
                self.value = self.strategy.borrow().tonumber();
                self.strategy = new_operator(match op.as_str() {
                    "+" => ArithmeticOp::Add,
                    "-" => ArithmeticOp::Sub,
                    "*" => ArithmeticOp::Mul,
                    _ => ArithmeticOp::Div,
                });
                self.strategy
                    .borrow_mut()
                    .receive(Value::Number(self.value), host);
                return;
            }
        }

        if let Value::Obj(obj) = &value {
            if obj.borrow().get_type() == "operator" {
                self.strategy = obj.clone();
                self.strategy
                    .borrow_mut()
                    .receive(Value::Number(self.value), host);
                self.value = self.strategy.borrow().tonumber();
                return;
            }
        }

        if let Some(number) = value.as_number() {
            self.strategy
                .borrow_mut()
                .receive(Value::Number(number), host);
            self.value = self.strategy.borrow().tonumber();
        }
    }

    fn tonumber(&self) -> f64 {
        self.value
    }

    fn tostring(&self) -> String {
        self.value.to_string()
    }

    fn get_type(&self) -> &'static str {
        "number"
    }

    fn get_value(&self) -> ScalarValue {
        ScalarValue::Number(self.value)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct OperatorStrategy {
    op: ArithmeticOp,
    value: Option<f64>,
}

impl OperatorStrategy {
    fn new(op: ArithmeticOp) -> Self {
        Self { op, value: None }
    }
}

impl GrapholObject for OperatorStrategy {
    fn receive(&mut self, value: Value, _host: &mut dyn ExecutionHost) {
        let Some(next_value) = value.as_number() else {
            return;
        };

        self.value = Some(match self.value {
            None => next_value,
            Some(current) => match self.op {
                ArithmeticOp::Add => current + next_value,
                ArithmeticOp::Sub => current - next_value,
                ArithmeticOp::Mul => current * next_value,
                ArithmeticOp::Div => current / next_value,
                ArithmeticOp::Xor => ((current as i64) ^ (next_value as i64)) as f64,
            },
        });
    }

    fn tonumber(&self) -> f64 {
        self.value.unwrap_or(0.0)
    }

    fn tostring(&self) -> String {
        self.tonumber().to_string()
    }

    fn get_type(&self) -> &'static str {
        "operator"
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct NoopHost;

impl ExecutionHost for NoopHost {
    fn read_input(&mut self, _prompt: &str) -> String {
        String::new()
    }

    fn emit_output(&mut self, _mode: crate::runtime::io::OutputMode, _value: &str) {}

    fn call_block(&mut self, _block: crate::runtime::object::BlockSnapshot) {}
}
