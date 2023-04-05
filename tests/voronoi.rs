#[cfg(test)]
mod tests {
    use space_net::types::SiteIdList;
    use space_net::utils::Voronoi;
    use std::collections::HashSet;

    fn test_voronoi() -> Voronoi {
        let site = (50.0, 50.0);
        let mut neighbours = SiteIdList::new();
        neighbours.insert("A".to_string(), (75., 75.));
        neighbours.insert("B".to_string(), (25., 25.));
        neighbours.insert("C".to_string(), (66.6, 66.6));
        neighbours.insert("D".to_string(), (33.3, 33.3));
        Voronoi::new(site, &neighbours)
    }

    #[test]
    fn test_voronoi_polygon() {
        //this might fail on other Pcs?
        let voronoi = test_voronoi();
        let expected = vec![
            [-0., 100.00000000000001],
            [0., 83.3],
            [83.29999999999998, -0.],
            [100., -0.],
            [100., 16.600000000000048],
            [16.60000000000001, 100.00000000000001],
        ];
        let actual = voronoi.diagram.cells()[0].points();

        for (actual_point, expected_point) in actual.iter().zip(expected.iter()) {
            if actual_point.x != expected_point[0] || actual_point.y != expected_point[1] {
                panic!("Polygon points do not match expected points");
            }
        }
    }

    #[test]
    fn test_voronoi_neighbours() {
        let expected: HashSet<_> = vec!["D".to_string(), "C".to_string()]
            .iter()
            .cloned()
            .collect();
        let actual: HashSet<_> = test_voronoi().get_neighbours().keys().cloned().collect();
        assert_eq!(actual, expected, "Neighbours dont match")
    }
}
