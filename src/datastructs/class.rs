use std::fmt;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::datastructs::values::{Callable, Literal};
use crate::datastructs::exceptions::RuntimeException;
use crate::evaluator::Evaluator;
use crate::datastructs::instance::Instance;

pub struct Class {
    pub name: String,
    methods: HashMap<String, Rc<dyn Callable>>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Rc<dyn Callable>>) -> Self {
        Class { name, methods }
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl Callable for Class {
    fn bind(&self, _instance: Rc<RefCell<Instance>>) -> Rc<dyn Callable> {
        Rc::new(Class { name: self.name.clone(), methods: self.methods.clone() })
    }

    fn arity(&self) -> usize {
        if let Some(initializer) = self.methods.get("init").cloned() {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(&self, evaluator: &mut Evaluator, arguments: &[Literal]) -> Result<Literal, RuntimeException> {
        let instance = Rc::new(RefCell::new(Instance::new(self.name.clone(), self.methods.clone())));
        let initializer = self.methods.get("init").cloned();
        if let Some(initializer) = initializer {
            initializer.bind(instance.clone()).call(evaluator, arguments)?;
        }
        Ok(Literal::Instance(instance.clone()))
    }
}