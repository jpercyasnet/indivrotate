// extern crate exif;
use std::fs;

use std::path::{PathBuf};

//  Use to get list of sorted files in the directory list in vector format
// input is the directory and output is error number, error string and model
pub fn get_dirlistr (current_dir: PathBuf) -> (u32, String, Vec<String>) {
    let errcode: u32;
    let errstring: String;
    let mut listitems: Vec<String> = Vec::new();
    let mut numentry = 0;
    for entry1 in fs::read_dir(&current_dir).unwrap() {
         let entry = entry1.unwrap();
         if let Ok(metadata) = entry.metadata() {
             if let Ok(file_name) = entry.file_name().into_string() {
                 if metadata.is_file() {
                     if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                         file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") |
                         file_name.ends_with(".png") |file_name.ends_with(".PNG") { 
                            listitems.push(file_name);
                            numentry = numentry + 1;
                     }
                 }
             }
         }
    }
    if numentry > 0 {
        listitems.sort();
        errstring = format!("{} files in directory ", numentry);
        errcode = 0;
    } else {
        errstring = "********* Directory 1: directory has no image files (jpg or png) **********".to_string();
        errcode = 1;
    }
    (errcode, errstring, listitems)
}

