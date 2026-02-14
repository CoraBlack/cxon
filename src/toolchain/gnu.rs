use std::num::NonZero;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::thread::Thread;

use which::which;

use crate::cson;
use crate::object::output::{Execuable, Object, SharedLib, StaticLib};
use crate::object::source::Source;
use crate::toolchain::compiler::{Compiler, CompilerPair};
use crate::toolchain::linker::Linker;

pub struct GNU {

}

impl Compiler for GNU {
    fn get_compiler() -> CompilerPair {
        const GNU_CC: &str = "gcc";
        const GNU_CXX: &str = "g++";

        let cc = which(GNU_CC)
            .expect("Failed to find gcc")
            .to_str().unwrap().to_string();

        let cxx = which(GNU_CXX)
            .expect("Failed to find g++")
            .to_str().unwrap().to_string();
        

        CompilerPair {
            cc: cc,
            cxx: cxx,
        }
    }

    fn compile(input: Vec<Source>) -> Option<Object> {
        let cc_pair = Self::get_compiler();
        if input.is_empty() {
            return None;
        }

        let threads_count = cson::get_cson_config().read().unwrap().threads;
        let threads_count = match threads_count {
            Some(t) => t,
            None => match std::thread::available_parallelism() {
                Ok(t) => t,
                Err(_) => NonZero::new(1).unwrap()     
            }.into(),
        };
        let mut compile_threads = Vec::new();
        let src_rwlock = Arc::new(RwLock::new(input));

        for _ in 0..threads_count {
            let cc = cc_pair.cc.clone();
            let cxx = cc_pair.cxx.clone();
            let src_rwlock = Arc::clone(&src_rwlock);

            let thread = std::thread::spawn(move || {
                loop {
                    let src = src_rwlock.write().unwrap().pop();
                    match src {
                        Some(src) => {
                            let compiler = 
                                if src.get_path().extension().unwrap().to_str().unwrap() == "c" {
                                    &cc
                                } else {
                                    &cxx
                                };
                            
                            std::process::Command::new(compiler)
                                .arg("-g")
                                .arg("-c")
                                .arg(src.get_path())
                                .arg("-o")
                                .arg(src.get_path().with_extension("o"))
                                .spawn()
                                .expect(format!("Failed to compile {}", src.get_path().to_str().unwrap()).as_str());
                                },
                        None => break,
                    }
                }

            });

            compile_threads.push(thread);
        }

        for thread in compile_threads {
            thread.join().expect("Thread panic when compile sources files");
        }

        return None;
    }
}


impl Linker for GNU {
    fn get_linker() -> Option<String> {
        const GNU_LD: &str = "ld";
        return match which(GNU_LD) {
            Ok(ld_path) => {
                Some(ld_path.to_str().unwrap().to_string())
            },
            Err(_) => { None }
        };
    }

    fn link_to_object(&self, input: Vec<&Object>) -> Option<Object> {
        return None;
    }

    fn link_to_execuable(&self, input: Vec<&Object>) -> Option<Execuable> {
        return None;
    }

    fn link_to_static_lib(&self, input: Vec<&Object>) -> Option<StaticLib> {
        return None;
    }

    fn link_to_dynamic_lib(&self, input: Vec<&Object>) -> Option<SharedLib> {
        return None;
    }
}