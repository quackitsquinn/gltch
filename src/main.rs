use gltch::GlitchApp;

fn main() -> iced::Result {
    env_logger::init();
    iced::run("GLXTCH", GlitchApp::update, GlitchApp::view)
}
