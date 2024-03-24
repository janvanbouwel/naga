use std::collections::HashMap;

use crate::types::{FunctionT, Type};

pub fn initial_context() -> HashMap<String, FunctionT> {
    HashMap::from([
        ("True".to_string(), FunctionT::new(&[], &[Type::Boolean])),
        ("False".to_string(), FunctionT::new(&[], &[Type::Boolean])),
        ("and".to_string(), {
            FunctionT::new(&[Type::Boolean, Type::Boolean], &[Type::Boolean])
        }),
        ("id".to_string(), {
            FunctionT::new(&[Type::Generic(0)], &[Type::Generic(0)])
        }),
        ("dup".to_string(), {
            FunctionT::new(&[Type::Generic(0)], &[Type::Generic(0), Type::Generic(0)])
        }),
    ])
}
