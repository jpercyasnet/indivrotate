extern crate exif;
extern crate chrono;
use iced::Color;
use std::path::Path;
use crate::get_dirlistr;
pub fn nextpressi (dir_value: String, fromtxt: String, totxt: String) -> (i32, Color, String, String, String) {
     let mut errcode: i32 = 0;
     let mut errstring: String = " ".to_string();
     let mut colorx : Color = Color::from([1.0, 0.0, 0.0]);
     let mut from_int1 = 0;
     let mut to_int1 = 0;
     let mut fromstr: String = " ".to_string();
     let mut tostr: String = " ".to_string();
     let mut badsize_int = 1;

     if fromtxt.len() == 0 {
         errstring = "********* List: From has no value **********".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
         errcode = 1;
     } else {
         let from_int: i32 = fromtxt.parse().unwrap_or(-99);
         if from_int > 0 {
             badsize_int = 0;
             from_int1 = from_int;
         } else if from_int == -99 {
             errstring = "********* List: From is not an integer **********".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
             errcode = 2;
         } else {
             errstring = "********* List: From not positive integer **********".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
             errcode = 3;
         }
         if badsize_int == 0 {
             badsize_int = 1;
             if totxt.len() == 0 {
                 errstring = "********* List: To has no value **********".to_string();
                 colorx = Color::from([1.0, 0.0, 0.0]);
                 errcode = 4;
             } else {
                 let to_int: i32 = totxt.parse().unwrap_or(-99);
                 if to_int > 0 {
                     badsize_int = 0;
                     to_int1 = to_int;
                 } else if to_int == -99 {
                     errstring = "********* List: To is not an integer **********".to_string();
                     colorx = Color::from([1.0, 0.0, 0.0]);
                     errcode = 5;
                 } else {
                     errstring = "********* List: To not positive integer **********".to_string();
                     colorx = Color::from([1.0, 0.0, 0.0]);
                     errcode = 6;
                 }
                 if badsize_int == 0 {
                     badsize_int = 1;
                     if to_int1 < from_int1 {
                         errstring = "********* List: From Greater than To **********".to_string();
                         colorx = Color::from([1.0, 0.0, 0.0]);
                         errcode = 7;
                     } else {
                         badsize_int = 0;
                     }
                 }
             }
         }
     }
     if badsize_int == 0 {
         if !Path::new(&dir_value).exists() {
             errstring = "the directory does not exist".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
             errcode = 8;
         } else { 
             let oldfrom_int1 = from_int1;
             from_int1 = to_int1 + 1;
             let dir_path = Path::new(&dir_value);
             let (errcd, errstr, liststr) = get_dirlistr(dir_path.to_path_buf());
             if errcd == 0 {
                 if liststr.len() < from_int1 as usize {
                     errstring =  format!("********* List: From {} Greater than number of files of {} **********", from_int1, liststr.len());
                     colorx = Color::from([1.0, 0.0, 0.0]);
                     errcode = 9;
                 } else {
                     to_int1 = to_int1 + to_int1 - oldfrom_int1 + 1;
                     fromstr = format!("{}", from_int1);
                     tostr = format!("{}", to_int1);
                 }
             } else {
                 errstring = errstr.to_string();
                 colorx = Color::from([1.0, 0.0, 0.0]);
                 errcode = 10;
             }
         }
     }
     (errcode, colorx, errstring, fromstr, tostr)
}

