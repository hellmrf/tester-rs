use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

const TESTFILE: &str = "/home/helitonmrf/Projects/tester-rs/aaa.txt";

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

fn main() {
    let the_path = Path::new(TESTFILE);
    let str1 = String::from("hello :)\nthis is another line");
    let str2 = String::from("hello :)\nthis is a different line");

    let diff1 = diff_file_str(&the_path, &str1);
    let diff2 = diff_file_str(&the_path, &str2);
    println!("1: {:?}", diff1);
    println!("2: {:?}", diff2);

    println!("{:?}", the_path);
}
