use std::process::Command;

fn main() {
    let result = transcode(String::from("big_buck_bunny_720p_2mb.mp4"));
    println!("{result:?}");
}

fn transcode(file_path: String) -> Vec<u8> {
    let args_arr = ["-i", &file_path, "-c:v", "libx264", "-b:v", "2M", "-c:a", "aac", "BigBuckBunny.mov"];

    let output = Command::new("ffmpeg").args(args_arr).output().expect("failed to run ffmpeg command");
    // let output = Command::new("ffmpeg").output().expect("failed");
    output.stdout
}
