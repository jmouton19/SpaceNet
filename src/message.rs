use serde::{Deserialize,Serialize};
pub use serde_json::json;


#[derive(Deserialize,Serialize)]
pub struct NewNodeRequest{
    pub value: String,
    pub sender_id: String,
}
#[derive(Deserialize,Serialize)]
pub struct NewNodeResponse{
    pub value: String,
    pub site:(f64,f64),
}