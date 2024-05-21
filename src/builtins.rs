use std::{collections::HashMap, rc::Rc};

use crate::{
    stacklike::Store,
    types::{FTy, Function, Ty},
};

pub fn initial_context() -> Store {
    let mut types: Store = HashMap::new();

    types.insert(
        "True".to_string(),
        Rc::new(|| Function::new(&[], &[Ty::Bool.into()])),
    );

    types.insert(
        "False".to_string(),
        Rc::new(|| Function::new(&[], &[Ty::Bool.into()])),
    );
    types.insert(
        "and".to_string(),
        Rc::new(|| Function::new(&[Ty::Bool.into(), Ty::Bool.into()], &[Ty::Bool.into()])),
    );
    types.insert(
        "not".to_string(),
        Rc::new(|| Function::new(&[Ty::Bool.into()], &[Ty::Bool.into()])),
    );
    types.insert(
        "id".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[gen.clone()], &[gen.clone()])
        }),
    );
    types.insert(
        "dup".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[gen.clone()], &[gen.clone(), gen.clone()])
        }),
    );
    types.insert(
        "drop".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[gen], &[])
        }),
    );
    types.insert(
        "eq".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[gen.clone(), gen.clone()], &[Ty::Bool.into()])
        }),
    );
    types.insert(
        "test".into(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[Ty::Bool.into(), gen.clone(), gen.clone()], &[gen.clone()])
        }),
    );

    types
}
