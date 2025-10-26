//! Image handling for Glitch.
use std::path::{Path, PathBuf};

use iced::widget::image::Handle;
use image::{ColorType, GenericImageView, RgbaImage};
use log::{error, info};

/// A struct representing an image loaded in Glitch.
#[derive(Debug)]

pub struct GlitchImage {
    /// The file path of the image.
    pub file: PathBuf,
    /// The image data.
    pub image: image::RgbaImage,
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
        if img.color() != ColorType::Rgba8 {
            error!(
                "Image at {:?} is not in RGBA8 format, found {:?} instead. Downsampling...",
                file,
                img.color()
            );
        }
        let img = img.to_rgba8();
        // This is gross but iced's image handle API is limited. This might mark as another reason to just use wgpu directly.
        let handle = Handle::from_rgba(width, height, img.as_raw().clone());
        Ok(GlitchImage {
            file,
            image: img,
            handle: Some(handle),
        })
    }

    /// Returns a reference to the Iced image handle.
    pub fn handle(&self) -> &Handle {
        self.handle.as_ref().unwrap()
    }

    /// Updates the image data and refreshes the Iced image handle.
    pub fn update_with(&mut self, img: RgbaImage) {
        let (width, height) = img.dimensions();
        info!(
            "Updating image {} with new dimensions ({}x{})",
            self.file.display(),
            width,
            height
        );
        self.image = img;
        let handle = Handle::from_rgba(width, height, self.image.as_raw().clone());
        self.handle = Some(handle);
    }
}
