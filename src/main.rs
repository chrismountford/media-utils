use std::env;

use media_utils::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    run(file_path, &String::from("\\\\chrispi\\PiShare"));
}
