use std::path::PathBuf;

use iced::{
    Element, Length,
    widget::{
        self, button,
        image::{Handle, viewer},
        text,
    },
};
use image::GenericImageView;
use log::{debug, error, info};

#[derive(Debug, Default)]
pub struct GlitchApp {
    current_image: Option<((), Handle)>,
}

#[derive(Debug, Clone)]
pub enum GlitchMessage {
    LoadImage(PathBuf),
    None,
}

impl GlitchApp {
    fn load_image(&mut self, path: PathBuf) {
        info!("Loading image from path: {:?}", path);
        if let Ok(img) = image::open(&path) {
            let dims = img.dimensions();
            let color = img.color();
            let raw = img.into_rgba8().into_vec();
            let handle = Handle::from_rgba(dims.0, dims.1, raw);
            info!(
                "Loaded image from path: {:?} -> ({:?}, {:?})",
                path, dims, color,
            );
            self.current_image = Some(((), handle));
        } else {
            error!("Failed to load image from path: {:?}", path);
        }
    }
    /// Update the application state based on the received message.
    pub fn update(&mut self, msg: GlitchMessage) {
        debug!("Received message: {:?}", msg);
        match msg {
            GlitchMessage::LoadImage(path) => {
                self.load_image(path);
            }
            GlitchMessage::None => {}
        }
    }

    /// View the current state of the application as an Iced Element.
    pub fn view(&self) -> Element<'_, GlitchMessage> {
        if let Some((_, handle)) = &self.current_image {
            // Display the image using iced widgets
            println!("Displaying image with handle: {:?}", handle);

            viewer(handle.clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
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
            GlitchMessage::None
        }
    }
}
