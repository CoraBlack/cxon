use crate::{cxon::get_cxon_config, object::{output::ObjectCollection, source::Source}, toolchain::{ToolChain, ToolChainTrait, gnu::GNU, llvm::LLVM, msvc::MSVC}};

pub mod cli {
    pub mod arg;
}
pub mod object {
    pub mod output;
    pub mod source;
}
pub mod toolchain;
pub mod utils;
pub mod cxon;

fn main() -> () {
    let toolchain = get_cxon_config().read().unwrap().get_toolchain();
    match toolchain {
        ToolChain::GNU()  => build_project::<GNU>(),
        ToolChain::LLVM() => build_project::<LLVM>(),
        ToolChain::MSVC() => build_project::<MSVC>(),
    }
}

fn build_project<T: ToolChainTrait>() {
    let cxon = cxon::get_cxon_config();

    let source_paths = cxon
        .read()
        .unwrap()
        .sources
        .clone()
        .expect("No source files specified in cxon configuration");

    let mut objects = ObjectCollection{
        objects: Vec::new(),
    };

    for path in source_paths {
        let source = Source::new(&path);
        let obj = toolchain::compiler::compile::<T>(source);
        objects.objects.push(obj);
    }

    toolchain::linker::link_to_execuable::<T>(objects);
}
