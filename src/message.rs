use crate::types::OrderedMapPairs;
use serde::{Deserialize, Serialize};
pub use serde_json::json;

#[derive(Deserialize, Serialize)]
pub struct DefaultMessage {
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewNodeResponse {
    pub new_site: (f64, f64),
    pub land_owner: String,
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NeighboursNeighboursRequest {
    pub sender_id: String,
    pub new_zid: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewVoronoiRequest {
    pub site: (f64, f64),
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NeighboursResponse {
    #[serde(with = "indexmap::serde_seq")]
    pub neighbours: OrderedMapPairs,
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewVoronoiResponse {
    pub polygon: Vec<(f64, f64)>,
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExpectedNodes {
    pub number: i32,
    pub sender_id: String,
}
