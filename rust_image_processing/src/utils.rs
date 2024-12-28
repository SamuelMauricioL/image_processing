use std::fs;
use std::io;

pub fn list_files_in_folder(folder_path: &str) -> Result<Vec<String>, io::Error> {
    let mut files: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        files.push(file_name_str.to_string());
                    }
                }
            }
        }
        return Ok(files);
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Error al leer el directorio",
        ));
    }
}
