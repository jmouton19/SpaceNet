use indexmap::IndexMap;

pub type OrderedMapPairs = IndexMap<String, (f64, f64)>;
pub type OrderedMapPolygon = IndexMap<String, Vec<(f64, f64)>>;

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
