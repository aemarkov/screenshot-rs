//! This module defines abstractions for screen capturing

pub mod x11;
use crate::image::Image;

/// Trait for screen capturing operations
pub trait ScreenCapture {
    /// Take a capture of the whole screen (or screens)
    fn all() -> anyhow::Result<Image>;
}
