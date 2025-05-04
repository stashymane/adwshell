use crate::app::AppModel;
use relm4::RelmApp;

mod app;
mod classes;
mod components;
mod data;
mod settings;
mod window_ext;
mod workers;

const APP_ID: &str = "dev.stashy.adwshell";
const APP_NAME: &str = "adwshell";

fn main() {
    let app = RelmApp::new(APP_ID);
    app.run::<AppModel>(());
}
