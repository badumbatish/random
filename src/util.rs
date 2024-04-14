use walkdir::WalkDir;

pub fn get_test_files(s: &str) -> Vec<String> {
    let mut test_files = Vec::new();
    for entry in WalkDir::new(s).into_iter() {
        let v = entry.unwrap();
        let ext = v.path().extension().unwrap_or_default();
        if ext.to_str().unwrap_or_default() == "rs" {
            test_files.push(v.path().display().to_string());
        }
    }
    test_files
}

pub fn get_input_dir() -> String {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Enter the directory: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    s
}
