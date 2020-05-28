use std::{path::PathBuf, process::Command};

use lang_tester::LangTester;
use tempfile::TempDir;

#[test]
fn test() {
    let tempdir = TempDir::new().expect("temp dir");
    LangTester::new()
        .test_dir("tests/run")
        .test_file_filter(|path| path.extension().expect("extension").to_str().expect("to_str") == "rs")
        .test_extract(|source| {
            let lines =
                source.lines()
                    .skip_while(|l| !l.starts_with("//"))
                    .take_while(|l| l.starts_with("//"))
                    .map(|l| &l[2..])
                    .collect::<Vec<_>>()
                    .join("\n");
            println!("{:?}", lines);
            Some(lines)
        })
        .test_cmds(move |path| {
            // Test command 1: Compile `x.rs` into `tempdir/x`.
            let mut exe = PathBuf::new();
            exe.push(&tempdir);
            exe.push(path.file_stem().expect("file_stem"));
            let mut compiler = Command::new("rustc");
            compiler.args(&[
                "-o", exe.to_str().expect("to_str"),
                path.to_str().expect("to_str"),
            ]);
            // Test command 2: run `tempdir/x`.
            let runtime = Command::new(exe);
            vec![("Compiler", compiler), ("Run-time", runtime)]
        })
        .run();
}
