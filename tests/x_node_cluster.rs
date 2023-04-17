#[cfg(test)]
mod test {
    use nalgebra::Point2;
    use space_net::boot_node::BootNode;
    use space_net::node::Node;
    use std::thread;
    use std::time::Instant;

    //check if distributed polygons is correct in an X(expected_len) node cluster
    #[test]
    fn test_x_node_cluster() {
        let expected_len = 5;

        let start_time = Instant::now();
        let mut boot_server = BootNode::new("test1");
        let handle1 = thread::spawn(move || loop {
            boot_server.run();
            if boot_server.draw_count == expected_len - 1 {
                assert_eq!(boot_server.polygon_list.len() as i32, expected_len - 1);
                assert_eq!(
                    boot_server.polygon_list.len() as i32,
                    boot_server.correct_polygon_list.len() as i32
                );

                let tolerance = 0.001;
                for i in 0..(expected_len - 1) {
                    let n_zid = boot_server.cluster.keys().nth(i as usize).unwrap();
                    let actual = boot_server.polygon_list.get(n_zid).unwrap();
                    let expected = boot_server.correct_polygon_list.get(n_zid).unwrap();
                    if actual.len() != expected.len() {
                        panic!("Polygon lengths do not match");
                    }
                    for actual_point in actual {
                        let point = Point2::new(actual_point.0, actual_point.1);
                        if !expected
                            .iter()
                            .map(|p| Point2::new(p.0, p.1))
                            .any(|p| (p - point).norm() <= tolerance)
                        {
                            panic!(
                                "Actual point {:?} is not within epsilon distance of any expected point.",
                                actual_point
                            );
                        }
                    }
                }
                break;
            }
            let elapsed_time = start_time.elapsed().as_secs();
            if elapsed_time > (10 * expected_len) as u64 {
                panic!("Test took too long to run!");
            }
        });

        for _i in 0..(expected_len - 1) {
            let mut node = Node::new("test1");
            let _ = thread::spawn(move || loop {
                node.run();
            });
        }
        handle1.join().unwrap();
    }
}
