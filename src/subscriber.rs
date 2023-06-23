use crate::message::PayloadMessage;
use crate::node::Node;
use bincode::deserialize;
use std::sync::Arc;
use zenoh::prelude::r#async::AsyncResolve;
use zenoh::sample::Sample;
use zenoh::Session;

#[derive(Clone)]
pub struct NodeSubscriber {
    pub(crate) session: Arc<Session>,
    //pub(crate) message_queue: Vec<Vec<u8>>,
    pub(crate) cluster_name: String,
    pub zid: String,
    pub(crate) tx: flume::Sender<Vec<u8>>,
    pub(crate) rx: flume::Receiver<Vec<u8>>,
}

impl NodeSubscriber {
    pub fn new(node: &Node) -> Arc<Self> {
        let (tx, rx) = flume::unbounded();
        let session = node.session.clone();
        let zid = node.get_zid();
        let cluster_name = node.cluster_name.clone();
        Arc::new(Self {
            session,
            //message_queue: vec![],
            cluster_name,
            zid,
            tx,
            rx,
        })
    }

    pub fn subscribe(&self, topic: String) {
        let cluster_name = self.cluster_name.clone();
        let zid = self.zid.clone();
        let zid="node1".to_string();

        let tx = self.tx.clone();
        let session = self.session.clone();
        async_std::task::spawn(async move {
            let subscriber = session
                .declare_subscriber(format!("{}/{}/{}", cluster_name, zid, topic))
                .with(flume::unbounded())
                .reliable()
                .res_async()
                .await
                .unwrap();
            while let Ok(sample) = subscriber.recv_async().await {
                let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                let data: PayloadMessage = deserialize(payload).unwrap();
                println!("Received message from {:?} on topic {:?}", data.sender_id,data.topic);
                tx.send(data.payload).unwrap();
            }
        });
    }

    pub fn receive(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![];
        if let Ok(message) = self.rx.try_recv() {
            payload = message;
        };
        payload
    }
}
