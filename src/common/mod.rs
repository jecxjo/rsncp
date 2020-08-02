use std::fs::metadata;
use glob::glob;

pub type Destination = String;

pub type File = String;

pub fn validate_files(files: &Vec<File>) -> Result<Vec<File>, String> {
    let mut new_files = Vec::new();

    for file_name in files.iter() {
        let file = metadata(file_name);

        if let Ok(file) = file {
            if file.is_file() {
                new_files.push(file_name.clone())
            } else {
                for entry in glob(format!("{}/**/*", file_name).as_str()).unwrap() {
                    match entry {
                        Ok(path) => {
                            if let Ok(path_name) = metadata(path.as_path()) {
                                if path_name.is_file() {
                                    new_files.push(String::from(path.as_path().to_str().unwrap()));
                                }
                            }
                        },
                        Err(_) => return Err(format!("Error traversing {}", file_name)),
                    }
                }
            }
        } else {
            return Err(format!("Error finding {}", file_name))
        }
    }

    Ok(new_files)
}
