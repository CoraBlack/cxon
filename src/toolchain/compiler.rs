use crate::object::{
    source::Source,
    output::Object
};

#[derive(Clone)]
pub struct CompilerPair {
    pub cc:  String,
    pub cxx: String,
}

pub trait Compiler {
    fn get_compiler() -> CompilerPair;

    fn compile(input: Vec<Source>) -> Option<Object>;
}

pub fn compile<T: Compiler>(input: Vec<Source>) -> Option<Object> {
    T::compile(input)
}