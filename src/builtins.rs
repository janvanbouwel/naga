use std::{collections::HashMap, rc::Rc};

use crate::{
    stacklike::Store,
    types::{FTy, Function, Ty},
};

pub fn initial_context() -> Store {
    let mut types: Store = HashMap::new();

    types.insert("'".to_string(), Rc::new(|| Function::from_fty(&[], &[])));
    types.insert(
        "?".into(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::from_fty(&[Ty::Bool.into(), gen.clone(), gen.clone()], &[gen.clone()])
        }),
    );
    types.insert(
        "dup".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::from_fty(&[gen.clone()], &[gen.clone(), gen.clone()])
        }),
    );
    types.insert(
        "_".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::from_fty(&[gen], &[])
        }),
    );

    types.insert(
        "=".to_string(),
        Rc::new(|| {
            let gen = FTy::new_gen();
            Function::from_fty(&[gen.clone(), gen.clone()], &[Ty::Bool.into()])
        }),
    );

    let math = Rc::new(|| Function::from_fty(&[Ty::Int.into(), Ty::Int.into()], &[Ty::Int.into()]));
    types.insert("add".to_string(), math.clone());
    types.insert("sub".to_string(), math.clone());
    types.insert("mul".to_string(), math.clone());

    types
}
