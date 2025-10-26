#![doc = include_str!("../README.md")]
use std::path::PathBuf;

use iced::{
    Element,
    widget::{self, button, image::viewer, text},
};

use log::{debug, error, info};

use crate::image::GlitchImage;

pub mod image;

/// The main application state for Glitch.
#[derive(Debug, Default)]
pub struct GlitchApp {
    current_image: Option<GlitchImage>,
}

/// Messages that can be sent to the Glitch application.
#[derive(Debug, Clone)]
pub enum GlitchMessage {
    /// Load an image from the specified file path.
    LoadImage(PathBuf),
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
            GlitchMessage::NoOp => {}
        }
    }

    /// View the current state of the application as an Iced Element.
    pub fn view(&self) -> Element<'_, GlitchMessage> {
        if let Some(handle) = &self.current_image {
            viewer(handle.handle().clone()).into()
        } else {
            widget::column![
                text("No image loaded"),
                button("Load Image...").on_press_with(Self::pick_file)
            ]
            .into()
        }
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
