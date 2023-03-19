use serde::{Deserialize,Serialize};
pub use serde_json::json;
use crate::node::{SiteIdPairs, ZenohId};


#[derive(Deserialize,Serialize)]
pub struct NewNodeRequest{
    pub value: String,
    pub sender_id: ZenohId,
}
#[derive(Deserialize,Serialize)]
pub struct NewNodeResponse{
    pub value: String,
    pub site:(f64,f64),
    pub land_owner: ZenohId,
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursRequest{
    pub value: String,
    pub site:(f64,f64),
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursResponse{
    pub value: String,
    pub neighbours: SiteIdPairs,
    pub sender_id: ZenohId,
}