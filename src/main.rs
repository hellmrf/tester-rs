use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use colored::Colorize;

#[allow(dead_code)]
enum LogLevel {
    Debug,
    Success,
    Info,
    Warn,
    Error,
}

fn soft_panic(msg: String) -> ! {
    log(&msg, LogLevel::Error);
    std::process::exit(1);
}

fn log(msg: &str, level: LogLevel) {
    match level {
        LogLevel::Debug => println!("{}", msg.blue()),
        LogLevel::Warn => println!("{}", msg.yellow()),
        LogLevel::Error => println!("{}", msg.red()),
        LogLevel::Success => println!("{}", msg.green()),
        _ => println!("{}", msg)
    };
}

// TODO: check how to get the output of the Python program
// TODO: implement the get_test_files_from_dir function

/// Check if the file contents is the same as the expected String.
/// Returns (file_contents, expected) if there is a difference. Returns None otherwise.
fn diff_file_str(filename: &Path, expected: &String) -> Option<(String, String)> {
    let file_contents = fs::read_to_string(filename).expect("Something went wrong reading the file. Check if you have all the permissions and that the file exists.");
    if file_contents == *expected {
        return None;
    }
    return Some((file_contents, expected.clone()));
}

/// Executes the Python program and returns the output.
fn run_py_and_capture_output(filename: &Path, input: &Path) -> io::Result<String> {
    let input_contents = fs::read_to_string(input)?;

    let mut process = Command::new("python3")
        .args(&[filename])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    process.stdin.as_mut().unwrap().write_all(input_contents.as_bytes())?;

    let output = process.wait_with_output()?;
    let result = String::from_utf8_lossy(output.stdout.as_slice()).to_string();
    Ok(result)
}

fn main() {
    // let the_path = Path::new(TESTFILE);
    // let str1 = String::from("hello :)\nthis is another line");
    // let str2 = String::from("hello :)\nthis is a different line");
    // let diff1 = diff_file_str(&the_path, &str1);
    // let diff2 = diff_file_str(&the_path, &str2);
    // println!("1: {:?}", diff1);
    // println!("2: {:?}", diff2);
    // println!("{:?}", the_path);

    let program = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/main.py");
    let program_out = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/tests/arq01.out");
    let input = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/tests/arq01.in");

    let result = run_py_and_capture_output(program, input);
    if let Err(e) = &result { 
        soft_panic(format!("Houve um erro ao tentar executar o programa: {}", e));
    }
    let diff = diff_file_str(&program_out, &result.unwrap());
    match diff {
        Some((file_contents, expected)) => {
            log("O resultado do programa é diferente do esperado.", LogLevel::Warn);
            log(&format!(">>> O resultado do programa é:\n{}", file_contents), LogLevel::Info);
            log(&format!(">>> O resultado esperado é:\n{}", expected), LogLevel::Info);
        },
        None => log("O resultado do programa é o mesmo do esperado.", LogLevel::Success)
    }
}
