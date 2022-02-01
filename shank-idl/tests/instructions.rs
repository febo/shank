use std::path::{Path, PathBuf};

use shank_idl::{idl::Idl, parse_file};

fn fixtures_dir() -> PathBuf {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    root_dir.join("tests").join("fixtures").join("instructions")
}

#[test]
fn instruction_from_single_file_no_args() {
    let file = fixtures_dir()
        .join("single_file")
        .join("instruction_no_args.rs");
    let idl = parse_file(&file, "1.0.0".to_string())
        .expect("Parsing should not fail")
        .expect("File contains IDL");
    // eprintln!("{}", serde_json::to_string_pretty(&idl).unwrap());

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/instructions/single_file/instruction_no_args.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}

#[test]
fn instruction_from_single_file_with_args() {
    let file = fixtures_dir()
        .join("single_file")
        .join("instruction_with_args.rs");
    let idl = parse_file(&file, "1.0.0".to_string())
        .expect("Parsing should not fail")
        .expect("File contains IDL");

    let expected_idl: Idl = serde_json::from_str(include_str!(
        "./fixtures/instructions/single_file/instruction_with_args.json"
    ))
    .unwrap();

    assert_eq!(idl, expected_idl);
}