use serde::{Deserialize,Serialize};
use serde::ser::SerializeMap;
use serde::de;
pub use serde_json::json;
use voronator::delaunator::Point;
use voronator::polygon::Polygon;
use crate::node::{OrderedMapPairs, SiteIdList};


#[derive(Deserialize,Serialize)]
pub struct NewNodeRequest{
    pub sender_id: String,
}
#[derive(Deserialize,Serialize)]
pub struct NoNeighbours{
    pub site:(f64,f64),
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NewNodeResponse{
    pub new_site:(f64,f64),
    pub land_owner: String,
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursRequest{
    pub new_site:(f64,f64),
    pub sender_id: String,
}

#[derive(Deserialize,Serialize)]
pub struct NeighboursNeighboursRequest{
    pub sender_id: String,
    pub new_zid:String,
}

#[derive(Deserialize,Serialize)]
pub struct NewVoronoiRequest{
    pub new_site:(f64,f64),
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
    pub number: i32,
    pub sender_id: String,
}