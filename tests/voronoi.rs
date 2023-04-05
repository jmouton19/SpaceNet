#[cfg(test)]
mod tests {
    use space_net::types::SiteIdList;
    use space_net::utils::Voronoi;

    #[test]
    fn test_voronoi_new() {
        let site = (0.0, 0.0);
        let neighbours = SiteIdList::new();
        neighbours.sites
        let voronoi = Voronoi::new(site, &neighbours);

        // Verify the length of the sites vector
        assert_eq!(voronoi.length, 1);

        // Verify the site coordinates
        assert_eq!(voronoi.site, site);

        // Verify the neighbours
        assert_eq!(voronoi.neighbours, neighbours);
    }
}