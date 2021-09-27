use glob::glob;
use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::logger::{log, LogLevel, soft_panic};

/// Check if the file contents is the same as the expected String.
/// Returns (file_contents, expected) if there is a difference. Returns None otherwise.
pub fn diff_file_str(filename: &Path, expected: &String) -> Option<(String, String)> {
    let file_contents = fs::read_to_string(filename).expect(format!("Something went wrong reading the file {}. Check if you have all the permissions and that the file exists.", filename.display()).as_str());
    if file_contents == *expected {
        return None;
    }
    return Some((file_contents, expected.clone()));
}

/// Executes the Python program and returns the output.
pub fn run_py_and_capture_output(filename: &Path, input: &Path) -> io::Result<String> {
    let input_contents = fs::read_to_string(input)?;

    let mut process = Command::new("python3")
        .args(&[filename])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    process
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input_contents.as_bytes())?;

    let output = process.wait_with_output()?;
    let result = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
    Ok(result)
}

pub fn get_test_files(dir: &Path) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    if !dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "The directory does not exist.",
        ));
    }
    let mut infiles = Vec::new();
    let mut outfiles = Vec::new();
    let glob_exp = glob(dir.join("*.in").to_str().unwrap()).expect("Failed to read glob pattern.");
    for infile in glob_exp {
        match infile {
            Ok(infile) => {
                let mut outfile = infile.clone();
                outfile.set_extension("out");
                if outfile.is_file() {
                    infiles.push(infile);
                    outfiles.push(outfile);
                }
            }
            Err(e) => log(
                format!("Error in the globbed file: {}. Skipping.", e).as_str(),
                LogLevel::Error,
            ),
        };
    }

    infiles.sort_unstable();
    outfiles.sort_unstable();

    let zipped = infiles.into_iter().zip(outfiles.into_iter()).collect();

    Ok(zipped)
}

pub fn test_all(test_path: &Path, script_path: &Path) -> Result<(), &'static str> {
    let test_files = get_test_files(test_path);
    if let Err(e) = test_files {
        soft_panic(format!("There was an error getting the test files: {}", e));
    }
    let test_files = test_files.unwrap();
    let mut test_counter = 1;
    for (infile, outfile) in test_files {
        let result = run_py_and_capture_output(script_path, infile.as_path());
        if let Err(e) = result {
            soft_panic(format!("There was an error running the script: {}", e));
        }
        let result = result.unwrap();
        let diff = diff_file_str(outfile.as_path(), &result);
        match diff {
            None => log(format!("Teste {}: passed", test_counter).as_str(), LogLevel::Success),
            Some((expected, your)) => {
                log(format!("Teste {}: failed", test_counter).as_str(), LogLevel::Error);
                log(">>> Your output:", LogLevel::Warn);
                log(format!("{}", your).as_str(), LogLevel::Info);
                log(">>> Expected output:", LogLevel::Warn);
                log(format!("{}", expected).as_str(), LogLevel::Info);
            }
        }
        test_counter += 1;
    }

    Ok(())
}

// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    fn get_tests_path() -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/files");
        path
    }

    #[test]
    fn test_diff_file_str() {
        let path = get_tests_path();
        let expected = "hello\nthere\n";
        let result = diff_file_str(&path.join("file1.txt"), &expected.to_string());
        match result {
            Some((file_contents, expected)) => {
                println!("File contents: {:?}", file_contents);
                println!("Expected: {:?}", expected);
                panic!("");
            }
            _ => {}
        }
    }

    #[test]
    fn test_run_py_and_capture_output() {
        let path = get_tests_path();
        let input = path.join("file1.txt");
        let hello_out = run_py_and_capture_output(&path.join("hello.py"), &input).unwrap();
        assert_eq!(hello_out, String::from("Hello, World!\n"));
    }

    #[test]
    fn test_diff_file_str_and_pyrun() {
        let path = get_tests_path();
        let input = path.join("testfiles/arq01.in");
        let output = path.join("testfiles/arq01.out");
        let hello_out = run_py_and_capture_output(&path.join("lab10.py"), &input).unwrap();
        let diff = diff_file_str(output.as_path(), &hello_out);
        assert!(diff.is_none());
    }

    #[test]
    fn test_get_test_files() {
        let path = get_tests_path();
        let testpath = path.join("testfiles");
        let files = get_test_files(testpath.as_path()).unwrap();
        assert_eq!(files.len(), 10);
        assert!(files[0].0.is_file() && files[0].1.is_file());
    }
}
