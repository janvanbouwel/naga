use std::{collections::HashMap, rc::Rc};

use crate::{
    stacklike::Store,
    types::{FTy, Function, Ty},
};

pub fn initial_context() -> Store {
    let mut types: Store = HashMap::new();

    types.insert("'".to_string(), Rc::new(|| Function::new(&[], &[])));
    types.insert(
        "?".into(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[Ty::Bool.into(), gen.clone(), gen.clone()], &[gen.clone()])
        }),
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
        "=".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::new(&[gen.clone(), gen.clone()], &[Ty::Bool.into()])
        }),
    );

    let math = Rc::new(|| Function::new(&[Ty::Int.into(), Ty::Int.into()], &[Ty::Int.into()]));
    types.insert("add".to_string(), math.clone());
    types.insert("sub".to_string(), math.clone());

    types
}
