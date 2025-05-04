use crate::app::AppMsg;
use crate::settings::settings;
use notify::{Event, RecommendedWatcher, Watcher};
use relm4::{ComponentSender, Worker};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

pub struct ConfigWatcher {
    _watcher: Option<RecommendedWatcher>,
}

impl Worker for ConfigWatcher {
    type Init = ();
    type Input = ();
    type Output = AppMsg;

    fn init(init: Self::Init, sender: ComponentSender<Self>) -> Self {
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(
            tx,
            notify::Config::default().with_poll_interval(Duration::from_secs(1)),
        )
        .unwrap(); //TODO what do when config dont exist, create new?

        watcher
            .watch(settings::get_path(), notify::RecursiveMode::NonRecursive)
            .unwrap_or_else(|e| {
                panic!("Failed to watch config: {:?}", e);
            });

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(Ok(Event {
                        kind: notify::event::EventKind::Modify(_),
                        ..
                    })) => {
                        println!("Reloading configuration...");
                        settings::refresh();
                        sender.output(AppMsg::ConfigUpdate).unwrap();
                    }
                    Err(e) => panic!("watch error: {:?}", e),
                    _ => {}
                }
            }
        });

        Self {
            _watcher: Some(watcher),
        }
    }

    fn update(&mut self, _message: Self::Input, sender: ComponentSender<Self>) {}
}
