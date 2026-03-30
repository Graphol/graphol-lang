use std::any::Any;

use crate::ast::{BooleanOp, LogicOp};
use crate::runtime::host::ExecutionHost;
use crate::runtime::object::{GrapholObject, ensure_node_value, object_ref};
use crate::runtime::value::{ObjectRef, ScalarValue, Value};

pub fn new_logic_operator(op: LogicOp) -> ObjectRef {
    object_ref(LogicOperatorStrategy::new(op))
}

pub fn new_boolean_operator(op: BooleanOp) -> ObjectRef {
    object_ref(BooleanOperatorStrategy::new(op))
}

struct LogicOperatorStrategy {
    op: LogicOp,
    value: Option<bool>,
    value_reference: Option<ObjectRef>,
}

impl LogicOperatorStrategy {
    fn new(op: LogicOp) -> Self {
        Self {
            op,
            value: None,
            value_reference: None,
        }
    }

    fn compare(&self, left: &ScalarValue, right: &ScalarValue) -> bool {
        match (left, right) {
            (ScalarValue::Number(a), ScalarValue::Number(b)) => match self.op {
                LogicOp::Eq => a == b,
                LogicOp::Ne => a != b,
                LogicOp::Gt => a > b,
                LogicOp::Lt => a < b,
                LogicOp::Ge => a >= b,
                LogicOp::Le => a <= b,
            },
            (ScalarValue::Text(a), ScalarValue::Text(b)) => match self.op {
                LogicOp::Eq => a == b,
                LogicOp::Ne => a != b,
                LogicOp::Gt => a > b,
                LogicOp::Lt => a < b,
                LogicOp::Ge => a >= b,
                LogicOp::Le => a <= b,
            },
            (ScalarValue::Bool(a), ScalarValue::Bool(b)) => match self.op {
                LogicOp::Eq => a == b,
                LogicOp::Ne => a != b,
                LogicOp::Gt => (*a as u8) > (*b as u8),
                LogicOp::Lt => (*a as u8) < (*b as u8),
                LogicOp::Ge => (*a as u8) >= (*b as u8),
                LogicOp::Le => (*a as u8) <= (*b as u8),
            },
            _ => false,
        }
    }
}

impl GrapholObject for LogicOperatorStrategy {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        let node_value = ensure_node_value(value, host);

        if self.value_reference.is_none() {
            self.value_reference = Some(node_value);
            return;
        }

        let reference = self.value_reference.as_ref().expect("checked above");
        if reference.borrow().get_type() != node_value.borrow().get_type() {
            self.value = Some(false);
            return;
        }

        let comparison = self.compare(
            &reference.borrow().get_value(),
            &node_value.borrow().get_value(),
        );
        self.value = Some(self.value.unwrap_or(true) && comparison);
    }

    fn toboolean(&self) -> bool {
        self.value.unwrap_or(false)
    }

    fn tonumber(&self) -> f64 {
        if self.toboolean() { 1.0 } else { 0.0 }
    }

    fn tostring(&self) -> String {
        self.toboolean().to_string()
    }

    fn get_type(&self) -> &'static str {
        "logicOperator"
    }

    fn get_value(&self) -> ScalarValue {
        ScalarValue::Bool(self.toboolean())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct BooleanOperatorStrategy {
    op: BooleanOp,
    value: Option<bool>,
    value_reference: Option<ObjectRef>,
}

impl BooleanOperatorStrategy {
    fn new(op: BooleanOp) -> Self {
        Self {
            op,
            value: None,
            value_reference: None,
        }
    }
}

impl GrapholObject for BooleanOperatorStrategy {
    fn receive(&mut self, value: Value, host: &mut dyn ExecutionHost) {
        let node_value = ensure_node_value(value, host);

        if self.op == BooleanOp::Not {
            self.value = Some(!node_value.borrow().toboolean());
            return;
        }

        if self.value_reference.is_none() {
            self.value_reference = Some(node_value);
            return;
        }

        let reference = self.value_reference.as_ref().expect("checked above");
        let left = reference.borrow().toboolean();
        let right = node_value.borrow().toboolean();

        self.value = Some(match self.op {
            BooleanOp::And => left && right,
            BooleanOp::Or => left || right,
            BooleanOp::Not => !right,
            BooleanOp::Xor => (left || right) && !(left && right),
        });
    }

    fn toboolean(&self) -> bool {
        self.value.unwrap_or(false)
    }

    fn tonumber(&self) -> f64 {
        if self.toboolean() { 1.0 } else { 0.0 }
    }

    fn tostring(&self) -> String {
        self.toboolean().to_string()
    }

    fn get_type(&self) -> &'static str {
        "booleanOperator"
    }

    fn get_value(&self) -> ScalarValue {
        ScalarValue::Bool(self.toboolean())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
