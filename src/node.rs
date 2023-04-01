use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
pub use zenoh::prelude::sync::*;

pub type OrderedMapPairs = LinkedHashMap<String, (f64, f64)>;
pub type OrderedMapPolygon = LinkedHashMap<String, Vec<(f64, f64)>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteIdList {
    pub sites: HashMap<String, (f64, f64)>,
}

#[derive(Clone)]
pub struct Node {
    pub session: Arc<Session>,
    pub site: (f64, f64),
    pub neighbours: SiteIdList,
    pub zid: String,
    pub received_counter: i32,
    pub expected_counter: i32,
    pub running:bool,
}

// #[derive(Clone)]
// pub struct BootNode{
//     pub node:Node,
//     pub received_counter:i32,
//     pub expected_counter:i32,
// }

impl Node {
    pub fn new(config: Config) -> Self {
        let session = zenoh::open(config).res().unwrap().into_arc();
        Self {
            zid: session.zid().to_string(),
            session,
            site: (-1., -1.),
            neighbours: SiteIdList::new(),
            received_counter: 0,
            expected_counter: -1,
            running:true,
        }
    }

    pub fn run(&mut self){
        while self.running{

        }
    }
}

// impl BootNode{
//     pub fn new(config:Config)-> Self{
//         Self{
//             node:Node::new(config),
//             received_counter:0,
//             expected_counter:-1,
//         }
//     }
// }

impl SiteIdList {
    pub fn new() -> SiteIdList {
        SiteIdList {
            sites: HashMap::new(),
        }
    }

    pub fn closest_point(&mut self, site: (f64, f64)) -> String {
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

    pub fn contains(&mut self, site: (f64, f64)) -> bool {
        self.sites.values().any(|v| *v == site)
    }
}

impl Default for SiteIdList {
    fn default() -> Self {
        Self::new()
    }
}
