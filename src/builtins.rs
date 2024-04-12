use std::collections::HashMap;

use crate::types::{Function, Type};

pub fn initial_context() -> HashMap<String, Function> {
    HashMap::from([
        ("True".to_string(), Function::new(&[], &[Type::Bool])),
        ("False".to_string(), Function::new(&[], &[Type::Bool])),
        ("and".to_string(), {
            Function::new(&[Type::Bool, Type::Bool], &[Type::Bool])
        }),
        ("id".to_string(), {
            Function::new(&[Type::Gen(0)], &[Type::Gen(0)])
        }),
        ("dup".to_string(), {
            Function::new(&[Type::Gen(0)], &[Type::Gen(0), Type::Gen(0)])
        }),
    ])
}
