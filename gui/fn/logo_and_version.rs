fn logo_and_version() -> Grid {
    fn logo() -> Frame {
        let mut logo = image::SvgImage::load("gui/gravurzeile-logo.svg").unwrap();
        let mut logoframe = frame::Frame::default(); //.with_size(200, 100);
        logo.scale(200, 100, true, true);
        logoframe.set_image(Some(logo));
        logoframe
    }

    fn slogan() -> Frame {
        return frame::Frame::default().with_label("Einfach persönlich schenken");
    }

    fn version() -> Frame {
        return frame::Frame::default().with_label(&format!("Version {}", cargo_crate_version!()));
    }

    let mut grid = Grid::default_fill();
    grid.set_layout(24, 3);
    // widget, row, col, row_span, col_span
    grid.insert_ext(&mut logo(), 0, 0, 3, 3);
    grid.insert_ext(&mut slogan(), 3, 0, 3, 1);
    grid.insert_ext(&mut version(), 5, 0, 3, 1);
    grid
}