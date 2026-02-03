use std::env;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Config {
    pub base: PathBuf,
    pub var: PathBuf,
    pub projects: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        let base = PathBuf::from(get_default_base_path());

        Self {
            var: base.join("var"),
            projects: base.join("projects"),
            base,
        }
    }

    pub fn setup(&self) {
        std::fs::create_dir_all(&self.var).unwrap();
        std::fs::create_dir_all(&self.projects).unwrap();
    }
}

fn get_default_base_path() -> String {
    match env::var("DRUCKR_BASE") {
        Ok(env_base) => env_base,
        Err(_) => env::current_dir().unwrap().to_string_lossy().to_string(),
    }
    // if let Ok(env_base) = env::var("SLOWHAND_BASE")
}
