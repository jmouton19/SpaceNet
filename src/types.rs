use indexmap::IndexMap;

/// HashMap with ordered keys (keeps original insertion order) for site id and site coordinates
pub type OrderedMapPairs = IndexMap<String, (f64, f64)>;
/// HashMap with ordered keys (keeps original insertion order) for site id and its polygon
pub type OrderedMapPolygon = IndexMap<String, Vec<(f64, f64)>>;

/// Calculate the closest site to a given site
pub fn closest_point(pairs: &OrderedMapPairs, site: (f64, f64)) -> ((f64, f64), String) {
    let mut closest_zid = "";
    let mut closest_site = (-1.0, -1.0);
    let mut min_distance = f64::INFINITY;

    for (zid, map_point) in pairs.iter() {
        let distance = ((map_point.0 - site.0).powi(2) + (map_point.1 - site.1).powi(2)).sqrt();
        if distance < min_distance {
            min_distance = distance;
            closest_zid = zid;
            closest_site = *map_point;
        }
    }
    (closest_site, closest_zid.to_string())
}

pub fn point_within_distance(cluster: &OrderedMapPairs, point: (f64, f64), dist: f64) -> bool {
    // Loop through each point in the cluster
    for p in cluster.values() {
        // Calculate the Euclidean distance between the two points
        let dx = p.0 - point.0;
        let dy = p.1 - point.1;
        let distance = (dx * dx + dy * dy).sqrt();
        // If the distance is less than `dist`, the point is within tolerance
        if distance < dist {
            return true;
        }
    }
    // If no points were within `dist`, the point is not within tolerance
    false
}
