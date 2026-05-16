use std::path::{Path, PathBuf};

pub fn upload_file_path(filename: &str) -> Option<PathBuf> {
    if filename.is_empty()
        || filename.contains("..")
        || filename.contains('/')
        || filename.contains('\\')
    {
        return None;
    }

    let path = Path::new("uploads").join(filename);

    if path.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return None;
    }

    Some(path)
}
