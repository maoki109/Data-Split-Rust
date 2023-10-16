use std::{env,fs};
use std::path::Path;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io::{stdin,stdout,Write};

// file_vector represents the files in a directory as a vector of strings.
// Each item in the returned vector is the name of a file from the directory as a string.
// The vector is also sorted using Rust's sort() function.
//
// Inputs:
//     - root_path: path to the training folder (inputted by user in the command line argument).
//     - file_type: name of subfolder to be put into a vector (either "images" or "labels").
// Returns:
//     a sorted vector of filenames as strings.
fn file_vector(root_path: &str, file_type: &str) -> Vec<String> {
    let this_path = fs::read_dir(root_path.to_string()+"/"+file_type);
    let mut this_vector: Vec<String> = Vec::new();
    // for every file in directory, get file name as string and add to vector.
    if let Ok(files) = this_path {
        for file in files.flatten() {
            let this_file = file.path().file_name().unwrap().to_str().unwrap().to_string();
            this_vector.push(this_file);
        }
    }
    // sort the vector
    this_vector.sort();
    this_vector
}

// new_directory creates a new directory (with images and labels subdirectories) and copies
// a portion of the original training set (specificied by user input) into this directory.
// If the images and labels directory already exist, the user is asked whether they want them
// overwritten with a new set of images and labels.
//
// Inputs:
//     - split_type: either "train" or "val" for the type of directory user wants to create.
//     - file_path: path to the training folder (inputted by user in the command line argument).
//     - img_vec: sorted vector of image file names as strings.
//     - lbl_vec: sorted vector of label file names as strings.
//     - split_vec: vector of two vectors containing randomized indices of appropriate size for train/val split.
// Returns:
//     New directory with 'images' and 'labels' subdirectories containing copies of original dataset
//     in the portion specificied by user in command line input.
fn new_directory(split_type: &str, file_path: &str, img_vec: Vec<String>, lbl_vec: Vec<String>, split_vec: Vec<&[usize]>) {
    let file_types = ["images", "labels"];
    let mut this_vector: Vec<String>;
    // Set flag for whether this is a train split or a val split.
    let i = match split_type == "train" {
        true => 0,
        false => 1,
    };
    // Get folder name from given directory for string manipulation.
    let split_path = file_path.split('/');
    let folder_name = split_path.last().unwrap();
    // For both images and labels subdirectories.
    for file_type in file_types {
        match file_type == "images" {
            true => this_vector = img_vec.clone(),
            false => this_vector = lbl_vec.clone(),
        }
        let new_path = file_path.strip_suffix(folder_name).unwrap().to_string()+"new_"+split_type+"/"+file_type+"/";
        match Path::new(&new_path).try_exists() {
            // If directory doesn't exist, create new directory and copy over appropriate files
            Ok(false) => {
                fs::create_dir_all(new_path.clone()).ok();
                println!("New directory created at {new_path:?}");
                for n in split_vec[i] {
                    let new_name = "/".to_owned()+file_type+"/"+&this_vector[*n];
                    let old_path = file_path.to_string()+&new_name;
                    fs::copy(old_path, new_path.clone()+&this_vector[*n]).ok();
                }
            },
            // If directory already exists, ask user if they want to overwrite files in directory.
            // If yes, delete and recreate directory. If no, cancel operation.
            Ok(true) => {
                println!("Directory already exists.");
                let mut s = String::new();
                println!("By continuing, the following directory will be overwritten: {new_path:?}");
                print!("Do you wish to continue? [y/n]: ");
                let _ = stdout().flush();
                stdin().read_line(&mut s).expect("Did not enter correct string");
                if let Some('\n')=s.chars().next_back() {
                    s.pop();
                }
                if let Some('\r')=s.chars().next_back() {
                    s.pop();
                }
                if s.contains('y') {
                    fs::remove_dir_all(new_path.clone()).ok();
                    fs::create_dir_all(new_path.clone()).ok();
                    for n in split_vec[i] {
                        let new_name = "/".to_owned()+file_type+"/"+&this_vector[*n];
                        let old_path = file_path.to_string()+&new_name;
                        fs::copy(old_path, new_path.clone()+&this_vector[*n]).ok();
                    }
                    println!("Directory rewritten: {new_path:?}");
                } else {
                    println!("Operation canceled.");
                }
            },
            Err(_) => println!("Uh oh! Error!"),
        }
    }
}

// main reads in user input regarding the ratio of original training set to be put into the new
// training set and the filepath of the original training set. It will gather and sort the names
// of the image and label files, and split the indices of the instances to be put in the new
// training and validation sets. It will then create new directories for the training and validation
// set (both with respective images and labels subdirectories) and copy over the appropriate files.
fn main() {
    // Read in command line arguments.
    let args: Vec<String> = env::args().collect();
    let training_ratio = &args[1];
    let file_path = &args[2];

    // Put image and label file names into vectors
    let image_vector = file_vector(file_path, "images");
    let label_vector = file_vector(file_path, "labels");

    // Split randomized indices with ratio needed for training set.
    // split_ind will have two vectors. One with sindices to be put in training set, second
    // with indices to be put in validation set.
    let len = image_vector.len();
    let mut vec: Vec<usize> = (0..len).collect();
    vec.shuffle(&mut thread_rng());
    let ratio: f32 = training_ratio.parse().unwrap();
    let split = ((len as f32) * ratio) as usize;
    let split_ind: Vec<&[usize]> = vec.chunks(split).collect();

    // Helpful notes to see how many instances will be put into the new training set.
    println!("{split} instances will be in the training set, rest in validation set. ");
    println!("From the directory {file_path}");

    // New train and validation directories
    new_directory("train", file_path, image_vector.clone(), label_vector.clone(), split_ind.clone());
    new_directory("val", file_path, image_vector, label_vector, split_ind.clone());

}
