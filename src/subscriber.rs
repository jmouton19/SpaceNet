use crate::node::{config, Node, SyncResolve};
use bincode::deserialize;
use std::sync::Arc;
use zenoh::prelude::r#async::AsyncResolve;

use crate::payload_message::PayloadMessage;
use crate::utils::check_reserved_topics;
use zenoh::Session;

#[derive(Clone)]
pub struct Subscriber {
    pub(crate) session: Arc<Session>,
    pub(crate) tx: flume::Sender<PayloadMessage>,
    pub(crate) rx: flume::Receiver<PayloadMessage>,
}

impl Subscriber {
    pub fn new() -> Self {
        let (tx, rx) = flume::unbounded();
        let session = zenoh::open(config::default())
            .res_sync()
            .unwrap()
            .into_arc();
        Self { session, tx, rx }
    }

    pub fn subscribe(&self, topic: &str) {
        let topic = topic.to_string();

        // List of reserved words to check for
        if check_reserved_topics(topic.as_str()) {
            return;
        }

        let tx = self.tx.clone();
        let session = self.session.clone();
        async_std::task::spawn(async move {
            let subscriber = session
                .declare_subscriber(topic) //removed zid,user must specify in topic
                .with(flume::unbounded())
                .reliable()
                .res_async()
                .await
                .unwrap();
            while let Ok(sample) = subscriber.recv_async().await {
                let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                let data: PayloadMessage = deserialize(payload).unwrap();
                println!(
                    "Received message from {:?} on topic {:?}",
                    data.sender_id, data.topic
                );
                tx.send(data).unwrap();
            }
        });
    }

    //maybe point make option, null if empty
    pub fn receive(&self) -> PayloadMessage {
        let mut payload = PayloadMessage::new();
        if let Ok(message) = self.rx.try_recv() {
            payload = message;
        };
        payload
    }
}
