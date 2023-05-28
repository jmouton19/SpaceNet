#[cfg(test)]
mod test {

    use nalgebra::Point2;
    use space_net::boot_node::BootNode;
    use space_net::node::Node;
    use std::thread;
    use std::time::Instant;
    use rand::Rng;

    //check if distributed polygons is correct in an X(expected_len) node cluster
    #[test]
    fn test_x_node_cluster() {
        let expected_len = 5;

        let start_time = Instant::now();
        let boot_server = BootNode::new("test1", true);
        let handle1 = thread::spawn(move || loop {
            if boot_server.get_draw_count() == expected_len - 1 {
                let polygon_list: Vec<(String, Vec<(f64, f64)>)> = boot_server.get_polygon_list();
                let cluster: Vec<(String, (f64, f64))> = boot_server.get_cluster();
                let correct_polygon_list: Vec<(String, Vec<(f64, f64)>)> =
                    boot_server.get_correct_polygon_list();
                assert_eq!(polygon_list.len() as i32, expected_len - 1);
                assert_eq!(polygon_list.len() as i32, correct_polygon_list.len() as i32);

                let tolerance = 0.001;
                for i in 0..(expected_len - 1) {
                    let n_zid = cluster.get(i as usize).unwrap().0.clone();
                    let actual = polygon_list.get(i as usize).unwrap().1.clone();

                    let expected = polygon_list
                        .iter()
                        .find(|(zid, _)| zid == &n_zid)
                        .unwrap()
                        .1
                        .clone();

                    //let expected = correct_polygon_list.get(i as usize).unwrap().1.clone();
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
            let mut rng = rand::thread_rng();
            let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
            let mut node = Node::new("test1",point);
            node.join();
        }
        handle1.join().unwrap();
    }
}

//todo leave test
