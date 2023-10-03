use std::{env,fs};
use std::path::Path;
use rand::thread_rng;
use rand::seq::SliceRandom;

// struct instance {
//     image: String,
//     label: String,
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let training_ratio = &args[1];
    let file_path = &args[2];

    println!("{} is how much we want in the training set, rest in validation set. ", training_ratio);
    println!("From the directory {}", file_path);

    let image_path = fs::read_dir(file_path.to_string()+"/images");
    let label_path = fs::read_dir(file_path.to_string()+"/labels");

    let mut image_vector: Vec<String> = Vec::new();
    let mut label_vector: Vec<String> = Vec::new();

    if let Ok(files) = image_path {
        for file in files {
            if let Ok(file) = file {
                let image_file = file.path().file_name().unwrap().to_str().unwrap().to_string();
                // let file_name = file.path().file_stem().unwrap().to_str().unwrap().to_string();
                image_vector.push(image_file);
                // println!("{}", image_file);
                // println!("{}", file_name);
            }
        }
    }
    image_vector.sort();
    println!("Image Vector: {:?}", image_vector);

    if let Ok(files) = label_path {
        for file in files {
            if let Ok(file) = file {
                let label_file = file.path().file_name().unwrap().to_str().unwrap().to_string();
                // let file_name = file.path().file_stem().unwrap().to_str().unwrap().to_string();
                label_vector.push(label_file);
            }
        }
    }
    label_vector.sort();
    println!("Label Vector: {:?}", label_vector);

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

    // make into a function
    // for new training directory
    let new_train_image_path = file_path.strip_suffix("train").unwrap().to_string()+"new_train/images";
    let new_train_label_path = file_path.strip_suffix("train").unwrap().to_string()+"new_train/labels";

    println!("New images path: {:?}", new_train_image_path);
    println!("New labels path: {:?}", new_train_label_path);

    match Path::new(&new_train_image_path).try_exists() {
        Ok(false) => {
            fs::create_dir_all(new_train_image_path.clone());
            println!("New directory for images created at {:?}", new_train_image_path);
        },
        Ok(true) => println!("Directory already exists"),
        Err(_) => println!("Error!"),
    }

    match Path::new(&new_train_label_path).try_exists() {
        Ok(false) => {
            fs::create_dir_all(new_train_label_path.clone());
            println!("New directory for labels created at {:?}", new_train_label_path);
        },
        Ok(true) => println!("Directory already exists"),
        Err(_) => println!("Error!"),
    }

    // for new validation directory
    let new_val_image_path = file_path.strip_suffix("train").unwrap().to_string()+"new_val/images";
    let new_val_label_path = file_path.strip_suffix("train").unwrap().to_string()+"new_val/labels";

    println!("New val images path: {:?}", new_val_image_path);
    println!("New val labels path: {:?}", new_val_label_path);

    match Path::new(&new_val_image_path).try_exists() {
        Ok(false) => {
            fs::create_dir_all(new_val_image_path.clone());
            println!("New directory for val images created at {:?}", new_val_image_path);
        },
        Ok(true) => println!("Directory already exists"),
        Err(_) => println!("Error!"),
    }

    match Path::new(&new_val_label_path).try_exists() {
        Ok(false) => {
            fs::create_dir_all(new_val_label_path.clone());
            println!("New directory for val labels created at {:?}", new_val_label_path);
        },
        Ok(true) => println!("Directory already exists"),
        Err(_) => println!("Error!"),
    }

}
