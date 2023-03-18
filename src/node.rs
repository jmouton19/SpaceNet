use std::sync::Arc;
pub use zenoh::prelude::sync::*;


#[derive(Clone)]
pub struct SiteIdPairs{
    pub sites:Vec<(f64,f64)>,
    pub ids:Vec<String>,
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

}