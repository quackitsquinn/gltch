use iced::{Element, widget::text};

#[derive(Debug, Default)]
pub struct GltchApp {}

#[derive(Debug, Clone, Copy)]
pub enum GltchMessage {}

impl GltchApp {
    pub fn update(&mut self, msg: GltchMessage) {}

    pub fn view(&self) -> Element<'_, GltchMessage> {
        Element::from(text("Hello, Gltch!"))
    }
}
