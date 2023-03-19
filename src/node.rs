use std::sync::Arc;
use voronator::delaunator::Point;
pub use zenoh::prelude::sync::*;
use serde::{Deserialize,Serialize};


#[derive(Clone,Debug,Deserialize,Serialize)]
pub struct SiteIdPairs{
    pub sites:Vec<(f64,f64)>,
    pub ids:Vec<ZenohId>,
}

#[derive(Clone)]
pub struct Node{
    pub session:Arc<Session>,
    pub site:(f64, f64),
    pub neighbours:SiteIdPairs,
}

impl Node{
    pub fn new(config:Config)-> Self{
        Self{
            session:zenoh::open(Config::default()).res().unwrap().into_arc(),
            site:(-1.,-1.),
            neighbours:SiteIdPairs{sites:vec![],ids:vec![]},
        }
    }

    pub fn push_pair_list(&mut self, list:SiteIdPairs){
        self.neighbours.sites.extend(list.sites);
        self.neighbours.ids.extend(list.ids);

    }

}

impl SiteIdPairs{
    pub fn push_pair(&mut self, site:(f64, f64), zid:ZenohId){
        self.sites.push(site);
        self.ids.push(zid);
    }

    pub fn closest_point(&mut self, site:(f64, f64)) -> usize {
        let mut closest_idx = 0;
        let mut closest_dist = f64::INFINITY;

        for (i, point) in self.sites.iter().enumerate() {
            let dist = ((point.0 - site.0).powi(2) + (point.1 - site.1).powi(2)).sqrt();
            if dist < closest_dist {
                closest_idx = i;
                closest_dist = dist;
            }
        }
        closest_idx
    }

}