use gltch::GltchApp;

fn main() -> iced::Result {
    iced::run("GLXTCH", GltchApp::update, GltchApp::view)
}
