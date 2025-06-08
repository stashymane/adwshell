use crate::settings::clock::ClockSettings;
use crate::workers::heartbeat::Heartbeat;
use crate::{classes, workers};
use chrono::Utc;
use gtk::prelude::{ButtonExt, WidgetExt};
use relm4::{Component, ComponentParts, ComponentSender, SimpleComponent, WorkerController};

#[tracker::track]
pub struct ClockWidget {
    format: String,
    time: String,
    #[tracker::do_not_track]
    heartbeat: WorkerController<workers::heartbeat::Heartbeat>,
}

#[derive(Debug)]
pub enum ClockWidgetMsg {
    Refresh,
}

//noinspection RsSortImplTraitMembers
#[relm4::component(pub)]
impl SimpleComponent for ClockWidget {
    type Init = ClockSettings;
    type Input = ClockWidgetMsg;
    type Output = ();

    view! {
        gtk::Button {
            #[track = "model.changed(ClockWidget::time())"]
            set_label: &model.time,
            set_css_classes: [classes::SHELL_BUTTON, classes::CLOCK_BUTTON].as_ref()
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ClockWidget {
            format: init.format.clone(),
            time: now(&init.format),
            heartbeat: Heartbeat::builder()
                .detach_worker(())
                .forward(sender.input_sender(), |_msg| ClockWidgetMsg::Refresh),
            tracker: 0,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        self.reset();

        match message {
            ClockWidgetMsg::Refresh => {
                self.set_time(now(&self.format));
            }
        };
    }
}

fn now(format: &str) -> String {
    Utc::now().format(format).to_string()
}
