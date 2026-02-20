use std::{fs, path::{Path, PathBuf}, sync::{LazyLock, RwLock}};

use serde::{Deserialize, Serialize};

use crate::{cli::arg};

static CONFIG: LazyLock<RwLock<CsonConfig>> = LazyLock::new(|| {
    RwLock::new({
        let arg = arg::get_args();
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
    #[serde(default = "default_build_dir")]
    pub build_dir: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output_dir: PathBuf,

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
        let file_path = if path.is_dir() {
            path.join("cson.json")
        } else {
            path.to_path_buf()
        };

        let content = fs::read_to_string(&file_path).expect(
            format!(
                "Failed to read cson.json file from {}",
                file_path.to_string_lossy()
            )
            .as_str(),
        );

        let cson: CsonConfig = serde_json::from_str(&content)
            .expect("Failed to parse cson configuration");

        if !cson.build_dir.exists() {
            fs::create_dir_all(&cson.build_dir).expect("Failed to clean build directory");
        }

        if !cson.output_dir.exists() {
            fs::create_dir_all(&cson.output_dir).expect("Failed to clean output directory");
        }

        cson
    }
}

fn default_build_dir() -> PathBuf {
    PathBuf::from("./build")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("./output")
}

#[test]
fn test_cson() {
    let config = CsonConfig::new("./cson.json".as_ref());
    println!("Project: {:?}", config);
}
