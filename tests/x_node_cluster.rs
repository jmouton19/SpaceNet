#[cfg(test)]
mod integration {
    use space_net::node::*;
    use std::thread;
    use std::time::Instant;

    //check if boot node polygon is correct
    #[test]
    fn test_x_node_cluster() {
        let start_time = Instant::now();
        let mut boot_server = BootNode::new_with_node(Node::new(Config::default()));
        let expected_len = 10;

        let handle1 = thread::spawn(move || loop {
            boot_server.run();
            if boot_server.draw_count == expected_len {
                assert_eq!(boot_server.polygon_list.len() as i32, expected_len);

                let actual = boot_server
                    .polygon_list
                    .get(&*boot_server.node.zid)
                    .unwrap();
                let expected = boot_server.correct_polygon_list.get("0").unwrap();

                println!("actual: {:?}", actual);
                println!("+++++++++++++++++++++++++");
                println!("expected: {:?}", expected);

                let tolerance = 0.01;
                for (actual_point, expected_point) in actual.iter().zip(expected.iter()) {
                    if (actual_point.0 - expected_point.0).abs() > tolerance
                        || (actual_point.1 - expected_point.1).abs() > tolerance
                    {
                        panic!("Polygon points do not match expected points");
                    }
                }
                break;
            }
            let elapsed_time = start_time.elapsed().as_secs();
            if elapsed_time > (10 * expected_len) as u64 {
                panic!("Test took too long to run!");
            }
        });

        for _i in 0..expected_len - 1 {
            let mut node = Node::new(Config::default());
            let _ = thread::spawn(move || loop {
                node.run();
            });
        }
        handle1.join().unwrap();
    }
}
