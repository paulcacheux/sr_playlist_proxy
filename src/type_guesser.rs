use std::path::PathBuf;

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Manifest,
    Segment,
}

impl FileType {
    pub fn uppercase_string(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }
}

pub fn guess_file_type(path: &str) -> Option<FileType> {
    let file_path = PathBuf::from(path);
    if let Some(extension) = file_path.extension() {
        let extension = extension.to_string_lossy();
        if extension == "ts" {
            Some(FileType::Segment)
        } else if extension.starts_with("m3u") {
            // m3u or m3u8
            Some(FileType::Manifest)
        } else {
            None
        }
    } else {
        None
    }
}
