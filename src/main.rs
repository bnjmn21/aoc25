mod d1;
mod d2;
mod d3;

#[macro_export]
macro_rules! solution {
    ($feature_name:literal: $solution:path, $input_file:literal) => {
        if cfg!(feature = $feature_name) {
            use std::fs;

            let file = fs::read_to_string(format!("inputs/{}", $input_file)).unwrap();
            let result = $solution(&file);
            println!("{} solution: {result}", $feature_name);
        }
    };
}

#[macro_export]
macro_rules! test_solution {
    ($solution:path, $input_file:literal => $expected:expr) => {
        use std::fs;

        let file = fs::read_to_string(format!("inputs/{}", $input_file)).unwrap();
        let result = $solution(&file);
        assert_eq!(result, $expected);
    };
}

fn main() {
    solution!("1a": d1::a, "1.txt");
    solution!("1b": d1::b, "1.txt");
    solution!("2a": d2::a, "2.txt");
    solution!("2b": d2::b, "2.txt");
    solution!("3a": d3::a, "3.txt");
    solution!("3b": d3::b, "3.txt");
}
