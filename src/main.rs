use std::path::Path;

mod logger;
mod tester;
use tester::{get_test_files};

fn main() {
    let tests_path = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/tests");
    let _test_files = get_test_files(&tests_path).expect("Failed to get test files.");
}

// fn main_diff() {
//     let the_path = Path::new(TESTFILE);
//     let str1 = String::from("hello :)\nthis is another line");
//     let str2 = String::from("hello :)\nthis is a different line");
//     let diff1 = diff_file_str(&the_path, &str1);
//     let diff2 = diff_file_str(&the_path, &str2);
//     println!("1: {:?}", diff1);
//     println!("2: {:?}", diff2);
//     println!("{:?}", the_path);
// }
// fn main_runpy() {
//     let program = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/main.py");
//     let program_out = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/tests/arq01.out");
//     let input = Path::new("/home/helitonmrf/Documents/TEMP/tally/pde/lab11/tests/arq01.in");

//     let result = run_py_and_capture_output(program, input);
//     if let Err(e) = &result {
//         soft_panic(format!(
//             "Houve um erro ao tentar executar o programa: {}",
//             e
//         ));
//     }
//     let diff = diff_file_str(&program_out, &result.unwrap());
//     match diff {
//         Some((file_contents, expected)) => {
//             log(
//                 "O resultado do programa é diferente do esperado.",
//                 LogLevel::Warn,
//             );
//             log(
//                 &format!(">>> O resultado do programa é:\n{}", file_contents),
//                 LogLevel::Info,
//             );
//             log(
//                 &format!(">>> O resultado esperado é:\n{}", expected),
//                 LogLevel::Info,
//             );
//         }
//         None => log(
//             "O resultado do programa é o mesmo do esperado.",
//             LogLevel::Success,
//         ),
//     }
// }
