use log::{debug, error};
use std::fs;
use std::path::PathBuf;
use image::Rgba;

#[derive(Copy, Clone)]
pub struct GridImageConfig {
    pub cell_size: u32,
    pub columns: u32,
    pub stroke: u32,
    pub grid_color: Rgba<u8>,
}

pub fn search_files(path: PathBuf) -> Vec<PathBuf> {
    debug!("Searching for files in {:?}", &path);
    let mut result: Vec<PathBuf> = Vec::new();
    let Ok(entries) = fs::read_dir(&path) else {
        error!("Could not read directory {:?}", path);
        return result;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            error!("{:?}", entry);
            continue;
        };
        let Ok(meta) = entry.metadata() else {
            error!("failed get metadata {:?}", entry);
            continue;
        };
        if meta.is_file() {
            result.push(entry.path());
        }
        if meta.is_dir() {
            let mut buf = search_files(entry.path());
            result.append(&mut buf);
        }
    }
    result
}
