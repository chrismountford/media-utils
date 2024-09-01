use converter::extract_film_name;

mod converter;

fn main() {
    let file = String::from("big_buck_bunny_720p_2mb.mp4");
    let output_name = extract_film_name(&file);

    converter::convert(&file, &output_name);
    
    println!("Complete!");
}
