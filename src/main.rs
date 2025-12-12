mod d1;
mod d10;
mod d11;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

#[macro_export]
macro_rules! solution {
    ($feature_name:literal: $solution:expr, $input_file:literal) => {
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
    ($solution:expr, $input_file:literal => $expected:expr) => {
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
    solution!("4a": d4::a, "4.txt");
    solution!("4b": d4::b, "4.txt");
    solution!("5a": d5::a, "5.txt");
    solution!("5b": d5::b, "5.txt");
    solution!("6a": d6::a, "6.txt");
    solution!("6b": d6::b, "6.txt");
    solution!("7a": d7::a, "7.txt");
    solution!("7b": d7::b, "7.txt");
    solution!("8a": d8::a::<1000>, "8.txt");
    solution!("8b": d8::b, "8.txt");
    solution!("9a": d9::a, "9.txt");
    solution!("9b": d9::b, "9.txt");
    solution!("10a": d10::a, "10.txt");
    #[cfg(feature = "10b")]
    solution!("10b": d10::b, "10.txt");
    solution!("11a": d11::a, "11.txt");
    solution!("11b": d11::b, "11.txt");
}
