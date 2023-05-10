extern crate exif;
extern crate chrono;
use iced::Color;
use std::path::Path;
use crate::get_dirlistr;
pub fn listpressi (dir_value: String, fromtxt: String, totxt: String, sizetxt: String) -> (Color, String, Vec<String>, i32, i32, i32, u32) {
     let mut errstring: String = " ".to_string();
     let mut colorx : Color = Color::from([1.0, 0.0, 0.0]);
     let mut from_int1 = 0;
     let mut to_int1 = 0;
     let mut shortto_int1 = 0;
     let mut icon_int1 = 0;
     let mut badsize_int = 1;
     let mut newliststr: Vec<String> = Vec::new();

     if fromtxt.len() == 0 {
         errstring = "********* List: From has no value **********".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
     } else {
         let from_int: i32 = fromtxt.parse().unwrap_or(-99);
         if from_int > 0 {
             badsize_int = 0;
             from_int1 = from_int;
         } else if from_int == -99 {
             errstring = "********* List: From is not an integer **********".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
         } else {
             errstring = "********* List: From not positive integer **********".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
         }
         if badsize_int == 0 {
             badsize_int = 1;
             if totxt.len() == 0 {
                 errstring = "********* List: To has no value **********".to_string();
                 colorx = Color::from([1.0, 0.0, 0.0]);
             } else {
                 let to_int: i32 = totxt.parse().unwrap_or(-99);
                 if to_int > 0 {
                     badsize_int = 0;
                     to_int1 = to_int;
                 } else if to_int == -99 {
                     errstring = "********* List: To is not an integer **********".to_string();
                     colorx = Color::from([1.0, 0.0, 0.0]);
                 } else {
                     errstring = "********* List: To not positive integer **********".to_string();
                     colorx = Color::from([1.0, 0.0, 0.0]);
                 }
                 if badsize_int == 0 {
                     badsize_int = 1;
                     if to_int1 < from_int1 {
                         errstring = "********* List: From Greater than To **********".to_string();
                         colorx = Color::from([1.0, 0.0, 0.0]);
                     } else {
                         if sizetxt.len() == 0 { 
                             errstring = "********* List: Icon has no value **********".to_string();
                             colorx = Color::from([1.0, 0.0, 0.0]);
                         } else {
                             let icon_int: i32 = sizetxt.parse().unwrap_or(-99);
                             if icon_int > 0 {
                                 if (icon_int < 50) | (icon_int > 255) {
                                     errstring = "********* List: Icon not between 50 and 255 **********".to_string();
                                     colorx = Color::from([1.0, 0.0, 0.0]);
                                 } else {
                                     badsize_int = 0;
                                     icon_int1 = icon_int;
                                 }
                             } else if icon_int == -99 {
                                 errstring = "********* List: Icon is not an integer **********".to_string();
                                 colorx = Color::from([1.0, 0.0, 0.0]);
                             } else {
                                 errstring = "********* List: Icon Size not positive integer **********".to_string();
                                 colorx = Color::from([1.0, 0.0, 0.0]);
                             }
                         }
                     }
                 }
             }
         }
     }
     if badsize_int == 0 {
         badsize_int = 1;
         if !Path::new(&dir_value).exists() {
             errstring = "the directory does not exist".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
         } else {       
             let dir_path = Path::new(&dir_value);
             let (errcd, errstr, liststr) = get_dirlistr(dir_path.to_path_buf());
             if errcd == 0 {
                 if liststr.len() < from_int1 as usize {
                     errstring =  format!("********* List: From {} Greater than number of files of {} **********", from_int1, liststr.len());
                     colorx = Color::from([1.0, 0.0, 0.0]);
                 } else {
                     newliststr = liststr;
                     newliststr.sort();
                     if to_int1 as usize > newliststr.len() {
                         shortto_int1 = newliststr.len() as i32 ;
                     } else {
                         shortto_int1 = to_int1.clone();
                     }
                     errstring = "got directory".to_string();
                     colorx = Color::from([0.0, 0.0, 0.0]);
                     badsize_int = 0;
                 }
             } else {
                 errstring = errstr.to_string();
                 colorx = Color::from([1.0, 0.0, 0.0]);
             }
         }
     }
     if badsize_int != 0 {
         to_int1 = 0;
     }
     (colorx, errstring, newliststr, from_int1, to_int1, shortto_int1, icon_int1 as u32)
}

