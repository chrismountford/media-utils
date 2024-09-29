use std::{fs, process::Command};
use regex::Regex;

pub fn convert(file_path: &String, output_file_name: &String) -> Vec<u8> {
    let args_arr = ["-i", &file_path, "-c:v", "libx264", "-b:v", "2M", "-c:a", "aac", &output_file_name];

    let output = Command::new("ffmpeg").args(args_arr).output().expect("failed to run ffmpeg command");
    output.stdout
}

pub fn extract_film_name(file_name: &String) -> String {
    let re = Regex::new(r"(?:.*\\)*(.*)_t\d+\..*").unwrap();

    let Some(output) = re.captures(&file_name) else {
        println!("No match");
        return String::from("");
    };

    return (&output[1]).to_string()
}

pub fn run(file_path: &String, dest: &String) {
    println!("Checking {file_path}");

    let output_file: String = get_output_name(file_path);
    convert(&file_path, &output_file);
    println!("Converting complete");

    println!("Removing {file_path}...");
    let _ = fs::remove_file(file_path);

    let output_file_name: String = get_file_name_from_path(file_path);
    let out = format!("{dest}/{output_file_name}.mov");
    println!("Copying {output_file} to {out}");
    let _ = fs::copy(&output_file, out);

    println!("Removing file {output_file}");
    let _ = fs::remove_file(output_file);
}

fn get_output_name(file_path: &String) -> String {
    let output_name = extract_film_name(&file_path);
    return ([output_name, String::from(".mov")].join("")).to_string()
}

fn get_file_name_from_path(file_path: &String) -> String {
    let path_arr = file_path.split("/");
    extract_film_name(&path_arr.last().unwrap().to_string())
    
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use test_case::test_case;
    use super::*;
    
    #[test]
    fn it_should_convert_to_a_mov() {
        let input_file_path = String::from("big_buck_bunny_t08.mp4");
        let expected_file = String::from("big_buck_bunny.mov");

        convert(&input_file_path, &expected_file);
        
        let converted_file_exists = Path::new(&expected_file).exists();
        assert!(converted_file_exists);

        std::fs::remove_file(expected_file).expect("Error when cleaning up after test");
    }

    #[test_case("my_film_t01.mkv", "my_film")]
    #[test_case("this_is_another_t09.mkv", "this_is_another")]
    #[test_case("big_buck_bunny_t09.mp4", "big_buck_bunny")]
    #[test_case(r".\Saving Private Ryan_t00.mkv", "Saving Private Ryan")]
    fn should_extract_the_film_name(input: &str, expected: &str) {
        let input = String::from(input);

        let actual = extract_film_name(&input);
        let expected = String::from(expected);

        assert_eq!(actual, expected);
    }
}
