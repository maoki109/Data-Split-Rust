# Data-Split-Rust

## Description
Tool to create a training and validation set out of a single training set. Takes a directory of training instances and creates a new directory where the training set is split into a smaller training and validation set. 

## Required File Structure
This program is intended for image datasets. Your original training set directory should have two subdirectories titled "images" and "labels".
```
train
 ├──images
 └──labels
```
The images and corresponding label files should have the same name, with the only difference being the file extension. For example, an image in `/images/001.png` should have a corresponding file with it's labels in `/labels/001.txt`. 

## Install Instructions
Learn about installing Rust here: https://www.rust-lang.org/learn/get-started 

## Build Instructions
```
cargo build
```

## Run Instructions
Parameters:
- How much of original training set you want in new training set as a decimial between 0 and .99
  - e.g. `.8`
- Filepath to original training set
  - e.g. `/home/user1/Documents/COCO128_example/original_train`
  - **IMPORTANT NOTE:** Filepath must NOT end with a `/`, or else it will not work properly. 

```
cargo run -- <ratio> <filepath>
```

Example: Doing a 80/20 train/val split on COCO128
```
cargo run -- .8 /home/user1/Documents/COCO128_example/original_train
```
