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
