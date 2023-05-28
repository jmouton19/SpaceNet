#[cfg(test)]
mod stress_test {
    use nalgebra::Point2;
    use space_net::boot_node::BootNode;
    use space_net::node::Node;
    use std::io::Write;
    use std::time::Instant;
    use std::{fs, thread};
    use rand::Rng;

    //check if distributed polygons is correct in an X(expected_len) node cluster
    #[test]
    #[ignore]
    fn stress_test_x_node_cluster() {
        let expected_len = 50;
        // let current_time = SystemTime::now();
        // let file_name = format!("{}.log", current_time.format("%H__%M__%S").as_str());
        let file_name = "test2".to_string();
        let file_name2 = file_name.clone();

        let start_time = Instant::now();
        let mut boot_server = BootNode::new(file_name.as_str(), true);
        let handle1 = thread::spawn(move || loop {
            if boot_server.get_draw_count() == expected_len {
                let polygon_list: Vec<(String, Vec<(f64, f64)>)> = boot_server.get_polygon_list();
                let cluster: Vec<(String, (f64, f64))> = boot_server.get_cluster();
                let correct_polygon_list: Vec<(String, Vec<(f64, f64)>)> =
                    boot_server.get_correct_polygon_list();
                assert_eq!(polygon_list.len() as i32, expected_len);
                assert_eq!(polygon_list.len() as i32, correct_polygon_list.len() as i32);

                let tolerance = 0.001;
                for i in 0..(expected_len) {
                    let n_zid = cluster.get(i as usize).unwrap().0.clone();
                    let actual = polygon_list.get(i as usize).unwrap().1.clone();
                    let expected = polygon_list
                        .iter()
                        .find(|(zid, _)| zid == &n_zid)
                        .unwrap()
                        .1
                        .clone();
                    // if actual.len() != expected.len() {
                    //     println!("Actual: {:?}", actual);
                    //     println!("Expected: {:?}", expected);
                    //     println!("Counter: {}", i);
                    //     panic!("Polygon lengths do not match");
                    // }
                    for actual_point in actual.clone() {
                        let point = Point2::new(actual_point.0, actual_point.1);
                        if !expected
                            .iter()
                            .map(|p| Point2::new(p.0, p.1))
                            .any(|p| (p - point).norm() <= tolerance)
                        {
                            println!("Actual: {:?}", actual);
                            println!("Expected: {:?}", expected);
                            println!("Counter: {}", i);

                            //save to log text file the sites in the cluster save in in documents/SpaceNet/logs
                            let mut images_path =
                                dirs::document_dir().unwrap().join("SpaceNet").join("logs");
                            fs::create_dir_all(&images_path).unwrap();
                            images_path.push(format!("{}.log", file_name.clone()));
                            let mut file = fs::File::create(images_path).unwrap();
                            for (_, site) in boot_server.get_cluster() {
                                let _ = writeln!(file, "{:?}", site);
                            }

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

        for _i in 0..(expected_len) {
            let mut rng = rand::thread_rng();
            let point = (rng.gen_range(1.0..=99.0), rng.gen_range(1.0..=99.0));
            let mut node = Node::new("test1",point);
            node.join();
        }
        handle1.join().unwrap();
    }
}
