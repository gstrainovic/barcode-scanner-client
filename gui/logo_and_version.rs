use fltk::{frame::{Frame, self}, image, prelude::{ImageExt, WidgetExt}};
use fltk_grid::Grid;
use config::VERSION;

use crate::logo::LOGO;


pub fn logo_and_version() -> Grid {
    fn logo() -> Frame {
        let mut logo = image::SvgImage::from_data(LOGO).unwrap();
        let mut logoframe = frame::Frame::default(); //.with_size(200, 100);
        logo.scale(200, 100, true, true);
        logoframe.set_image(Some(logo));
        logoframe
    }

    let mut slogan = frame::Frame::default().with_label("Einfach persönlich schenken");
    let mut version = frame::Frame::default().with_label(&format!("Version {}", VERSION));

    let mut grid = Grid::default_fill();
    grid.set_layout(24, 3);
    grid.insert_ext(&mut logo(), 0, 0, 3, 3);
    grid.insert_ext(&mut slogan, 3, 0, 3, 1);
    grid.insert_ext(&mut version, 5, 0, 3, 1);
    grid
}