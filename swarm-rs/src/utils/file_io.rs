use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use std::fs;
use std::path::PathBuf;
use url::Url;
use walkdir::{DirEntry, WalkDir};

use super::time;


pub fn create_parent_dirs<P: AsRef<Path>>(path: P) {
    let pathbuf = PathBuf::from(path.as_ref());
    if let Some(parent) = pathbuf.parent() {
        fs::create_dir_all(parent).expect("Unable to create db directory");
    }
}

pub fn read_text_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(&path).expect(&format!("unable to read file {:?}", path.as_ref()));
    let mut string_content = String::new();
    file.read_to_string(&mut string_content)
        .expect(&format!("unable to read file content{:?}", path.as_ref()));
    string_content
}

pub fn write_text_file<P: AsRef<Path>>(path: P, content: &str) {
    create_parent_dirs(&path);
    let mut f = File::create(&path).expect("No Error");
    write!(&mut f, "{}", content).expect("");
}

pub fn write_binary_file<P: AsRef<Path>>(path: P, buffer: &Vec<u8>) {
    create_parent_dirs(&path);
    let pathbuf = PathBuf::from(path.as_ref());
    let mut f = File::create(path).expect("No Error");
    f.write_all(buffer)
        .expect(&format!("unable to read file content{:?}", pathbuf));
}

pub fn read_binary_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, String> {
    let pathbuf = PathBuf::from(path.as_ref());
    if let Ok(buffer) = fs::read(pathbuf) {
        Ok(buffer)
    } else {
        Err("Unable to read file".to_string())
    }
}

pub fn remove_file(path: &str) {
    let _ = fs::remove_file(path);
}

pub fn remove_dir<P: AsRef<Path>>(path: P) {
    let _ = fs::remove_dir_all(path.as_ref());
}

pub fn get_absolute_path<P: AsRef<Path>>(relative_path: P) -> Result<PathBuf, std::io::Error> {
    let path = PathBuf::from(relative_path.as_ref());
    match fs::canonicalize(&path) {
        Ok(path) => Ok(path),
        Err(error) => Err(error),
    }
}

pub fn format_absolute_path_str(path: &PathBuf) -> Result<String, std::io::Error> {
    match fs::canonicalize(&path) {
        Ok(path) => {
            let path = path.to_str().unwrap();
            let url = if path.starts_with("\\\\?\\") {
                let formatted_url = path.replacen("\\\\?\\", "", 1);
                formatted_url
            } else {
                format!("{}", path)
            };
            Ok(url)
        }
        Err(error) => Err(error),
    }
}

pub fn get_absolute_path_str(relative_path: &str) -> Result<String, std::io::Error> {
    let path = PathBuf::from(relative_path);
    match fs::canonicalize(&path) {
        Ok(path) => {
            let path = path.to_str().unwrap();
            let url = if path.starts_with("\\\\?\\") {
                let formatted_url = path.replacen("\\\\?\\", "", 1);
                formatted_url
            } else {
                format!("{}", path)
            };
            Ok(url)
        }
        Err(error) => Err(error),
    }
}

pub fn format_os_path(path: &str) -> String {
    let url = if path.starts_with("\\\\?\\") {
        let formatted_url = path.replacen("\\\\?\\", "", 1);
        formatted_url
    } else {
        format!("file://{}", path)
    };
    url
}

pub fn get_file_url_from_str(relative_path: &str) -> Result<String, std::io::Error> {
    let path = PathBuf::from(relative_path);
    match fs::canonicalize(&path) {
        Ok(path) => {
            let url = Url::from_file_path(&path);
            match url {
                Ok(url) => Ok(String::from(url.as_str())),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    relative_path,
                )),
            }
        }
        Err(error) => Err(error),
    }
}

pub fn get_file_url_from_path(relative_path: &PathBuf) -> Result<String, std::io::Error> {
    match fs::canonicalize(relative_path) {
        Ok(path) => {
            let url = format_os_path(path.to_str().unwrap());
            Ok(url)
        }
        Err(error) => Err(error),
    }
}

pub fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

pub fn is_file<P: AsRef<Path>>(f: P) -> bool {
    let f_p = PathBuf::from(f.as_ref());
    match f_p.metadata() {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

pub fn list_folders<P: AsRef<Path>>(root: P, max_depth: Option<usize>) -> Vec<PathBuf> {
    let wd = WalkDir::new(&root);
    let wd = if let Some(max_depth) = max_depth {
        wd.max_depth(max_depth)
    } else {
        wd
    };
    let files: Vec<PathBuf> = wd
        .min_depth(1)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|entry| entry.ok())
        // .filter_map(|entry| entry.metadata().ok())
        .filter(|entry| match entry.metadata().ok() {
            Some(metadata) => {
                if let Ok(_time) = metadata.modified() {
                    // println!("{time:?}");
                } else {
                    println!("Not supported on this platform");
                }
                metadata.is_dir()
            }
            None => false,
        })
        .map(|entry| entry.into_path())
        .collect();
    files
}

pub fn list_files<P: AsRef<Path>>(root: P, max_depth: Option<usize>) -> Vec<PathBuf> {
    let wd = WalkDir::new(&root);
    let wd = if let Some(max_depth) = max_depth {
        wd.max_depth(max_depth)
    } else {
        wd
    };
    let files: Vec<PathBuf> = wd
        .min_depth(1)
        // .max_depth(3)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|entry| entry.ok())
        // .filter_map(|entry| entry.metadata().ok())
        .filter(|entry| match entry.metadata().ok() {
            Some(metadata) => {
                if let Ok(_time) = metadata.modified() {
                    // println!("{time:?}");
                } else {
                    println!("Not supported on this platform");
                }
                metadata.is_file()
            }
            None => false,
        })
        .map(|entry| entry.into_path())
        .collect();
    files
}

pub fn get_created_date(path: &PathBuf) -> Option<String> {
    let metadata = path.metadata().unwrap();
    let time = metadata.created();
    match time {
        Ok(time) => Some(time::system_time_to_iso(time)),
        Err(_) => None,
    }
}

pub fn get_modified_date(path: &PathBuf) -> String {
    let metadata = path.metadata().unwrap();
    let time = metadata.modified();
    // system_time_to_iso(time)
    match time {
        Ok(time) => time::system_time_to_iso(time),
        Err(_) => time::current_time_iso(),
    }
}

pub fn get_metadata<P: AsRef<Path>>(path: P) -> HashMap<String, String> {
    let path = PathBuf::from(path.as_ref());
    let file_metadata = path.metadata().unwrap();
    let mut metadata: HashMap<String, String> = HashMap::new();

    if let Ok(value) = file_metadata.modified() {
        metadata.insert("modification_date".to_string(), time::system_time_to_iso(value));
    }
    if let Ok(value) = file_metadata.created() {
        metadata.insert("creation_date".to_string(), time::system_time_to_iso(value));
    }
    metadata
}

pub fn get_accessed_date(path: &PathBuf) -> String {
    let metadata = path.metadata().unwrap();
    let time = metadata.accessed().unwrap();
    time::system_time_to_iso(time)
}

pub fn get_size(path: &PathBuf) -> u64 {
    if let Ok(metadata) = path.metadata() {
        metadata.len()
    } else {
        0
    }
}

pub fn get_file_extension<P: AsRef<Path>>(path: &P) -> Result<String, String> {
    let p: &Path = path.as_ref();
    if let Some(os_sxt) = p.extension() {
        if let Some(ext) = os_sxt.to_str() {
            return Ok(ext.to_string());
        }
    }
    Err("Unable to get file extension".to_string())
}
