use rfd::FileDialog;
use crate::get_dirlistr;
pub fn dirpressr () -> (u32, String, String, Vec<String>) {
     let errcode: u32;
     let errstring: String;
     let new_dirlist: Vec<String> = Vec::new();
     let mut new_dir: String = " ".to_string();
     let folder = FileDialog::new()
//        .show_open_single_dir()
//        .unwrap();
          .pick_folder();
     if folder == None {
         errstring = "error getting directory -- possible cancel key hit".to_string();
         errcode = 1;
     } else {
         new_dir = folder.as_ref().expect("REASON").display().to_string();
         let current_dir = folder;
         let (errcd, errstr, _newlist) = get_dirlistr(current_dir.unwrap());
         if errcd == 0 {
             errstring = "got directory".to_string();
             errcode = 0;
         } else {
             errstring = errstr.to_string();
             errcode = 2;
         }
     } 
    (errcode, errstring, new_dir, new_dirlist)
}

