use crate::datastructs::values::{Callable, Literal};
use crate::datastructs::exceptions::RuntimeException;
use crate::evaluator::Evaluator;

pub struct Class {
    pub name: String,
}

impl Class {
    pub fn new(name: String) -> Self {
        Class { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        0 // Classes don't take arguments in this simplified implementation
    }

    fn call(&self, _evaluator: &mut Evaluator, _arguments: &[Literal]) -> Result<Literal, RuntimeException> {
        // In a full implementation, this would create an instance of the class.
        // For now, we just return a placeholder value.
        Ok(Literal::Nil)
    }
}