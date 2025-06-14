use crate::components::clock_widget::ClockWidget;
use crate::components::launcher::LauncherWidget;
use crate::settings::settings::Settings;
use crate::window_ext::WindowExt;
use crate::workers::config_watcher::ConfigWatcher;
use crate::{classes, settings, APP_NAME};
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt, WidgetExt};
use gtk::{Align, Orientation};
use gtk4_layer_shell::LayerShell;
use relm4::{
    Component, ComponentController, ComponentParts, ComponentSender, Controller, RelmWidgetExt,
    SimpleComponent, WorkerController,
};
use settings::loader;
use std::convert::identity;

#[tracker::track]
pub struct AppModel {
    pub settings: Settings,
    pub language: String,
    #[tracker::do_not_track]
    launcher_widget: Controller<LauncherWidget>,
    #[tracker::do_not_track]
    clock_widget: Controller<ClockWidget>,
    #[tracker::do_not_track]
    config_watcher: WorkerController<ConfigWatcher>,
}

#[derive(Debug)]
pub enum AppMsg {
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

                model.launcher_widget.widget(),

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
        let settings = loader::get();
        let model = AppModel {
            settings: settings.clone(),
            language: "en".to_string(),
            launcher_widget: LauncherWidget::builder().launch(settings.launcher).detach(),
            clock_widget: ClockWidget::builder().launch(settings.clock).detach(),
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
            AppMsg::ConfigUpdate => {
                loader::refresh();
                self.set_settings(loader::get());
            }

            _ => {}
        }
    }
}
