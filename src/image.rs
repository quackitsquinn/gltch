//! Image handling for Glitch.
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use bytes::Bytes;
use iced::widget::image::Handle;
use image::{ColorType, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use log::{error, info};

/// Type alias for an image buffer used in Glitch.
pub type GlitchImageBuffer = ImageBuffer<Rgba<u8>, Arc<[u8]>>;

/// A struct representing an image loaded in Glitch.
#[derive(Debug)]
pub struct GlitchImage {
    /// The file path of the image.
    pub file: PathBuf,
    /// The image data.
    pub image: GlitchImageBuffer,
    /// The Iced image handle. This will likely be refactored out later to replace the redundant cloning of image data.
    pub handle: Option<Handle>,
}

impl GlitchImage {
    /// Opens an image from the given file path.
    pub fn open(file: impl AsRef<Path>) -> Result<Self, image::ImageError> {
        let file = file.as_ref().to_path_buf();
        let img = image::open(&file)?;
        let (width, height) = img.dimensions();

        info!(
            "Opening {} with dimensions ({}x{})",
            file.display(),
            width,
            height
        );

        let img = img.to_rgba8();

        let image_data = img.as_raw().clone();
        let arc: Arc<[u8]> = Arc::from(image_data);
        // First, create a new ImageBuffer that uses Arc for its pixel data.
        // This allows us to share the pixel data without unnecessary copies.
        let image_buf: GlitchImageBuffer =
            ImageBuffer::from_raw(width, height, arc.clone()).expect("failed to convert to arc");
        // Now, create the Iced Handle from the raw RGBA data. Thankfully, Bytes::from_owned exists and supports Arc.
        let handle = Self::make_handle(&image_buf);

        Ok(GlitchImage {
            file,
            image: image_buf,
            handle: Some(handle),
        })
    }

    /// Returns a reference to the Iced image handle.
    pub fn handle(&self) -> &Handle {
        self.handle.as_ref().unwrap()
    }

    /// Updates the image data and refreshes the Iced image handle.
    pub fn update_with(&mut self, img: GlitchImageBuffer) {
        let (width, height) = img.dimensions();
        info!(
            "Updating image {} with new dimensions ({}x{})",
            self.file.display(),
            width,
            height
        );

        self.image = img;
        self.handle = Some(Self::make_handle(&self.image));
    }

    fn make_handle(image: &GlitchImageBuffer) -> Handle {
        let (width, height) = image.dimensions();
        let image_data = image.as_raw().clone();
        Handle::from_rgba(width, height, Bytes::from_owner(image_data))
    }
}
