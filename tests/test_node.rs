#[cfg(test)]
mod node {
    use nalgebra::point;
    use space_net::node::{Node, NodeStatus};

    //test get_zid
    #[test]
    fn test_get_zid() {
        let node = Node::new("node_test1");
        assert_ne!(node.get_zid(), "default-zid");
        assert_eq!(node.get_zid().len(), 32);
    }

    //test get_neighbours
    #[test]
    fn test_get_neighbours() {
        let node = Node::new("node_test2");
        assert_eq!(node.get_neighbours().len(), 0);
    }

    //test get_polygon
    #[test]
    fn test_get_polygon() {
        let node = Node::new("node_test3");
        assert_eq!(node.get_polygon().len(), 0);
    }

    //test is in polygon
    #[test]
    fn test_is_in_polygon() {
        let node = Node::new("node_test4");
        assert!(!node.is_in_polygon((0.0, 0.0)));
    }

    //test get_status
    #[test]
    fn test_get_status() {
        let node = Node::new("node_test5");
        assert_eq!(node.get_status(), NodeStatus::Offline);
    }
}
