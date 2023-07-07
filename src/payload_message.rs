use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PayloadMessage {
    pub(crate) payload: Vec<u8>,
    pub(crate) sender_id: String,
    pub(crate) topic: String,
}

impl PayloadMessage {
    pub fn new() -> Self {
        PayloadMessage {
            payload: vec![],
            sender_id: "".to_string(),
            topic: "".to_string(),
        }
    }

    pub fn get_topic(&self) -> String {
        return self.topic.clone();
    }
    pub fn get_payload(&self) -> Vec<u8> {
        return self.payload.clone();
    }
    pub fn get_sender_id(&self) -> String {
        return self.sender_id.clone();
    }
}
