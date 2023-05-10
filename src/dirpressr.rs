extern crate chrono;
use iced::Color;
use rfd::FileDialog;
use crate::get_dirlistr;
pub fn dirpressr () -> (Color, String, String, Vec<String>) {
     let errstring: String;
     let new_dirlist: Vec<String> = Vec::new();
     let mut new_dir: String = " ".to_string();
     let colorx : Color;
     let folder = FileDialog::new()
                    .pick_folder();
     if folder == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let current_dir = folder;
         let (errcd, errstr, _newlist) = get_dirlistr(current_dir.unwrap());
         if errcd == 0 {
//             new_dirlist = newlist;
             errstring = "got directory".to_string();
             colorx = Color::from([0.0, 0.0, 0.0]);
         } else {
             errstring = errstr.to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
         }
     } 
    (colorx, errstring, new_dir, new_dirlist)
}

