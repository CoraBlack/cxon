use std::{error::Error, fs, path::{Path, PathBuf}, sync::{LazyLock, Mutex, RwLock}};

use serde::{Deserialize, Serialize};

use crate::cli::arg::CliArgs;

static CONFIG: LazyLock<RwLock<CsonConfig>> = LazyLock::new(|| {
    RwLock::new({
        let arg = CliArgs::new();
        let path = arg.project_dir;

        CsonConfig::new(path.as_path())
    })
});

pub fn get_cson_config() -> &'static RwLock<CsonConfig> {
    &CONFIG
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CsonConfig {
    // project settings
    pub project: String,
    pub target_name: String,
    pub cc: String,
    pub cxx: String,

    // building settings
    pub threads: Option<usize>,

    // temp directory
    pub build_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,

    // compiler flags
    pub flags:    Option<Vec<String>>,
    pub cflags:   Option<Vec<String>>,
    pub cxxflags: Option<Vec<String>>,

    // source files
    pub sources: Option<Vec<PathBuf>>,

    // compiler defines and includes
    pub defines: Option<Vec<String>>,
    pub include: Option<Vec<PathBuf>>,
    pub link:    Option<Vec<String>>,
    pub libs:    Option<Vec<String>>,
}

impl CsonConfig {
    pub fn new(path: &Path) -> CsonConfig {
        let cson = fs::read_to_string(path)
            .expect(format!("Failed to read cson.json file from {}", path.to_str().unwrap()).as_str());
        let cson = serde_json::from_str(&cson)
            .expect("Failed to parse cson configration");

        cson
    }
}
