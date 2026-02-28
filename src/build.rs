use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::compile_commands_json::generate_compile_commands_json;
use crate::cxon::{CxonConfig, get_cxon_config};
use crate::object::output::ObjectCollection;
use crate::object::source::Source;
use crate::toolchain::{ToolChainTrait, compiler, linker};

fn build_cxon_project<T: ToolChainTrait>(cxon_config: CxonConfig) {

    let sources = cxon_config.sources.expect("Failed to read source files");
        
    let sources = Arc::new(Mutex::new(VecDeque::from(sources)));

    let objects = ObjectCollection {
        objects: Vec::new(),
    };
    let objects = Arc::new(Mutex::new(objects));

    let thread_count = match cxon_config.threads {
        Some(count) => count,
        None => num_cpus::get() as usize - 1,
    };

    let mut compile_threads = Vec::new();

    for _ in 0..thread_count {
        let sources = sources.clone();
        let objects = objects.clone();

        compile_threads.push(thread::spawn(move || {
            while let Some(source) = sources.lock().unwrap().pop_back() {
                let source = Source::new(source.clone().as_path());
                let obj = compiler::compile::<T>(source);
                objects.lock().unwrap().objects.push(obj);
            }
        }));
    }

    for thread in compile_threads {
        thread.join().unwrap();
    }

    linker::link::<T>(
        objects.lock().unwrap().clone(),
        get_cxon_config().read().unwrap().get_target_type(),
    );

    if cxon_config.export_compile_commands {
        generate_compile_commands_json().expect("Failed to export compile_commands.json")
    }
}
