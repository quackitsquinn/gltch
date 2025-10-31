#![doc = include_str!("../README.md")]
use std::path::PathBuf;

use iced::{Element, widget::image::Handle};

use log::{debug, error, info};

use crate::{
    image::{GlitchImage, GlitchImageBuffer},
    sort::cpu::{CpuSorter, ValueGenerator},
};

pub mod image;
pub mod sort;
mod ui;

/// The main application state for Glitch.
#[derive(Debug, Default)]
pub struct GlitchApp {
    current_image: Option<GlitchImage>,
    current_view: Option<(GlitchImageBuffer, Handle)>,
    sorter: CpuSorter<PixelBrightness>,
}

/// Messages that can be sent to the Glitch application.
#[derive(Debug, Clone)]
pub enum GlitchMessage {
    /// Load an image from the specified file path.
    LoadImage(PathBuf),
    /// Set the lower threshold value.
    SetLowerThreshold(u8),
    /// Set the upper threshold value.
    SetUpperThreshold(u8),
    /// Perform the sorting operation.
    DoSort,
    /// No operation.
    NoOp,
}

impl GlitchApp {
    fn load_image(&mut self, path: PathBuf) {
        info!("Loading image from path: {:?}", path);
        match GlitchImage::open(path) {
            Ok(img) => {
                self.current_image = Some(img);
            }
            Err(e) => {
                error!("Failed to load image: {}", e);
            }
        }
    }
    /// Update the application state based on the received message.
    pub fn update(&mut self, msg: GlitchMessage) {
        debug!("Received message: {:?}", msg);

        match msg {
            GlitchMessage::LoadImage(path) => {
                self.load_image(path);
            }
            GlitchMessage::SetLowerThreshold(min) => {
                let max = self.sorter.config.max_threshold();
                self.sorter.config.set_thresholds(min, max);
            }
            GlitchMessage::SetUpperThreshold(max) => {
                let min = self.sorter.config.min_threshold();
                self.sorter.config.set_thresholds(min, max);
            }
            GlitchMessage::DoSort => {
                if let Some(image) = &mut self.current_image {
                    info!("Starting sort operation...");
                    self.current_view = Some(self.sorter.sort(&image.image));
                    info!("Sort operation completed.");
                } else {
                    error!("No image loaded to sort.");
                }
            }
            GlitchMessage::NoOp => {}
        }
    }

    /// Returns the handle of the image currently being displayed.
    pub fn display_image(&self) -> Option<&Handle> {
        if let Some((_, handle)) = &self.current_view {
            Some(handle)
        } else if let Some(img) = &self.current_image {
            Some(img.handle())
        } else {
            error!("No image to display!");
            None
        }
    }

    /// View the current state of the application as an Iced Element.
    pub fn view(&self) -> Element<'_, GlitchMessage> {
        ui::main_view(self)
    }
}

/// Pixel brightness value generator.
pub struct PixelBrightness;

impl ValueGenerator for PixelBrightness {
    fn generate(pixel: [u8; 3]) -> u8 {
        // Calculate brightness using the luminance formula.
        let r = pixel[0] as f32;
        let g = pixel[1] as f32;
        let b = pixel[2] as f32;
        (0.299 * r + 0.587 * g + 0.114 * b) as u8
    }
}

#[cfg(test)]
mod tests {}
