use std::path::{Path, PathBuf};

use rocket::{fs::NamedFile, get, State};

pub struct SPA {
    ui_dir: String,
    resources_dir: String
}

impl SPA {

    pub fn default() -> Self {
        Self{
            ui_dir: "./ui".to_string(),
            resources_dir: "./resources".to_string()
        }
    }

    #[allow(unused)]
    pub fn new(ui_dir: &str, resources_dir: &str) -> Self {
        Self{
            ui_dir: ui_dir.to_string(),
            resources_dir: resources_dir.to_string()
        }
    }
}

#[get("/<file..>")]
pub async fn app_index(file: PathBuf, spa: &State<SPA>) -> Option<NamedFile> {
    let p = Path::new(&spa.ui_dir).join(file);
    if p.exists() && p.is_file() {
        NamedFile::open(p).await.ok()
    } else {
        let p = Path::new(&spa.ui_dir).join("index.html");
        NamedFile::open(p).await.ok()
    }
}

#[get("/resources/<file..>")]
pub async fn app_resources(file: PathBuf,spa: &State<SPA>) -> Option<NamedFile> {
    let p = Path::new(&spa.resources_dir).join(file);
    if p.exists() && p.is_file() {
        NamedFile::open(p).await.ok()
    } else {
        None
    }
}