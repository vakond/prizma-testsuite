//! build.rs

use std::{collections::BTreeSet, env, fs, path::Path};

fn main() {
    let cases = collect_testcases();
    generate_constant(&cases);
    generate_select_function(&cases);
    println!("cargo:rerun-if-changed=build.rs");
}

/// Collect all testcase filenames from the project tree
/// and generate code working with the actual set of testcases.
fn collect_testcases() -> BTreeSet<String> {
    let msg = "Undefined variable CARGO_MANIFEST_DIR";
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").expect(msg);
    let testcases_path = Path::new(&src_dir).join("src").join("testcases");

    let msg = format!("Failed to walk `{:?}`", testcases_path);
    let files = fs::read_dir(testcases_path).expect(&msg);

    let mut sorted_items = BTreeSet::new();
    for file in files {
        let item = file.unwrap().path();
        let filename = item
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        if !filename.is_empty() && filename != "mod.rs" {
            sorted_items.insert(filename.to_owned());
        }
    }

    sorted_items
}

fn generate_constant(cases: &BTreeSet<String>) {
    let mut list = String::new();
    for (index, item) in cases.iter().enumerate() {
        let test_name = Path::new(&item).with_extension("");
        list += &format!("{}\t{}\n", index + 1, test_name.display());
    }
    list.pop();

    let constant = format!("const TESTCASES: &str = r#\"{list}\"#;");

    let msg = "Undefined variable OUT_DIR";
    let out_dir = env::var_os("OUT_DIR").expect(msg);
    let dest_path = Path::new(&out_dir).join("constant_list_of_testcases.rs");

    let msg = format!("Cannot write to `{:?}`", dest_path);
    fs::write(&dest_path, &constant).expect(&msg);
}

fn generate_select_function(_cases: &BTreeSet<String>) {
    //    let msg = "Undefined variable OUT_DIR";
    //    let out_dir = env::var_os("OUT_DIR").expect(msg);
    //    let dest_path = Path::new(&out_dir).join("select_function.rs");
    //
    //    let msg = format!("Cannot write to `{:?}`", dest_path);
    //    fs::write(&dest_path, &select_function).expect(&msg);
}
