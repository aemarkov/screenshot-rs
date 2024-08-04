use xcb::{x};
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;

fn main() -> xcb::Result<()> {
    let (conn, screen_num) = xcb::Connection::connect(None)?;

    // Get root window
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();
    let root = screen.root();

    let (geom, image) = take_screenshot(&conn, &root)?;
    save_png(image.data(), &geom, Path::new("/tmp/screenshot.png"));

    Ok(())
}

fn take_screenshot(conn: &xcb::Connection, root: &x::Window) -> xcb::Result<(x::GetGeometryReply, x::GetImageReply)>
{
    // Get root window size
    let cookie = conn.send_request(&x::GetGeometry {
        drawable: x::Drawable::Window(*root)
    });
    let geom = conn.wait_for_reply(cookie)?;

    // Make screenshot
    let cookie = conn.send_request(&x::GetImage {
        drawable: x::Drawable::Window(*root),
        x: geom.x(),
        y: geom.y(),
        width: geom.width(),
        height: geom.height(),
        format: x::ImageFormat::ZPixmap,
        plane_mask: !0
    });
    let image = conn.wait_for_reply(cookie)?;

    Ok((geom, image))
}

fn save_png(data: &[u8], geometry: &x::GetGeometryReply, path: &Path)
{
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, geometry.width().into(), geometry.height().into());
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let buffer = bgra2rgba(data);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&buffer).unwrap();
}

fn bgra2rgba(in_buf: &[u8]) -> Vec<u8> {
    let mut out_buf = Vec::with_capacity(in_buf.len());

    for chunk in in_buf.chunks(4) {
        out_buf.push(chunk[2]);
        out_buf.push(chunk[1]);
        out_buf.push(chunk[0]);
        out_buf.push(chunk[3]);
    }

    out_buf
}
