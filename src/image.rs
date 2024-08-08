//! Common structures for working with images

pub struct Image {
    pub size: Size,
    pub data: Vec<u8>,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}
