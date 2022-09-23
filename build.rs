//! build.rs

use std::{collections::BTreeSet, env, fs, io, path::Path};

/// Collect all testcase module filenames from the project
/// and create list of testcases for later use.
fn main() {
    let msg = "Undefined variable CARGO_MANIFEST_DIR";
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").expect(msg);
    let testcases_path = Path::new(&src_dir).join("src").join("testcases");

    let msg = format!("Failed to walk `{:?}`", testcases_path);
    let list = collect_testcases(&testcases_path).expect(&msg);

    let constant = format!("const TESTCASES: &str = r#\"{list}\"#;");

    let msg = "Undefined variable OUT_DIR";
    let out_dir = env::var_os("OUT_DIR").expect(msg);
    let dest_path = Path::new(&out_dir).join("testcases.txt");

    let msg = format!("Cannot write to `{:?}`", dest_path);
    fs::write(&dest_path, &constant).expect(&msg);

    println!("cargo:rerun-if-changed=build.rs");
}

fn collect_testcases(dir: &Path) -> io::Result<String> {
    let files = fs::read_dir(dir)?;

    let mut items = BTreeSet::new();
    for file in files {
        let item = file.unwrap().path();
        let filename = item
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if !filename.is_empty() && filename != "mod.rs" {
            items.insert(filename.to_owned());
        }
    }

    let mut list = String::new();
    for (index, item) in items.into_iter().enumerate() {
        let test_name = Path::new(&item).with_extension("");
        list += &format!("{}\t{}\n", index + 1, test_name.display());
    }
    list.pop();

    Ok(list)
}
