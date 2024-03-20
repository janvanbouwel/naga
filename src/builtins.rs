use std::collections::HashMap;

use crate::types::{FunctionT, Type};

type FunctionTypeProvider = fn() -> FunctionT;

fn hashmap_from<const N: usize>(
    entries: [(&'static str, FunctionTypeProvider); N],
) -> HashMap<&'static str, FunctionTypeProvider> {
    HashMap::from(entries)
}

pub fn builtin_context() -> HashMap<&'static str, FunctionTypeProvider> {
    hashmap_from([
        ("True", || FunctionT::new(&[], &[Type::Boolean])),
        ("False", || FunctionT::new(&[], &[Type::Boolean])),
        ("and", || {
            FunctionT::new(&[Type::Boolean, Type::Boolean], &[Type::Boolean])
        }),
    ])
}
