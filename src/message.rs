use serde::{Deserialize,Serialize};
pub use serde_json::json;
use crate::node::{SiteIdPairs, ZenohId};


#[derive(Deserialize,Serialize)]
pub struct NewNodeRequest{
    pub sender_id: ZenohId,
}
#[derive(Deserialize,Serialize)]
pub struct NewNodeResponse{
    pub site:(f64,f64),
    pub land_owner: ZenohId,
    pub land_owner_site: (f64,f64),
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursRequest{
    pub site:(f64,f64),
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct NewVoronoiRequest{
    pub new_site:(f64,f64),
    pub new_zid:ZenohId,
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursResponse{
    pub neighbours: SiteIdPairs,
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct NewVoronoiResponse{
    pub success: bool,
    pub sender_id: ZenohId,
}

#[derive(Deserialize,Serialize)]
pub struct ExpectedNodes{
    pub number: usize,
    pub sender_id: ZenohId,
}