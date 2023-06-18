use crate::types::OrderedMapPairs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct PlayerMoveEventMessage {
    pub(crate) start: (f64, f64),
    pub(crate) end: (f64, f64),
    pub(crate) sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DefaultMessage {
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewNodeResponse {
    pub new_site: (f64, f64),
    pub new_id: String,
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
    pub neighbours: OrderedMapPairs,
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct OwnerResponse {
    pub neighbours: OrderedMapPairs,
    pub new_site: (f64, f64),
    pub sender_site: (f64, f64),
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct NewVoronoiResponse {
    pub polygon: Vec<(f64, f64)>,
    pub site: (f64, f64),
    pub sender_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExpectedNodes {
    pub number: i32,
    pub sender_id: String,
}
