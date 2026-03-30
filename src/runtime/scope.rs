use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::object::{
    StdoutState, new_echo, new_if, new_input, new_message_async, new_message_else, new_message_run,
    new_node, new_stdout,
};
use super::value::ObjectRef;

pub type ScopeRef = Rc<RefCell<Scope>>;

pub struct Scope {
    values: HashMap<String, ObjectRef>,
    parent: Option<ScopeRef>,
}

impl Scope {
    pub fn new(parent: Option<ScopeRef>, stdout: StdoutState) -> ScopeRef {
        let mut values = HashMap::new();
        values.insert("input".to_string(), new_input());
        values.insert("run".to_string(), new_message_run());
        values.insert("stdout".to_string(), new_stdout(stdout.clone_ref()));
        values.insert("echo".to_string(), new_echo(stdout.clone_ref()));
        values.insert("async".to_string(), new_message_async());
        values.insert("if".to_string(), new_if());
        values.insert("else".to_string(), new_message_else());

        Rc::new(RefCell::new(Self { values, parent }))
    }

    pub fn find(scope: &ScopeRef, key: &str) -> Option<ObjectRef> {
        let mut current = Some(scope.clone());
        while let Some(node) = current {
            let borrowed = node.borrow();
            if let Some(value) = borrowed.values.get(key) {
                return Some(value.clone());
            }
            current = borrowed.parent.clone();
        }
        None
    }

    pub fn get(scope: &ScopeRef, key: &str) -> ObjectRef {
        if let Some(value) = Self::find(scope, key) {
            return value;
        }

        let created = new_node();
        scope
            .borrow_mut()
            .values
            .insert(key.to_string(), created.clone());
        created
    }

    pub fn set(scope: &ScopeRef, key: &str, value: ObjectRef) {
        scope.borrow_mut().values.insert(key.to_string(), value);
    }
}
