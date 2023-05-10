use iced::Color;
use std::path::Path;

pub fn rotatepressi (dir_value: String, listofimages: Vec<String>) -> (u32, Color, String) {
     let mut errcode: u32 = 0;
     let mut errstring: String = "xx".to_string();
     let mut colorx: Color = Color::from([0.0, 1.0, 0.0]);
     let mut numrow = 0;
     let mut bolok = true;
     if Path::new(&dir_value).exists() {
         let lenmg1 = listofimages.len();
         if lenmg1 < 1 {
             errstring = "no images to rotatex".to_string();
             colorx = Color::from([1.0, 0.0, 0.0]);
             errcode = 1;
         } else {
//             lenmg1 = lenmg1 - 1;
             for indl in 0..lenmg1 {
                let str_cur_dirfrom = dir_value.clone();
                let fullfrom = str_cur_dirfrom.clone() + "/" + &listofimages[indl].clone();
                if !Path::new(&fullfrom).exists() {
                    errstring = format!("********* ERROR {} does not exist **********",fullfrom);
                    colorx = Color::from([1.0, 0.0, 0.0]);
                    bolok = false;
                    errcode = 2;
                    break;
                } else {
                    numrow = numrow + 1;
                }
             }
             if bolok {
                 if numrow > 0 {
                     errstring = "Rotating in Progress".to_string();
                     colorx = Color::from([0.0, 1.0, 0.0]);
                     errcode = 0;
                 } else {
                     errstring = "no images to rotatey".to_string();
                     colorx = Color::from([1.0, 0.0, 0.0]);
                     errcode = 3;
                 }
             }
         }
     } else {
         errstring = "the directory does not exist".to_string();
         colorx = Color::from([1.0, 0.0, 0.0]);
         errcode = 4;
     }
     (errcode, colorx, errstring)
}

