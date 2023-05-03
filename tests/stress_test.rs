#[cfg(test)]
mod stress_test {
    use nalgebra::Point2;
    use space_net::boot_node::BootNode;
    use space_net::node::Node;
    use std::io::Write;
    use std::time::{Instant, SystemTime};
    use std::{fs, thread};

    //check if distributed polygons is correct in an X(expected_len) node cluster
    #[test]
    #[ignore]
    fn stress_test_x_node_cluster() {
        let expected_len = 90;
        // let current_time = SystemTime::now();
        // let file_name = format!("{}.log", current_time.format("%H__%M__%S").as_str());
        let file_name = "test2".to_string();
        let file_name2 = file_name.clone();

        let start_time = Instant::now();
        let mut boot_server = BootNode::new(file_name.as_str());
        let handle1 = thread::spawn(move || loop {
            boot_server.run();
            if boot_server.draw_count == expected_len {
                assert_eq!(boot_server.polygon_list.len() as i32, expected_len);
                assert_eq!(
                    boot_server.polygon_list.len() as i32,
                    boot_server.correct_polygon_list.len() as i32
                );

                let tolerance = 0.001;
                for i in 0..(expected_len) {
                    let n_zid = boot_server.cluster.keys().nth(i as usize).unwrap();
                    let actual = boot_server.polygon_list.get(n_zid).unwrap();
                    let expected = boot_server.correct_polygon_list.get(n_zid).unwrap();
                    // if actual.len() != expected.len() {
                    //     println!("Actual: {:?}", actual);
                    //     println!("Expected: {:?}", expected);
                    //     println!("Counter: {}", i);
                    //     panic!("Polygon lengths do not match");
                    // }
                    for actual_point in actual {
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
                            for (_, site) in boot_server.cluster.iter() {
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
            let mut node = Node::new(file_name2.clone().as_str());
            let _ = thread::spawn(move || loop {
                node.run();
            });
        }
        handle1.join().unwrap();
    }
}
