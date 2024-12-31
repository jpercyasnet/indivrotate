use iced::alignment::{Alignment};
use iced::theme::{Theme};
use iced::widget::{
    button, checkbox, column, row, scrollable, text,
    image, container, Column, Row, text_input, Space
};
use iced::event::{self, Event};
use iced::window;
use iced::{Element};
use iced::{Center, Color, Task, Length, Size};
use iced::Subscription;
use serde::{Deserialize, Serialize};

use std::process::Command as stdCommand;
use std::time::Instant;
use std::path::Path;
use std::env;

extern crate image as create_image;
mod get_winsize;
mod dirpressr;
mod get_dirlistr;

mod listpressi;
mod nextpressi;
mod firstpressi;
mod rotatepressi;
use listpressi::listpressi;
use nextpressi::nextpressi;
use firstpressi::firstpressi;
use rotatepressi::rotatepressi;

use get_dirlistr::get_dirlistr;
use get_winsize::get_winsize;
use dirpressr::dirpressr;

pub fn main() -> iced::Result {
     let mut widthxx: f32 = 1350.0;
     let mut heightxx: f32 = 750.0;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho as f32 - 20.0;
         heightxx = heighto as f32 - 75.0;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }
     iced::application(ImageList::title, ImageList::update, ImageList::view)
        .window_size((widthxx, heightxx))
        .theme(ImageList::theme)
        .subscription(ImageList::subscription)
        .run_with(ImageList::new)


}

#[derive(Debug)]
enum ImageList {
    Loaded(State),
    
}

#[derive(Debug, Default)]
struct State {
    filter: Filter,
    images: Vec<ImageItem>,
    dir_value: String,
    msg_value: String,
    mess_color: Color,
    from_value: String,
    to_value: String,
    size_value: String,
    screenwidth: f32,
}

#[derive(Debug, Clone)]
enum Message {
    FilterChanged(Filter),
    ImageMessage(usize, ImageMessage),
    DirPressed,
    ListPressed,
    NextGroupPressed,
    FirstGroupPressed,
    RotateClockwisePressed,
    RotateCounterClockwisePressed,
    Rotate180Pressed,
    RotatexFound(Result<Rotatex, Error>),
    FromChanged(String),
    ToChanged(String),
    SizeChanged(String),
    Size(Size),
}

impl ImageList {
    fn new() -> (Self, Task<Message>) {
        let mut parmdir = "no directory".to_string();
        let mut msgclr = Color::from([0.0, 0.0, 1.0]);
        let mut msgval = "no message".to_string();
        let args: Vec<_> = env::args().collect();
        if args.len() > 1 {
            println!("The first argument is {}", args[1]);
            if Path::new(&args[1]).exists() {
                parmdir = args[1].to_string();
                msgclr = Color::from([0.0, 1.0, 0.0]);
                msgval = "got a existing item. Hopefully a directory".to_string();
            } else {
                msgclr = Color::from([1.0, 0.0, 0.0]);
                msgval = format!("parameter directory of {} does not exist", args[1]);
            }
        } else {
            println!(" no input parameters");
        }
        let mut widthxx: u32 = 1300;
        let (errcode, errstring, widtho, _heighto) = get_winsize();
        if errcode == 0 {
            widthxx = widtho;
            println!("{}", errstring);
        } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
        }

        (
            ImageList::Loaded(State
               {
                filter:Filter::All,
                images:Vec::<ImageItem>::new(),
                dir_value: parmdir.to_string(),
                mess_color: msgclr,
                msg_value: msgval.to_string(),
                from_value: "1".to_string(),
                to_value: "16".to_string(),
                size_value: "160".to_string(),
                screenwidth: widthxx as f32,
                }
            ),
            Task::none(),
        )
    }

    fn title(&self) -> String {
        format!("Individual Rotation -- iced")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            ImageList::Loaded(state) => {

                let taskx = match message {
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Task::none()
                    }
                    Message::ImageMessage(i, image_message) => {
                        if let Some(image) = state.images.get_mut(i) {

                            image.update(image_message);

                               Task::none()
                        } else {
                            Task::none()
                        }
                    }
                    Message::DirPressed => {
                        let (errcode, errstr, newdir, _listitems) = dirpressr();
                        state.msg_value = errstr.to_string();
                        if errcode == 0 {
                            state.dir_value = newdir.to_string();
                            state.mess_color = Color::from([0.0, 1.0, 0.0]);
                        } else {
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                        } 
                        Task::none()
                    } 

                    Message::Size(size) => {
                         state.screenwidth = size.width;
                         Task::none()
                    }



                    Message::ListPressed => {
                       let (errcd, errstr, listitems, from_int1, _to_int1, newtoi, totfiles, icon_int1) = listpressi(state.dir_value.clone(), state.from_value.clone(), state.to_value.clone(), state.size_value.clone());
                       if errcd == 0 {
                           state.msg_value = format!("from: {}   to: {}  of {} images", from_int1, newtoi, totfiles);
                           state.images.clear();                         
                           for indexi in (from_int1 - 1)..newtoi {
                                let fullpath = state.dir_value.clone() + "/" + &listitems[indexi as usize];
                                let newwidth: u32;
                                let newheight: u32;
                                if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                    if iwidth > iheight {
                                        newwidth = icon_int1;
                                        newheight = icon_int1 * iheight / iwidth;
                                    } else {
                                        newheight = icon_int1;
                                        newwidth = icon_int1 * iwidth / iheight;
                                    }
                                    let loadimg = create_image::open(fullpath.clone()).unwrap();
                                    let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                    let rgbconv = imgbuffer.into_vec();
                                    state
                                       .images
                                       .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight, icon_int1.clone()));

                                }
                            }
                            state.mess_color = Color::from([0.0, 1.0, 0.0]);          
                       } else {
                            state.msg_value = errstr.to_string();
                            state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };
                       Task::none()
                    }
                    Message::NextGroupPressed => {
                       let (errcode, errstrx, fromstr, tostr) = nextpressi(state.dir_value.clone(), state.from_value.clone(), state.to_value.clone());
                       if errcode == 0 {
                           let (errcd, errstr, listitems, from_int1, _to_int1, newtoi, totfiles, icon_int1) = listpressi(state.dir_value.clone(), fromstr.clone(), tostr.clone(), state.size_value.clone());
                           if errcd == 0 {
                               state.msg_value = format!("from: {}   to: {}  of {} images", from_int1, newtoi, totfiles);
                               state.images.clear();                         
                               for indexi in (from_int1 - 1)..newtoi {
                                    let fullpath = state.dir_value.clone() + "/" + &listitems[indexi as usize];
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        state
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight, icon_int1.clone()));
                                    }
                               }
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);          
                               state.from_value = fromstr.to_string();
                               state.to_value = tostr.to_string();
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       } else {
                          state.msg_value = errstrx.to_string();
                          state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };
                       Task::none()
                    }
                    Message::FirstGroupPressed => {
                       let (errcode, errstrx, fromstr, tostr) = firstpressi(state.dir_value.clone(), state.from_value.clone(), state.to_value.clone());
                       if errcode == 0 {
                           let (errcd, errstr, listitems, from_int1, _to_int1, newtoi, totfiles, icon_int1) = listpressi(state.dir_value.clone(), fromstr.clone(), tostr.clone(), state.size_value.clone());
                           if errcd == 0 {
                               state.msg_value = format!("from: {}   to: {}  of {} images", from_int1, newtoi, totfiles);
                               state.images.clear();                         
                               for indexi in (from_int1 - 1)..newtoi {
                                    let fullpath = state.dir_value.clone() + "/" + &listitems[indexi as usize];
                                    let newwidth: u32;
                                    let newheight: u32;
                                    if let Ok((iwidth, iheight)) = create_image::image_dimensions(fullpath.clone()) {
                                        if iwidth > iheight {
                                            newwidth = icon_int1;
                                            newheight = icon_int1 * iheight / iwidth;
                                        } else {
                                            newheight = icon_int1;
                                            newwidth = icon_int1 * iwidth / iheight;
                                        }
                                        let loadimg = create_image::open(fullpath.clone()).unwrap();
                                        let imgbuffer = create_image::imageops::thumbnail(&loadimg, newwidth, newheight);
                                        let rgbconv = imgbuffer.into_vec();
                                        state
                                           .images
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight, icon_int1.clone()));
                                    }
                               }
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);          
                               state.from_value = fromstr.to_string();
                               state.to_value = tostr.to_string();
                           } else {
                              state.msg_value = errstr.to_string();
                              state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           }
                       } else {
                          state.msg_value = errstrx.to_string();
                          state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       };
                       Task::none()
                    }
                    Message::RotateClockwisePressed => {
                       let errfnd: i32;
                       let mut listofimages: Vec<String> = Vec::new();
                       let images_left = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_left < 1 {
                           state.msg_value = "no images selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           errfnd = 1;
                       } else {
                           for imagesy in state.images.iter() {
                                if imagesy.completed {
                                    listofimages.push(imagesy.description.clone());
                                }
                           }
                           let (errcode, errstr) = rotatepressi(state.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           Task::perform(Rotatex::rotateit(state.dir_value.clone(), listofimages.clone(), 0), Message::RotatexFound)
                       } else {
                           Task::none()
                       }
                    }
                    Message::RotateCounterClockwisePressed => {
                       let errfnd: i32;
                       let mut listofimages: Vec<String> = Vec::new();
                       let images_left = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_left < 1 {
                           state.msg_value = "no images selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           errfnd = 1;
                       } else {
                           for imagesy in state.images.iter() {
                                if imagesy.completed {
                                    listofimages.push(imagesy.description.clone());
                                }
                           }
                           let (errcode, errstr) = rotatepressi(state.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           Task::perform(Rotatex::rotateit(state.dir_value.clone(), listofimages.clone(), 1), Message::RotatexFound)
                       } else {
                           Task::none()
                       }
                    }
                    Message::Rotate180Pressed => {
                       let errfnd: i32;
                       let mut listofimages: Vec<String> = Vec::new();
                       let images_left = state.images.iter().filter(|imageitem| imageitem.completed).count();
                       if images_left < 1 {
                           state.msg_value = "no images selected".to_string();
                           state.mess_color = Color::from([1.0, 0.0, 0.0]);
                           errfnd = 1;
                       } else {
                           for imagesy in state.images.iter() {
                                if imagesy.completed {
                                    listofimages.push(imagesy.description.clone());
                                }
                           }
                           let (errcode, errstr) = rotatepressi(state.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = Color::from([1.0, 0.0, 0.0]);
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           Task::perform(Rotatex::rotateit(state.dir_value.clone(), listofimages.clone(), 2), Message::RotatexFound)
                       } else {
                           Task::none()
                       }
                    }
                    Message::RotatexFound(Ok(copyx)) => {
                       state.msg_value = copyx.errval.clone();
                       state.mess_color = copyx.errcolor.clone();
                       Task::none()
                    }
                    Message::RotatexFound(Err(_error)) => {
                       state.msg_value = "error in copyx copyit routine".to_string();
                       state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       Task::none()
                    }
                    Message::FromChanged(value) => { state.from_value = value; Task::none() }
                    Message::ToChanged(value) => { state.to_value = value; Task::none() }
                    Message::SizeChanged(value) => { state.size_value = value; Task::none() }

                };

                Task::batch(vec![taskx, Task::none()])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            ImageList::Loaded(State {
                filter,
                images,
                dir_value,
                msg_value,
                mess_color,
                from_value,
                to_value,
                size_value,
                screenwidth,
                ..
            }) => {
                let mut messcol = Column::new().spacing(10);
                messcol = messcol.push(container(row![text("Message:").size(20),
                 text(msg_value).size(20).color(*mess_color),].align_y(Alignment::Center).spacing(10).padding(10)
                 ));

                let mut dirbutshow = Column::new().spacing(10);
                dirbutshow = dirbutshow.push(container(row![button("DirectoryButton").on_press(Message::DirPressed),
                 text(dir_value).size(20),].align_y(Alignment::Center).spacing(100).padding(10),
                 ));

                let mut butcol = Column::new().spacing(10);
                butcol = butcol.push(container(row![button("List").on_press(Message::ListPressed),
                 button("Next Group").on_press(Message::NextGroupPressed),
                 button("First Group").on_press(Message::FirstGroupPressed),
                 button("Rotate Clockwise").on_press(Message::RotateClockwisePressed),
                 button("Rotate CounterClockwise").on_press(Message::RotateCounterClockwisePressed),
                 button("Rotate 180").on_press(Message::Rotate180Pressed),
                 ].align_y(Alignment::Center).spacing(100).padding(10),
                 ));

                let mut fromtosize = Column::new().spacing(10);
                fromtosize = fromtosize.push(container(row![text("From: ").size(20),
                 text_input("1", from_value)
                            .on_input(Message::FromChanged).padding(10).size(20),
                 text("                    To: "),
                 text_input("16", to_value).on_input(Message::ToChanged).padding(10).size(20),
                 text("                    Icon Size: "),
                 text_input("160", size_value).on_input(Message::SizeChanged).padding(10).size(20),
                 ].align_y(Alignment::Center).spacing(20).padding(10),
                 ));

                let controls = view_controls(images, *filter);
                let filtered_images =
                    images.iter().filter(|imageitem| filter.matches(imageitem));

                let mut imagescol1 = Column::new().spacing(10);
                let mut imagescol2 = Column::new().spacing(10);
                let mut imagescol3 = Column::new().spacing(10);
                let mut imagescol4 = Column::new().spacing(10);
                let mut imagescol5 = Column::new().spacing(10);
                let mut imagescol6 = Column::new().spacing(10);
                let mut imagescol7 = Column::new().spacing(10);
                let mut numcol = 5;

                let winwidth: f32 = screenwidth - 20.0;
                if winwidth > 1800.0 {
                     numcol = 7;
                } else if winwidth > 1540.0 {
                     numcol = 6;
                }
                let mut colpos = 0;
                let mut n = 0;
                if filtered_images.clone().count() == 0 {
                    n = 1;
                    imagescol1 = imagescol1.push(container(row![empty_message(match filter {
                        Filter::All => "No directory selected or no files in directory",
                        Filter::Active => "All files have been selected",
                        Filter::Completed => {
                            "No files have been selected" }
                    })]));
                } else {
                    for imagesy in images.iter() {
                         if imagesy.completed {
                             if (filter == &Filter::All) || (filter == &Filter::Completed) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 2;
                               } else if colpos == 2 {
                                 imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 3;
                               } else if colpos == 3 {
                                 imagescol4 = imagescol4.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 4;
                               } else if colpos == 4 {
                                 imagescol5 = imagescol5.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if numcol < 6 {
                                     colpos = 0;
                                 } else {
                                     colpos = 5;
                                 }
                               } else if colpos == 5 {
                                 imagescol6 = imagescol6.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if numcol < 7 {
                                     colpos = 0;
                                 } else {
                                     colpos = 6;
                                 }
                               } else if colpos == 6 {
                                 imagescol7 = imagescol7.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                             }
                        } else {
                             if (filter == &Filter::All) || (filter == &Filter::Active) {
                               if colpos == 0 {
                                 imagescol1 = imagescol1.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos  = 1;
                               } else if colpos == 1 {
                                 imagescol2 = imagescol2.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 2;
                               } else if colpos == 2 {
                                 imagescol3 = imagescol3.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 3;
                               } else if colpos == 3 {
                                 imagescol4 = imagescol4.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 4;
                               } else if colpos == 4 {
                                 imagescol5 = imagescol5.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if numcol < 6 {
                                     colpos = 0;
                                 } else {
                                     colpos = 5;
                                 }
                               } else if colpos == 5 {
                                 imagescol6 = imagescol6.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 if numcol < 7 {
                                     colpos = 0;
                                 } else {
                                     colpos = 6;
                                 }
                               } else if colpos == 6 {
                                 imagescol7 = imagescol7.push(container(row![imagesy.view(n).map(move |message| {
                                    Message::ImageMessage(n, message)
                                   })]));
                                 colpos = 0;
                               }
                             }
                         }
                         n = n + 1;
                    }
                }
                let mut imagesrow = Row::new().spacing(20);
                imagesrow = imagesrow.push(container(imagescol1).padding(10).width(Length::Fixed(250.0)));
                if n > 1 {
                    imagesrow = imagesrow.push(container(imagescol2).padding(10).width(Length::Fixed(250.0)));
                    if n > 2 {
                       imagesrow = imagesrow.push(container(imagescol3).padding(10).width(Length::Fixed(250.0)));
                       if n > 3 {
                           imagesrow = imagesrow.push(container(imagescol4).padding(10).width(Length::Fixed(250.0)));
                           if n > 4 {
                               imagesrow = imagesrow.push(container(imagescol5).padding(10).width(Length::Fixed(250.0)));
                               if n > 5 && numcol > 5 {
                                   imagesrow = imagesrow.push(container(imagescol6).padding(10).width(Length::Fixed(250.0)));
                                   if n > 6 && numcol > 6 {
                                       imagesrow = imagesrow.push(container(imagescol7).padding(10).width(Length::Fixed(250.0)));
                                   }
                               }
                           }
                       }
                    }
                }
                let scrollable_content: Element<Message> =
                  Element::from(scrollable(
                    imagesrow
                )
                .height(Length::Fill)
                .spacing(10)
                .direction({
                    let scrollbar = scrollable::Scrollbar::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10);
//                        .anchor(self.anchor);

                    scrollable::Direction::Both {
                        horizontal: scrollbar,
                        vertical: scrollbar,
                    }
                 })
                ); 

                column![messcol, dirbutshow, butcol, fromtosize, controls, scrollable_content]
                    .spacing(1)
                    .max_width(winwidth)
                    .into()
            }
        }
    }
    fn theme(&self) -> Theme {
       Theme::Dracula
    }
    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _status, _window| match event {
            Event::Window(window::Event::Resized(size)) => {
                Some(Message::Size(size))
            }
            _ => None,
        })
    }

}

#[derive(Debug, Clone)]
struct ImageItem {
    description: String,
    completed: bool,
    rgbconv: Vec<u8>,
    twidth: u32,
    theight: u32,
    ticon: u32,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
    Completed(bool),
}

impl ImageItem {

    fn new(description: String, rgbconv: Vec<u8>, twidth:  u32, theight: u32, ticon: u32) -> Self {
        ImageItem {
            description,
            completed: false,
            rgbconv,
            twidth,
            theight,
            ticon,
        }
    }

    fn update(&mut self, message: ImageMessage) {
        match message {
            ImageMessage::Completed(completed) => {
                self.completed = completed;
            }
        }
    }

    fn view(&self, _i: usize) -> Element<ImageMessage> {
        let checkbox = checkbox(
            &self.description,
            self.completed).on_toggle(ImageMessage::Completed).width(Length::Fill);
        let newimage = image::Handle::from_rgba(self.twidth.clone(), self.theight.clone(), self.rgbconv.clone()); 
        let newiconh = self.ticon as f32 + 5.0;
        column![
           container(
        // This should go away once we unify resource loading on native
        // platforms
             image::Viewer::new(newimage)
                 .height(Length::Fixed(newiconh)),
           )
           .width(Length::Fill),
            checkbox,
        ]
        .align_x(Alignment::Center)
        .spacing(5)
        .into()

    }
}

fn view_controls(images: &[ImageItem], current_filter: Filter) -> Element<Message> {
    let images_left = images.iter().filter(|imageitem| imageitem.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![Space::with_width(20),
        text(format!(
            "{} {} selected",
            images_left,
            if images_left == 1 { "file" } else { "files" }
        ))
        .width(Length::Fill)
        .size(16),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Not Selected", Filter::Active, current_filter),
            filter_button("Selected", Filter::Completed, current_filter,),
        ]
        .width(Length::Shrink)
        .spacing(10)
    ]
    .spacing(20)
    .align_y(Alignment::Center)
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

impl Filter {
    fn matches(&self, imageitem: &ImageItem) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !imageitem.completed,
            Filter::Completed => imageitem.completed,
        }
    }
}
fn empty_message(message: &str) -> Element<'_, Message> {
    container(
        text(message)
            .width(Length::Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .into()
}

#[derive(Debug, Clone)]
struct Rotatex {
    errcolor: Color,
    errval: String,
}

impl Rotatex {

    async fn rotateit(dir_value: String, listofimages: Vec<String>, rottype: i32) -> Result<Rotatex, Error> {
     let mut errstring  = " ".to_string();
     let mut colorx = Color::from([0.0, 0.0, 0.0]);
     let mut bolok = true;
     let mut rottypestr = " ".to_string();
     if rottype == 0 {
         rottypestr = "/home/jp/gimprotck.sh".to_string();
     } else if rottype == 1 {
         rottypestr = "/home/jp/gimprotcck.sh".to_string();
     } else if rottype == 2 {
         rottypestr = "/home/jp/gimprot180.sh".to_string();
     } else {
         errstring = format!("********* invalid rottype of {} **********",rottype);
         colorx = Color::from([1.0, 0.0, 0.0]);
         bolok = false;
     }
     let start_time = Instant::now();
     let lenmg1 = listofimages.len();
     if bolok {
         let mut numrot = 0;
         let mut numprocess = 0;
         for indl in 0..lenmg1 {
              numrot = numrot + 1;
              let str_cur_dirfrom = dir_value.clone();
              let fullfrom = str_cur_dirfrom.clone() + "/" + &listofimages[indl].clone();
              if !Path::new(&fullfrom).exists() {
                  errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
                  colorx = Color::from([1.0, 0.0, 0.0]);
                  bolok = false;
                  break;
              }
              if (numrot < lenmg1) & (numprocess < 4) {
                  stdCommand::new(&rottypestr.clone())
                               .arg(&fullfrom)
                               .spawn()
                               .expect("failed to execute process");
                  numprocess = numprocess + 1;
              } else {
                  let _output = stdCommand::new(&rottypestr.clone())
                                             .arg(&fullfrom)
                                             .output()
                                             .expect("failed to execute process");
                  numprocess = 0;
              }
         }
     }
     if bolok {
         let diffx = start_time.elapsed();     
         errstring = format!("rotated {} files in {} seconds", lenmg1, diffx.as_secs());
         colorx = Color::from([0.0, 0.0, 0.0]);
     }
     Ok(Rotatex {
            errcolor: colorx,
            errval: errstring,
        })
    }
}
#[derive(Debug, Clone)]
enum Error {
//    APIError,
}

