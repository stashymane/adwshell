use crate::app::AppModel;
use clap::Parser;
use relm4::RelmApp;

mod app;
mod classes;
mod components;
mod data;
mod settings;
mod util;
mod window_ext;
mod workers;

const APP_ID: &str = "dev.stashy.adwshell";
const APP_NAME: &str = "adwshell";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pub config_path: Option<String>,

    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    gtk_options: Vec<String>,
}

fn main() {
    let args = Args::parse();
    settings::settings::init(args.config_path);

    let app = RelmApp::new(APP_ID);
    app.with_args(args.gtk_options).run::<AppModel>(());
}
