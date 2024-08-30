mod converter;

fn main() {
    let result = converter::convert(String::from("big_buck_bunny_720p_2mb.mp4"));
    println!("{result:?}");
}
