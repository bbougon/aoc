use std::fs;

pub fn file_content(file_path: &String) -> String {
    let mut file_content = fs::read_to_string(file_path)
        .unwrap_or_else(|_| panic!("Should have been able to read the file '{}'", file_path));
    file_content.pop();
    file_content
}
