use image::{ImageBuffer, Rgb};
use xcb::{self, x, Connection};

pub struct ScreenRes {
    pub width: u16,
    pub height: u16,
}

pub fn get_screen_res() -> Result<ScreenRes, xcb::Error> {
    let (conn, _) = Connection::connect(None)?;
    let setup = conn.get_setup();

    let screen = setup
        .roots()
        .next()
        .ok_or(xcb::Error::Connection(xcb::ConnError::ClosedInvalidScreen))?;

    let width = screen.width_in_pixels();
    let height = screen.height_in_pixels();

    Ok(ScreenRes { height, width })
}

pub fn capture_screen() -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, xcb::Error> {
    let (conn, screen_num) = Connection::connect(None)?;
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let screen_res = get_screen_res()?;
    let mut img = ImageBuffer::new(screen_res.width.into(), screen_res.height.into());

    let get_img = x::GetImage {
        format: x::ImageFormat::ZPixmap,
        drawable: x::Drawable::Window(screen.root()),
        x: 0,
        y: 0,
        width: screen_res.width,
        height: screen_res.height,
        plane_mask: !0,
    };

    let cookie = conn.send_request(&get_img);
    let reply = conn.wait_for_reply(cookie).unwrap();

    let data = reply.data();

    for y in 0..screen_res.height {
        for x in 0..screen_res.width {
            let i = (y as usize * (screen_res.width as usize) + x as usize) * 4;

            let r = data[i + 2];
            let g = data[i + 1];
            let b = data[i];

            img.put_pixel(x as u32, y as u32, Rgb([r, g, b]));
        }
    }

    Ok(img)
}

pub fn capture_num_screens(
    num_screens: usize,
) -> Result<Vec<ImageBuffer<Rgb<u8>, Vec<u8>>>, xcb::Error> {
    let mut imgs = Vec::new();

    for _ in 0..num_screens {
        let img = capture_screen()?;
        imgs.push(img);
    }

    Ok(imgs)
}
