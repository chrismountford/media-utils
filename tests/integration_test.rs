use std::{fs, path::Path};

use media_utils::{self, run};

#[test]
fn it_should_convert_copy_and_clean_up_files() {
    let dest_path = String::from("./tmpDest");

    let _ = fs::create_dir("./tmpSource");
    let _ = fs::create_dir("./tmpDest");
    let _ = fs::copy("./big_buck_bunny_t08.mp4", "./tmpSource/big_buck_bunny_t08.mp4");

    let test_file = &String::from("./tmpSource/big_buck_bunny_t08.mp4");

    run(test_file, &dest_path);

    assert!(Path::new("./tmpDest/big_buck_bunny.mov").exists());

    let _ = fs::remove_dir_all("./tmpSource");
    let _ = fs::remove_dir_all("./tmpDest");
}
