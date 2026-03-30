use std::cell::RefCell;
use std::rc::Rc;

use super::object::GrapholObject;

pub type ObjectRef = Rc<RefCell<dyn GrapholObject>>;

#[derive(Clone)]
pub enum Value {
    Obj(ObjectRef),
    Number(f64),
    Text(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScalarValue {
    Number(f64),
    Text(String),
    Bool(bool),
    Null,
}

impl Value {
    pub fn as_text(&self) -> String {
        match self {
            Self::Obj(obj) => obj.borrow().tostring(),
            Self::Number(v) => v.to_string(),
            Self::Text(v) => v.clone(),
            Self::Bool(v) => v.to_string(),
            Self::Null => String::new(),
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Obj(obj) => Some(obj.borrow().tonumber()),
            Self::Number(v) => Some(*v),
            Self::Text(v) => v.parse::<f64>().ok(),
            Self::Bool(v) => Some(if *v { 1.0 } else { 0.0 }),
            Self::Null => None,
        }
    }

    pub fn as_bool(&self) -> bool {
        match self {
            Self::Obj(obj) => obj.borrow().toboolean(),
            Self::Number(v) => *v != 0.0,
            Self::Text(v) => !v.is_empty(),
            Self::Bool(v) => *v,
            Self::Null => false,
        }
    }

    pub fn to_scalar(&self) -> ScalarValue {
        match self {
            Self::Obj(obj) => obj.borrow().get_value(),
            Self::Number(v) => ScalarValue::Number(*v),
            Self::Text(v) => ScalarValue::Text(v.clone()),
            Self::Bool(v) => ScalarValue::Bool(*v),
            Self::Null => ScalarValue::Null,
        }
    }
}
