use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // for each file within the abis/ dir, we need to write it as a constant string within src/abi_constants.rs file
    let path = Path::new("./abis");
    let files = fs::read_dir(path).unwrap();
    let mut abi_constants = String::new();
    for file in files  {
        let path = file.unwrap().path();
        let file_contents = fs::read_to_string(&path).unwrap();
        let file_name = path.file_stem().unwrap().to_str().unwrap().to_uppercase();
        abi_constants.push_str(&format!("pub const {}: &str = r#\"{}\"#;\n", file_name, file_contents));
    }

    fs::write("./src/abi_constants.rs", abi_constants).unwrap();
}
