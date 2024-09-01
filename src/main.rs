use converter::extract_film_name;
use std::env;

mod converter;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    println!("Checking {file_path}");

    let output_name = extract_film_name(&file_path);

    let output_file: String = [output_name, String::from(".mov")].join("");
    converter::convert(&file_path, &output_file);

    println!("Complete!");
}
