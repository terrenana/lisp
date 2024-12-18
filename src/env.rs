use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

impl Env {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }
    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(obj) = self.vars.get(name) {
            Some(obj.clone())
        } else {
            self.parent
                .as_ref()
                .and_then(|o| o.borrow().get(name).clone())
        }
    }

    pub fn set(&mut self, name: &str, obj: Object) {
        self.vars.insert(name.to_string(), obj);
    }
}
