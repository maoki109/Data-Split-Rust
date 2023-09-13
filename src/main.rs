use std::{env,fs};
use std::path::Path;

struct instance {
    image: String,
    label: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let training_ratio = &args[1];
    let file_path = &args[2];

    println!("{} is how much training we want", training_ratio);
    println!("From the directory {}", file_path);

    let image_path = fs::read_dir(file_path.to_string()+"/images").unwrap();
    let label_path = fs::read_dir(file_path.to_string()+"/labels").unwrap();

    for file in image_path {
        println!("{}", file.unwrap().file_name().to_str().unwrap());
    }

}
