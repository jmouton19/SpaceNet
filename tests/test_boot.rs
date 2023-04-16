#[cfg(test)]
mod boot_node {
    use space_net::boot_node::BootNode;

    //test get_zid
    #[test]
    fn test_get_zid() {
        let node = BootNode::new("boot_test1");
        assert_ne!(node.get_zid(), "default-zid");
        assert_eq!(node.get_zid().len(), 32);
    }
}
