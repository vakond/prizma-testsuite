//! build.rs

fn main() {
    let cases = collect_testcases();
    generate_select_function(&cases);
    generate_verify_function(&cases);
    generate_check_function(&cases);
    println!("cargo:rerun-if-changed=build.rs");
}

use std::{collections::BTreeSet, env, fs, path::Path};

/// Collect all testcase filenames from the project tree
/// and generate code working with the actual set of testcases.
fn collect_testcases() -> BTreeSet<String> {
    let err = "Undefined variable CARGO_MANIFEST_DIR";
    let src_dir = env::var_os("CARGO_MANIFEST_DIR").expect(err);
    let testcases_path = Path::new(&src_dir).join("src").join("testcases");

    let err = format!("Failed to walk `{:?}`", testcases_path);
    let files = fs::read_dir(testcases_path).expect(&err);

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

/// Generate function which can iterate all available testcases and filter the list.
fn generate_select_function(cases: &BTreeSet<String>) {
    let mut text = String::from("pub fn select(selected: std::collections::HashSet<String>, excluded: std::collections::HashSet<String>) -> crate::error::Result<Vec<(usize, Box<dyn Test + Send>)>> {\n");
    text += "let s = !selected.is_empty();\n";
    text += "let x = !excluded.is_empty();\n";
    text += "assert!(!(s && x), \"mutually exclusive options\");\n";
    text += "check(&selected)?;\n";
    text += "check(&excluded)?;\n";
    text += "let mut tests = Vec::new();\n";

    for (index, item) in cases.iter().enumerate() {
        let test_name = Path::new(&item).with_extension("");
        text += "if (!s && !x)\n";
        text += &format!(
            "|| (s && (selected.contains(\"{}\") || selected.contains(\"{}\")))\n",
            index + 1,
            test_name.display()
        );
        text += &format!(
            "|| (x && !(excluded.contains(\"{}\") || excluded.contains(\"{}\")))\n",
            index + 1,
            test_name.display()
        );
        text += "{\n";
        text += &format!(
            "tests.push(({}, {}::get()));\n",
            index + 1,
            test_name.display()
        );
        text += "}\n";
    }

    text += "Ok(tests)\n";
    text += "}\n";

    dump_to_file(&text, "select_function.rs");
}

/// Generate function to verify that all testcases are consistent.
/// This function runs once before any processing.
fn generate_verify_function(cases: &BTreeSet<String>) {
    let mut text = String::from("pub fn verify() -> crate::error::Result<()> {\n");

    for item in cases.iter() {
        let test_name = Path::new(&item).with_extension("");
        text += &format!("let test = {}::get();\n", test_name.display());
        text += &format!("if test.name() != \"{}\" {{\n", test_name.display());
        text += &format!(
            "return Err(crate::error::Error::InconsistentTestcase(\"{}\", test.name()));\n",
            test_name.display()
        );
        text += "}\n";
    }

    text += "Ok(())\n";
    text += "}\n";

    dump_to_file(&text, "verify_function.rs");
}

/// Generate function to check correctness of a user input.
fn generate_check_function(cases: &BTreeSet<String>) {
    let mut text = String::from(
        "pub fn check(set: &std::collections::HashSet<String>) -> crate::error::Result<()> {\n",
    );

    text += "for item in set.iter() {\n";
    text += "if let Ok(n) = item.parse::<usize>() {\n";
    text += &format!("if (1..={}).contains(&n) {{\n", cases.len());
    text += "continue;\n";
    text += "} else {\n";
    text += "return Err(crate::error::Error::InvalidIndex(n));\n";
    text += "}\n";
    text += "}\n";
    text += "let mut found = false;\n";
    for item in cases.iter() {
        let test_name = Path::new(&item).with_extension("");
        text += &format!("if item == \"{}\" {{\n", test_name.display());
        text += "found = true;\n";
        text += "}\n";
    }
    text += "if !found {\n";
    text += "return Err(crate::error::Error::NameNotFound(item.to_string()));\n";
    text += "}\n";
    text += "}\n";

    text += "Ok(())\n";
    text += "}\n";

    dump_to_file(&text, "check_function.rs");
}

fn dump_to_file(text: &str, filename: &str) {
    let err = "Undefined variable OUT_DIR";
    let out_dir = env::var_os("OUT_DIR").expect(err);
    let dest_path = Path::new(&out_dir).join(filename);

    let err = format!("Cannot write to `{:?}`", dest_path);
    fs::write(&dest_path, text).expect(&err);
}
