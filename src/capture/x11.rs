//! This module implements ScreenCapture trait for X11

use crate::capture::ScreenCapture;
use anyhow::{anyhow, Result};
use image::RgbaImage;
use xcb::x;

pub struct X11Capture;

impl ScreenCapture for X11Capture {
    fn all() -> Result<RgbaImage> {
        // TODO: this is a common code, need to be extracted to the method
        // However, we have to keep conn, setup and screen objects, because they
        // borrows conn. Too many objects to return from method.
        let (conn, screen_num) = xcb::Connection::connect(None)?;
        let setup = conn.get_setup();
        let screen = setup
            .roots()
            .nth(screen_num as usize)
            .ok_or_else(|| anyhow!("No screen"))?;
        let root = screen.root();

        let (geom, buffer) = Self::capture(&conn, &root)?;
        // TODO: probably, there is a better and faster way to swap channels
        let buffer = Self::bgra2rgba(buffer.data());

        RgbaImage::from_raw(geom.width().into(), geom.height().into(), buffer)
            .ok_or_else(|| anyhow!("Failed to create image"))
    }
}

impl X11Capture {
    fn capture(
        conn: &xcb::Connection,
        window: &x::Window,
    ) -> Result<(x::GetGeometryReply, x::GetImageReply)> {
        // Get window size
        let cookie = conn.send_request(&x::GetGeometry {
            drawable: x::Drawable::Window(*window),
        });
        let geom = conn.wait_for_reply(cookie)?;

        // Make screenshot
        let cookie = conn.send_request(&x::GetImage {
            drawable: x::Drawable::Window(*window),
            x: geom.x(),
            y: geom.y(),
            width: geom.width(),
            height: geom.height(),
            format: x::ImageFormat::ZPixmap,
            plane_mask: !0,
        });
        let buffer = conn.wait_for_reply(cookie)?;

        Ok((geom, buffer))
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
}
