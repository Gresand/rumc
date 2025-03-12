use std::fs;
use std::path::Path;

pub fn list_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(Path::new(path)) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                files.push(file_name);
            }
        }
    }
    files
}

#[allow(dead_code)]
pub fn copy_file(src: &str, dest: &str) -> std::io::Result<()> {
    fs::copy(src, dest).expect("Ошибка при копировании файла");
    Ok(())
}

#[allow(dead_code)]
pub fn delete_files(path: &str) -> std::io::Result<()> {
    fs::remove_file(path)?;
    Ok(())
}
