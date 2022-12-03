use std::fs;

pub fn file_content(file_path: &String) -> String {
    fs::read_to_string(file_path).expect(&*format!(
        "Should have been able to read the file '{}'",
        file_path
    ))
}
