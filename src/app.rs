use crate::components::clock_widget::ClockWidget;
use crate::data::Settings;
use crate::settings::settings;
use crate::window_ext::WindowExt;
use crate::workers::config_watcher::ConfigWatcher;
use crate::{classes, APP_NAME};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use gtk::{Align, Orientation};
use gtk4_layer_shell::LayerShell;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, WorkerController,
};
use std::convert::identity;
use std::process::Command;

#[tracker::track]
pub struct AppModel {
    pub settings: Settings,
    pub language: String,
    #[tracker::do_not_track]
    clock_widget: Controller<ClockWidget>,
    #[tracker::do_not_track]
    config_watcher: WorkerController<ConfigWatcher>,
}

#[derive(Debug)]
pub enum AppMsg {
    OpenLauncher,
    OpenLanguageSwitcher,
    OpenClock,
    ConfigUpdate,
}

//noinspection RsSortImplTraitMembers
#[relm4::component(pub)]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some(APP_NAME),
            #[track = "model.changed(AppModel::settings())"]
            set_css_classes: [classes::SHELL].as_ref(),
            #[track = "model.changed(AppModel::settings())"]
            set_default_height: model.settings.panel.height,
            set_default_width: 400,
            set_decorated: false,
            init_layer_shell: (),
            #[track = "model.changed(AppModel::settings())"]
            apply_layer_shell: model.settings.panel.position,

            gtk::Box {
                set_align: Align::Fill,
                set_orientation: Orientation::Horizontal,
                set_spacing: 16,

                gtk::Button {
                    set_css_classes: [classes::SHELL_BUTTON, classes::APP_LAUNCHER_BUTTON].as_ref(),
                    set_icon_name: "view-app-grid-symbolic",
                    connect_clicked => AppMsg::OpenLauncher,
                },

                gtk::Box { //app list
                },

                gtk::Box { //spacer
                    set_hexpand: true,
                },

                gtk::Button {
                    #[track = "model.changed(AppModel::language())"]
                    set_label: model.language.as_str(),
                    set_css_classes: [classes::SHELL_BUTTON, classes::LANGUAGE_SWITCHER_BUTTON].as_ref(),
                },

                model.clock_widget.widget(),

                gtk::Button { // quick settings

                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            settings: settings::get(),
            language: "en".to_string(),
            clock_widget: ClockWidget::builder().launch(()).detach(),
            tracker: 0,
            config_watcher: ConfigWatcher::builder()
                .detach_worker(())
                .forward(sender.input_sender(), identity),
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        self.reset();

        match message {
            AppMsg::OpenLauncher => {
                Command::new(&self.settings.launcher.on_click)
                    .spawn()
                    .expect("failed to execute process");
            }

            AppMsg::ConfigUpdate => {
                settings::refresh();
                self.set_settings(settings::get());
            }

            _ => {}
        }
    }
}
