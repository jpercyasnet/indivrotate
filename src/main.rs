use iced::alignment::{self, Alignment};
use iced::widget::scrollable::{Properties};
use iced::theme::{self, Theme};
use iced::widget::{
    button, checkbox, column, row, scrollable, text, horizontal_space,
    image, container, Column, Row, text_input
};

use iced::window;
use iced::{Application, Element};
use iced::{Color, Command, Length, Settings};

use serde::{Deserialize, Serialize};

// use iced_futures::futures;
// use futures::channel::mpsc;
use std::process::Command as stdCommand;
// use std::time::{Duration, Instant};
use std::time::{Instant};
use std::path::{Path};


extern crate image as create_image;
mod get_winsize;
// mod dump_file;
mod dirpressr;
mod get_dirlistr;

// mod get_dirlisti;

mod listpressi;
mod nextpressi;
mod firstpressi;
mod rotatepressi;
use listpressi::listpressi;
use nextpressi::nextpressi;
use firstpressi::firstpressi;
use rotatepressi::rotatepressi;
// use get_dirlisti::get_dirlisti;
// use dirpressy::dirpressx;

use get_dirlistr::get_dirlistr;
use get_winsize::get_winsize;
use dirpressr::dirpressr;

pub fn main() -> iced::Result {
     let mut widthxx: u32 = 1350;
     let mut heightxx: u32 = 750;
     let (errcode, errstring, widtho, heighto) = get_winsize();
     if errcode == 0 {
         widthxx = widtho;
         heightxx = heighto;
         println!("{}", errstring);
     } else {
         println!("**ERROR {} get_winsize: {}", errcode, errstring);
     }

     ImageList::run(Settings {
        window: window::Settings {
            size: (widthxx, heightxx),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
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
//    tx_send: mpsc::UnboundedSender<String>,
//    rx_receive: mpsc::UnboundedReceiver<String>,
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
}

impl Application for ImageList {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (ImageList, Command<Message>) {

        (
//            ImageList::Loaded(State::default()),
            ImageList::Loaded(State
               {
                filter:Filter::All,
                images:Vec::<ImageItem>::new(),
                dir_value: "no directory".to_string(),
                mess_color: Color::from([0.0, 0.0, 0.0]),
                msg_value: "no message".to_string(),
//                scrol_value: " No directory selected \n ".to_string(),
                from_value: "1".to_string(),
                to_value: "16".to_string(),
                size_value: "160".to_string(),
//                tx_send,
//                rx_receive,
                }
            ),
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("Individual Rotation -- iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            ImageList::Loaded(state) => {

                let command = match message {
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Command::none()
                    }
                    Message::ImageMessage(i, image_message) => {
                        if let Some(image) = state.images.get_mut(i) {

                            image.update(image_message);

                               Command::none()
                        } else {
                            Command::none()
                        }
                    }
                    Message::DirPressed => {
                        let (colorout, errstr, newdir, _listitems) = dirpressr();
                        if errstr == "got directory" {
                            state.dir_value = newdir.to_string();
                        } 
                        state.msg_value = errstr.to_string();
                        state.mess_color = colorout;

                        Command::none()
                    } 
                    Message::ListPressed => {
                       let (colorout, errstr, listitems, from_int1, _to_int1, newtoi, icon_int1) = listpressi(state.dir_value.clone(), state.from_value.clone(), state.to_value.clone(), state.size_value.clone());
                       if newtoi != 0 {
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
                                       .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight));

                                }
                            }
                       }
                       state.msg_value = errstr.to_string();
                       state.mess_color = colorout;
                       Command::none()
                    }
                    Message::NextGroupPressed => {
                       let (errcode, coloroutx, errstrx, fromstr, tostr) = nextpressi(state.dir_value.clone(), state.from_value.clone(), state.to_value.clone());
                       let mut errcolor: Color = coloroutx;
                       let mut errstring: String = errstrx.to_string();
                       if errcode == 0 {
                           let (colorout, errstr, listitems, from_int1, _to_int1, newtoi, icon_int1) = listpressi(state.dir_value.clone(), fromstr.clone(), tostr.clone(), state.size_value.clone());
                           if newtoi != 0 {
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
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight));
                                    }
                                }
                                state.from_value = fromstr.to_string();
                                state.to_value = tostr.to_string();
                           }
                           errcolor = colorout;
                           errstring = errstr.to_string();
                       }
                       state.msg_value = errstring.to_string();
                       state.mess_color = errcolor;
                       Command::none()
                    }
                    Message::FirstGroupPressed => {
                       let (errcode, coloroutx, errstrx, fromstr, tostr) = firstpressi(state.dir_value.clone(), state.from_value.clone(), state.to_value.clone());
                       let mut errcolor: Color = coloroutx;
                       let mut errstring: String = errstrx.to_string();
                       if errcode == 0 {
                           let (colorout, errstr, listitems, from_int1, _to_int1, newtoi, icon_int1) = listpressi(state.dir_value.clone(), fromstr.clone(), tostr.clone(), state.size_value.clone());
                           if newtoi != 0 {
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
                                           .push(ImageItem::new(listitems[indexi as usize].clone(), rgbconv, newwidth, newheight));
                                    }
                                }
                                state.from_value = fromstr.to_string();
                                state.to_value = tostr.to_string();
                           }
                           errcolor = colorout;
                           errstring = errstr.to_string();
                       }
                       state.msg_value = errstring.to_string();
                       state.mess_color = errcolor;
                       Command::none()
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
                           let (errcode, colorout, errstr) = rotatepressi(state.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = colorout;
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           Command::perform(Rotatex::rotateit(state.dir_value.clone(), listofimages.clone(), 0), Message::RotatexFound)
                       } else {
                           Command::none()
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
                           let (errcode, colorout, errstr) = rotatepressi(state.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = colorout;
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           Command::perform(Rotatex::rotateit(state.dir_value.clone(), listofimages.clone(), 1), Message::RotatexFound)
                       } else {
                           Command::none()
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
                           let (errcode, colorout, errstr) = rotatepressi(state.dir_value.clone(), listofimages.clone());
                           if errcode == 0 {
                               state.msg_value = format!("{} images selected {}", images_left, listofimages[0]);
                               state.mess_color = Color::from([0.0, 1.0, 0.0]);
                               errfnd = 0;
                           } else {
                               state.msg_value = errstr.to_string();
                               state.mess_color = colorout;
                               errfnd = 2;
                           }
                       }
                       if errfnd == 0 {
                           Command::perform(Rotatex::rotateit(state.dir_value.clone(), listofimages.clone(), 2), Message::RotatexFound)
                       } else {
                           Command::none()
                       }
                    }
                    Message::RotatexFound(Ok(copyx)) => {
                       state.msg_value = copyx.errval.clone();
                       state.mess_color = copyx.errcolor.clone();
                       Command::none()
                    }
                    Message::RotatexFound(Err(_error)) => {
                       state.msg_value = "error in copyx copyit routine".to_string();
                       state.mess_color = Color::from([1.0, 0.0, 0.0]);
                       Command::none()
                    }
                    Message::FromChanged(value) => { state.from_value = value; Command::none() }
                    Message::ToChanged(value) => { state.to_value = value; Command::none() }
                    Message::SizeChanged(value) => { state.size_value = value; Command::none() }

                };

                Command::batch(vec![command, Command::none()])
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
                ..
            }) => {
                let title = text("List Images in a Directory")
                    .width(Length::Fill)
                    .size(20)
                    .style(Color::from([0.5, 0.5, 0.5]))
                    .horizontal_alignment(alignment::Horizontal::Center);

                let mut messcol = Column::new().spacing(10);
                messcol = messcol.push(container(row![text("Message:").size(30),
                 text(msg_value).size(30).style(*mess_color),
            ].align_items(Alignment::Center).spacing(10).padding(10)
                    ));

                let mut dirbutshow = Column::new().spacing(10);
                dirbutshow = dirbutshow.push(container(row![button("DirectoryButton").on_press(Message::DirPressed).style(theme::Button::Secondary),
                 text(dir_value).size(20),].align_items(Alignment::Center).spacing(100).padding(10),
                 ));

                let mut butcol = Column::new().spacing(10);
                butcol = butcol.push(container(row![button("List").on_press(Message::ListPressed),
                 button("Next Group").on_press(Message::NextGroupPressed),
                 button("First Group").on_press(Message::FirstGroupPressed),
                 button("Rotate Clockwise").on_press(Message::RotateClockwisePressed),
                 button("Rotate CounterClockwise").on_press(Message::RotateCounterClockwisePressed),
                 button("Rotate 180").on_press(Message::Rotate180Pressed),
                 ].align_items(Alignment::Center).spacing(100).padding(10),
                 ));

                let mut fromtosize = Column::new().spacing(10);
                fromtosize = fromtosize.push(container(row![text("From: ").size(20),
                 text_input("1", from_value)
                            .on_input(Message::FromChanged).padding(10).size(20),
                 text("                    To: "),
                 text_input("16", to_value).on_input(Message::ToChanged).padding(10).size(20),
                 text("                    Icon Size: "),
                 text_input("160", size_value).on_input(Message::SizeChanged).padding(10).size(20),
                 ].align_items(Alignment::Center).spacing(20).padding(10),
                 ));

                let controls = view_controls(images, *filter);
                let filtered_images =
                    images.iter().filter(|imageitem| filter.matches(imageitem));

                let mut imagescol1 = Column::new().spacing(10);
                let mut imagescol2 = Column::new().spacing(10);
                let mut imagescol3 = Column::new().spacing(10);
                let mut imagescol4 = Column::new().spacing(10);
                let mut imagescol5 = Column::new().spacing(10);
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
                           }
                       }
                    }
                }

                let scrollable_content: Element<Message> =
                  Element::from(scrollable(
                    imagesrow
                )
                .height(Length::Fill)
                .horizontal_scroll(
                    Properties::new()
                        .width(10)
                        .margin(10)
                        .scroller_width(10),
                )); 

                column![title, messcol, dirbutshow, butcol, fromtosize, controls, scrollable_content]
                    .spacing(20)
                    .max_width(1300)
                .into()
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ImageItem {
    description: String,
    completed: bool,
    rgbconv: Vec<u8>,
    twidth: u32,
    theight: u32,
}

#[derive(Debug, Clone)]
pub enum ImageMessage {
    Completed(bool),
}

impl ImageItem {

    fn new(description: String, rgbconv: Vec<u8>, twidth:  u32, theight: u32,) -> Self {
        ImageItem {
            description,
            completed: false,
            rgbconv,
            twidth,
            theight,
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
            self.completed,
            ImageMessage::Completed,
        )
        .width(Length::Fill);
        let newimage = image::Handle::from_pixels(self.twidth.clone(), self.theight.clone(), self.rgbconv.clone()); 

        column![
           container(
        // This should go away once we unify resource loading on native
        // platforms
             image::Viewer::new(newimage)
                 .height(Length::Fixed(300.0)),
           )
           .width(Length::Fill),
            checkbox,
        ]
        .align_items(Alignment::Center)
        .spacing(5)
        .into()

    }
}

fn view_controls(images: &[ImageItem], current_filter: Filter) -> Element<Message> {
    let images_left = images.iter().filter(|imageitem| imageitem.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label).size(16);

        let button = button(label).style(if filter == current_filter {
            theme::Button::Primary
        } else {
            theme::Button::Text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![horizontal_space(20),
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
    .align_items(Alignment::Center)
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
            .horizontal_alignment(alignment::Horizontal::Center)
            .style(Color::from([0.7, 0.7, 0.7])),
    )
    .width(Length::Fill)
    .height(Length::Fixed(200.0))
    .center_y()
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

