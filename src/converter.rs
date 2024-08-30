use std::process::Command;
use std::fs;

pub fn convert(file_path: String) -> Vec<u8> {
    let args_arr = ["-i", &file_path, "-c:v", "libx264", "-b:v", "2M", "-c:a", "aac", "BigBuckBunny.mov"];

    let output = Command::new("ffmpeg").args(args_arr).output().expect("failed to run ffmpeg command");
    // let output = Command::new("ffmpeg").output().expect("failed");
    output.stdout
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;
    
    #[test]
    fn it_should_convert_to_a_mov() {
        let input_file_path = String::from("big_buck_bunny_720p_2mb.mp4");
        let expected_file = String::from("BigBuckBunny.mov");

        convert(input_file_path);
        
        let converted_file_exists = Path::new(&expected_file).exists();
        assert!(converted_file_exists);

        fs::remove_file(expected_file).expect("Error when cleaning up after test");
    }
}
