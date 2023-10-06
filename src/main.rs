use std::{env,fs};
use std::path::Path;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    let training_ratio = &args[1];
    let file_path = &args[2];

    println!("{} is how much we want in the training set, rest in validation set. ", training_ratio);
    println!("From the directory {}", file_path);

    // Put image and label file names into vectors
    let image_vector = file_vector(file_path, "images");
    let label_vector = file_vector(file_path, "labels");

    // Split image and label vectors based on ratio needed for training set
    let len = image_vector.len();
    let mut vec: Vec<usize> = (0..len).collect();
    vec.shuffle(&mut thread_rng());
    println!("Vec 1: {:?}", vec);

    let ratio: f32 = training_ratio.parse().unwrap();
    let split = ((len as f32) * ratio) as usize;
    println!("{}", split);

    let split_ind: Vec<&[usize]> = vec.chunks(split).collect();
    println!("Split indices: {:?}", split_ind);
    println!("image indices: {:?}", split_ind[0]);
    println!("label indices: {:?}", split_ind[1]);

    // Creating new train and validation directory
    new_directory("train", file_path);
    new_directory("val", file_path);

    // Copy training image and label files into new directories
    // for n in split_ind[0] {
    //     // println!("{:?}", image_vector[*n]);
    //     // let img_name = format!("/images/{:?}", image_vector[*n]);
    //     // let label_name = format!("/labels/{:?}", label_vector[*n]);
    //     let img_name = "/images/".to_owned()+&image_vector[*n];
    //     let label_name = "/labels/".to_owned()+&label_vector[*n];
    //     let old_image_path = file_path.to_string()+&img_name;
    //     let old_label_path = file_path.to_string()+&label_name;
    //     // let testname = new_train_image_path.clone()+&image_vector[*n];
    //     println!("Old image path: {:?}", old_image_path);
    //     println!("New image path: {:?}", new_train_image_path.clone()+&image_vector[*n]);
    //     fs::copy(old_image_path, new_train_image_path.clone()+&image_vector[*n]);
    //     fs::copy(old_label_path, new_train_label_path.clone()+&label_vector[*n]);
    // }

    // Copy validation image and label filesinto new directories
}

fn file_vector(root_path: &str, file_type: &str) -> Vec<String> {
    let this_path = fs::read_dir(root_path.to_string()+"/"+file_type);
    let mut this_vector: Vec<String> = Vec::new();

    if let Ok(files) = this_path {
        for file in files {
            if let Ok(file) = file {
                let this_file = file.path().file_name().unwrap().to_str().unwrap().to_string();
                // let file_name = file.path().file_stem().unwrap().to_str().unwrap().to_string();
                this_vector.push(this_file);
                // println!("{}", image_file);
                // println!("{}", file_name);
            }
        }
    }
    this_vector.sort();
    println!("This Vector: {:?}", this_vector);
    this_vector
}

fn new_directory(split_type: &str, file_path: &str) {
    // println!("Split type: {:?}", split_type);
    // println!("File type: {:?}", file_type);
    let file_types = ["images", "labels"];

    for file_type in file_types {
        let new_path = file_path.strip_suffix("train").unwrap().to_string()+"new_"+&split_type+"/"+&file_type+"/";
        match Path::new(&new_path).try_exists() {
            Ok(false) => {
                fs::create_dir_all(new_path.clone());
                println!("New directory created at {:?}", new_path);
            },
            Ok(true) => println!("Directory already exists"),
            Err(_) => println!("Error!"),
        }
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
