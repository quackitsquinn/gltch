//! UI Components for Glitch

use iced::{
    Element, Length,
    widget::{button, column, row, text},
};
use iced_aw::number_input;

use crate::{GlitchApp, GlitchMessage};

pub fn main_view<'a>(app: &'a GlitchApp) -> Element<'a, GlitchMessage> {
    column![toolbar(app), image_display(app)].into()
}

fn image_display<'a>(app: &'a GlitchApp) -> Element<'a, GlitchMessage> {
    if let Some(handle) = app.display_image() {
        iced::widget::image::viewer(handle.clone())
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    } else {
        text("No image loaded")
            .size(24)
            .width(Length::Fill)
            .height(Length::Fill)
            .center()
            .into()
    }
}

fn toolbar<'a>(app: &'a GlitchApp) -> Element<'a, GlitchMessage> {
    row![
        button("Load Image...").on_press_with(pick_file),
        column![
            number_input(&app.sorter.config.min_threshold(), 0..255, |n| {
                GlitchMessage::SetLowerThreshold(n)
            })
            .step(1),
            text("Min Threshold"),
        ],
        column![
            number_input(&app.sorter.config.max_threshold(), 0..255, |n| {
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
