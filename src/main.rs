use std::{env::current_dir, error::Error, fs, path::PathBuf, process::Stdio};

use crate::{cli::arg, object::source::Source, toolchain::gnu::GNU};

pub mod cli {
    pub mod app;
    pub mod arg;
}

pub mod compile {

}

pub mod object {
    pub mod output;
    pub mod source;
}

pub mod toolchain {
    pub mod compiler;
    pub mod linker;
    pub mod gnu;
    pub mod llvm;
    pub mod msvc;
}

pub mod utils {

}

pub mod cson;

fn main() -> () {
    let cson = cson::get_cson_config();

    let source_paths = &cson.read().unwrap().sources.clone().expect("No source file");
    let mut sources = Vec::new();
    for path in source_paths {
        let source = Source::new(path);
        sources.push(source);
    }

    toolchain::compiler::compile::<GNU>(sources);
}


#[test]
fn build_cson() -> Result<(), Box<dyn Error>> {
    println!("Cson start, handling the args...");
    let args = cli::arg::CliArgs::new();
    let project_dir = args.project_dir;
    println!("project_dir: {:?}", project_dir);

    let cson = cson::get_cson_config();

    println!("{:?}", cson);

    let Some(sources) = cson.read().unwrap().sources.as_ref().cloned() else {
        return Err("No sources in cson file".into());
    };

    let mut compile = std::process::Command::new("g++")
        .arg("-g")
        .arg({
            let mut src_str = String::new();
            for src in sources.iter() {
                if let Some(s) = src.to_str() {
                    src_str.push_str(s);
                    src_str.push_str(" ");
                }
            }
            src_str
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let status = compile.wait()?;

    println!("status: {}", status);

    Ok(())
}

