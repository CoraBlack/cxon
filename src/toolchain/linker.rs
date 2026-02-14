use crate::object::output::{Execuable, Object, SharedLib, StaticLib};

pub trait Linker {
    fn get_linker() -> Option<String>;

    fn link_to_object(&self, input: Vec<&Object>) -> Option<Object>;
    fn link_to_execuable(&self, input: Vec<&Object>) -> Option<Execuable>;
    fn link_to_static_lib(&self, input: Vec<&Object>) -> Option<StaticLib>;
    fn link_to_dynamic_lib(&self, input: Vec<&Object>) -> Option<SharedLib>;
}