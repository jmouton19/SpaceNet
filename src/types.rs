use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;

pub type OrderedMapPairs = LinkedHashMap<String, (f64, f64)>;
pub type OrderedMapPolygon = LinkedHashMap<String, Vec<(f64, f64)>>;
pub type SiteIdList= HashMap<String, (f64, f64)>;

//
// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct SiteIdList {
//     pub sites: HashMap<String, (f64, f64)>,
// }
//
// impl SiteIdList {
//     pub fn new() -> SiteIdList {
//         SiteIdList {
//             sites: HashMap::new(),
//         }
//     }
//     // pub fn closest_point(&mut self, site: (f64, f64)) -> String {
//     //     let mut closest_zid = "";
//     //     let mut min_distance = f64::INFINITY;
//     //
//     //     for (zid, map_point) in self.sites.iter() {
//     //         let distance = ((map_point.0 - site.0).powi(2) + (map_point.1 - site.1).powi(2)).sqrt();
//     //         if distance < min_distance {
//     //             min_distance = distance;
//     //             closest_zid = zid;
//     //         }
//     //     }
//     //
//     //     closest_zid.to_string()
//     // }
//     //
//     // pub fn contains(&mut self, site: (f64, f64)) -> bool {
//     //     self.sites.values().any(|v| *v == site)
//     // }
// }
//
// impl Default for SiteIdList {
//     fn default() -> Self {
//         Self::new()
//     }
// }

pub fn closest_point(pairs: &OrderedMapPairs, site: (f64, f64)) -> String {
    let mut closest_zid = "";
    let mut min_distance = f64::INFINITY;

    for (zid, map_point) in pairs.iter() {
        let distance = ((map_point.0 - site.0).powi(2) + (map_point.1 - site.1).powi(2)).sqrt();
        if distance < min_distance {
            min_distance = distance;
            closest_zid = zid;
        }
    }

    closest_zid.to_string()
}
