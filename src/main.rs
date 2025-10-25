use gltch::GlitchApp;

fn main() -> iced::Result {
    env_logger::init();
    iced::application(GlitchApp::default, GlitchApp::update, GlitchApp::view).run()
}
