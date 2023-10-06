use std::{env,fs};
use std::path::Path;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{stdin,stdout,Write};

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
    // println!("Vec 1: {:?}", vec);

    let ratio: f32 = training_ratio.parse().unwrap();
    let split = ((len as f32) * ratio) as usize;
    // println!("{}", split);

    let split_ind: Vec<&[usize]> = vec.chunks(split).collect();
    // println!("Split indices: {:?}", split_ind);
    // println!("image indices: {:?}", split_ind[0]);
    // println!("label indices: {:?}", split_ind[1]);

    // New train and validation directories
    new_directory("train", file_path, image_vector.clone(), label_vector.clone(), split_ind.clone());
    new_directory("val", file_path, image_vector.clone(), label_vector.clone(), split_ind.clone());

}

fn file_vector(root_path: &str, file_type: &str) -> Vec<String> {
    let this_path = fs::read_dir(root_path.to_string()+"/"+file_type);
    let mut this_vector: Vec<String> = Vec::new();

    if let Ok(files) = this_path {
        for file in files {
            if let Ok(file) = file {
                let this_file = file.path().file_name().unwrap().to_str().unwrap().to_string();
                this_vector.push(this_file);
            }
        }
    }

    this_vector.sort();
    // println!("This Vector: {:?}", this_vector);
    this_vector
}

fn new_directory(split_type: &str, file_path: &str, img_vec: Vec<String>, lbl_vec: Vec<String>, split_vec: Vec<&[usize]>) {
    let file_types = ["images", "labels"];
    let mut i = 0;
    let mut this_vector: Vec<String> = Vec::new();
    match split_type == "train" {
        true => i = 0,
        false => i = 1,
    }
    for file_type in file_types {
        match file_type == "images" {
            true => this_vector = img_vec.clone(),
            false => this_vector = lbl_vec.clone(),
        }
        let new_path = file_path.strip_suffix("train").unwrap().to_string()+"new_"+&split_type+"/"+&file_type+"/";
        match Path::new(&new_path).try_exists() {
            Ok(false) => {
                fs::create_dir_all(new_path.clone());
                println!("New directory created at {:?}", new_path);
                for n in split_vec[i] {
                    let new_name = "/".to_owned()+file_type+"/"+&this_vector[*n];
                    let old_path = file_path.to_string()+&new_name;
                    // println!("Old image path: {:?}", old_path);
                    // println!("New image path: {:?}", new_path.clone()+&this_vector[*n]);
                    fs::copy(old_path, new_path.clone()+&this_vector[*n]);
                }
            },
            Ok(true) => {
                println!("Directory already exists.");
                let mut s = String::new();
                println!("By continuing, the following directory will be overwritten: {:?}", new_path);
                print!("Do you wish to continue? [y/n]: ");
                let _ = stdout().flush();
                stdin().read_line(&mut s).expect("Did not enter correct string");
                if let Some('\n')=s.chars().next_back() {
                    s.pop();
                }
                if let Some('\r')=s.chars().next_back() {
                    s.pop();
                }
                if s.contains("y") {
                    fs::remove_dir_all(new_path.clone());
                    fs::create_dir_all(new_path.clone());
                    for n in split_vec[i] {
                        let new_name = "/".to_owned()+file_type+"/"+&this_vector[*n];
                        let old_path = file_path.to_string()+&new_name;
                        // println!("Old image path: {:?}", old_path);
                        // println!("New image path: {:?}", new_path.clone()+&this_vector[*n]);
                        fs::copy(old_path, new_path.clone()+&this_vector[*n]);
                    }
                    println!("Directory rewritten: {:?}", new_path);
                } else {
                    println!("Operation canceled.");
                }
            },
            Err(_) => println!("Uh oh! Error!"),
        }
    }
}
