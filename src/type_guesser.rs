use std::path::PathBuf;

/// The `FileType` type. Represents the different kind of files that can be reached from a m3u playlist.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    /// A Manifest file
    Manifest,
    /// A Segment file (or content file)
    Segment,
}

impl FileType {
    /// Returns an uppercase string representation of the file type
    pub fn uppercase_string(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }
}

/// Returns a potential file type, guessed from the file extension
///
/// # Examples
///
/// ```rust
/// assert_eq!(guess_file_type("test.m3u8"), Some(FileType::Manifest))
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_uppercase() {
        assert_eq!(FileType::Manifest.uppercase_string(), "MANIFEST");
        assert_eq!(FileType::Segment.uppercase_string(), "SEGMENT");
    }

    #[test]
    fn test_guess_file_type() {
        assert_eq!(guess_file_type("test.m3u8"), Some(FileType::Manifest));
        assert_eq!(guess_file_type("test.m3u"), Some(FileType::Manifest));
        assert_eq!(guess_file_type("test.ts"), Some(FileType::Segment));
        assert_eq!(guess_file_type("test.pdf"), None);

        assert_eq!(guess_file_type("/test.m3u8"), Some(FileType::Manifest));
        assert_eq!(
            guess_file_type("https://test.io/test.m3u8"),
            Some(FileType::Manifest)
        );
    }
}
