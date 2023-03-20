use serde::{Deserialize,Serialize};
pub use serde_json::json;
use voronator::delaunator::Point;
use voronator::polygon::Polygon;
use crate::node::SiteIdList;


#[derive(Deserialize,Serialize)]
pub struct NewNodeRequest{
    pub sender_id: String,
}
#[derive(Deserialize,Serialize)]
pub struct NewNodeResponse{
    pub site:(f64,f64),
    pub land_owner: String,
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursRequest{
    pub site:(f64,f64),
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NewVoronoiRequest{
    pub new_site:(f64,f64),
    pub new_zid:String,
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursResponse{
    pub neighbours: SiteIdList,
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NewVoronoiResponse{
    pub polygon:Vec<(f64,f64)>,
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct ExpectedNodes{
    pub number: usize,
    pub sender_id: String,
}