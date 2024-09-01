use std::process::Command;
use std::fs;

use regex::Regex;

pub fn convert(file_path: &String, output_file: &String) -> Vec<u8> {
    let args_arr = ["-i", &file_path, "-c:v", "libx264", "-b:v", "2M", "-c:a", "aac", output_file];

    let output = Command::new("ffmpeg").args(args_arr).output().expect("failed to run ffmpeg command");
    output.stdout
}

pub fn extract_film_name(file_name: &String) -> String {
    let re = Regex::new(r"(.*)_t\d+\..*").unwrap();

    let Some(output) = re.captures(&file_name) else {
        println!("No match");
        return String::from("");
    };

    return (&output[1]).to_string()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use test_case::test_case;
    use super::*;
    
    #[test]
    fn it_should_convert_to_a_mov() {
        let input_file_path = String::from("big_buck_bunny_720p_2mb.mp4");
        let expected_file = String::from("BigBuckBunny.mov");

        convert(&input_file_path, &expected_file);
        
        let converted_file_exists = Path::new(&expected_file).exists();
        assert!(converted_file_exists);

        fs::remove_file(expected_file).expect("Error when cleaning up after test");
    }

    #[test_case("my_film_t01.mkv", "my_film")]
    #[test_case("this_is_another_t09.mkv", "this_is_another")]
    fn should_extract_the_film_name(input: &str, expected: &str) {
        let input = String::from(input);

        let actual = extract_film_name(&input);
        let expected = String::from(expected);

        assert_eq!(actual, expected);
    }
}
