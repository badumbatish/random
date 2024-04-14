mod util;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input a string that is our directory
    let s = util::get_input_dir();
    let v = util::get_test_files(&s);

    for i in v {
        println!("{}", i);
    }
    Ok(())
}
