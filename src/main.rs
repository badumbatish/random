use walkdir::WalkDir;

fn get_test_files(s: &str) -> Vec<String> {
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input a string that is our directory
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

    let v = get_test_files(&s);

    for i in v {
        println!("{}", i);
    }
    Ok(())
}
