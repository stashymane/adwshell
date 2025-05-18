use std::time::Duration;

use relm4::{ComponentSender, Worker};

#[derive(Debug)]
pub enum HeartbeatMsg {
    Tick,
}

#[derive(Debug)]
pub struct Heartbeat;

impl Worker for Heartbeat {
    type Init = ();
    type Input = ();
    type Output = HeartbeatMsg;

    fn init(init: Self::Init, sender: ComponentSender<Self>) -> Self {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));
            interval.tick().await;
            loop {
                interval.tick().await;
                sender
                    .output(HeartbeatMsg::Tick)
                    .expect("Failed to send heartbeat message");
            }
        });
        Self
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {}
}
