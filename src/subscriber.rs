use crate::message::PayloadMessage;
use crate::node::Node;
use bincode::deserialize;
use std::sync::Arc;
use zenoh::prelude::r#async::AsyncResolve;

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
    pub fn new(node: &Node) -> Self {
        let (tx, rx) = flume::unbounded();
        let session = node.session.clone();
        let zid = node.get_zid();
        let cluster_name = node.cluster_name.clone();
        Self {
            session,
            //message_queue: vec![],
            cluster_name,
            zid,
            tx,
            rx,
        }
    }

    pub fn subscribe(&self, topic: &str) {
        let _cluster_name = self.cluster_name.clone();
        let topic = topic.to_string();

        //todo!: CHECK FOR RESERVED TOPICS
        //CHANGE START TO SPACENET/**
        //cant be clustername/node/zid/*
        //cant be clustername/bootnode/zid/*
        //clustername/counter/*
        //boot/join

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
                tx.send(data.payload).unwrap();
            }
        });
    }

    //maybe point make option, null if empty
    pub fn receive(&self) -> Vec<u8> {
        let mut payload: Vec<u8> = vec![];
        if let Ok(message) = self.rx.try_recv() {
            payload = message;
        };
        payload
    }
}
