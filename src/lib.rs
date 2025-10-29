#![doc = include_str!("../README.md")]
use std::path::PathBuf;

use iced::{
    Element, Length,
    overlay::menu::{self, Menu},
    widget::{self, button, column, image::viewer, row, text},
};

use iced_aw::number_input;
use log::{debug, error, info};

use crate::{
    image::GlitchImage,
    sort::cpu::{CpuSorter, ValueGenerator},
};

pub mod image;
pub mod sort;

/// The main application state for Glitch.
#[derive(Debug, Default)]
pub struct GlitchApp {
    current_image: Option<GlitchImage>,
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
                    let sorted_image = self.sorter.sort(&image.image);
                    image.update_with(sorted_image);
                    info!("Sort operation completed.");
                } else {
                    error!("No image loaded to sort.");
                }
            }
            GlitchMessage::NoOp => {}
        }
    }

    /// View the current state of the application as an Iced Element.
    pub fn view(&self) -> Element<'_, GlitchMessage> {
        if let Some(handle) = &self.current_image {
            widget::column![
                self.toolbar(),
                viewer(handle.handle().clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
            ]
            .into()
        } else {
            widget::column![
                text("No image loaded"),
                button("Load Image...").on_press_with(Self::pick_file)
            ]
            .into()
        }
    }

    fn toolbar(&self) -> Element<'_, GlitchMessage> {
        row![
            button("Load Image...").on_press_with(Self::pick_file),
            column![
                number_input(&self.sorter.config.min_threshold(), 0..255, |n| {
                    GlitchMessage::SetLowerThreshold(n)
                })
                .step(1),
                text("Min Threshold"),
            ],
            column![
                number_input(&self.sorter.config.max_threshold(), 0..255, |n| {
                    GlitchMessage::SetUpperThreshold(n)
                })
                .step(1),
                text("Max Threshold"),
            ],
            button("Sort Image (may take a second)").on_press(GlitchMessage::DoSort),
        ]
        .into()
    }

    fn pick_file() -> GlitchMessage {
        let file = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "gif", "tiff", "tif"])
            .pick_file();
        if let Some(file) = file {
            GlitchMessage::LoadImage(file)
        } else {
            GlitchMessage::NoOp
        }
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
