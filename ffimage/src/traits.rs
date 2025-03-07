use crate::error::Error;

/// Generic pixel container
pub trait Pixel {
    /// Type of the container elements
    type T;

    /// Number of channels for this pixel
    fn channels() -> u8;

    /// Number of image pixels for this pixel
    fn subpixels() -> u8;
}

/// View into an image, provides read-only pixel access
pub trait GenericImageView {
    /// Pixel type
    type T: Pixel;

    /// Width in pixels
    fn width(&self) -> u32;

    /// Height in pixels
    fn height(&self) -> u32;

    /// Returns the pixel at the specified coordinates
    fn pixel(&self, x: u32, y: u32) -> Option<Self::T>;
}

/// Buffered image, provides read-write pixel access
pub trait GenericImage: GenericImageView {
    /// Sets the pixel values at the specified coordinates
    fn set_pixel(&mut self, x: u32, y: u32, pix: &Self::T) -> Result<(), Error>;
}
