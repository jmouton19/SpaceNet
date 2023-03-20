use std::sync::Arc;
use voronator::delaunator::Point;
pub use zenoh::prelude::sync::*;
use serde::{Deserialize,Serialize};
use std::collections::{HashMap, HashSet};

// pub type SiteIdList=HashMap<String,(f64,f64)>;


#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct SiteIdList{
   pub sites:HashMap<String,(f64,f64)>
}


#[derive(Clone)]
pub struct Node{
    pub session:Arc<Session>,
    pub site:(f64, f64),
    pub neighbours: SiteIdList,
    pub zid:String,
}


impl Node{
    pub fn new(config:Config)-> Self{
        let session=zenoh::open(Config::default()).res().unwrap().into_arc();
        Self{
            zid:session.zid().to_string(),
            session,
            site:(-1.,-1.),
            neighbours:SiteIdList::new(),
        }
    }

    pub fn push_pair_list(&mut self, list:SiteIdList){
        self.neighbours.sites.extend(list.sites);

    }

}


impl SiteIdList{
    pub fn new() -> SiteIdList {
        SiteIdList { sites: HashMap::new() }
    }
    pub fn push_pair(&mut self, site:(f64, f64), zid:String){
        self.sites.insert(zid,site);
    }

    pub fn closest_point(&mut self, site:(f64, f64)) -> String {
        let mut closest_zid = "";
        let mut min_distance = f64::INFINITY;

        for (zid, map_point) in self.sites.iter() {
            let distance = ((map_point.0 - site.0).powi(2) + (map_point.1 - site.1).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
                closest_zid = zid;
            }
        }

        closest_zid.to_string()
    }

    pub fn contains(&mut self, site:(f64, f64)) -> bool {
        self.sites.values().any(|v| *v == site)
    }

}