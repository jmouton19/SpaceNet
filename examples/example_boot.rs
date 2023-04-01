use space_net::handlers::*;
use space_net::node::*;
use space_net::utils::*;
use std::collections::HashMap;

fn main() {
    let mut boot_node = Node::new(Config::default());
    println!("boot node online..... {:?}", boot_node.zid);

    let counter_subscriber = boot_node
        .session
        .declare_subscriber("counter/*")
        .reliable()
        .res()
        .unwrap();
    let boot_subscriber = boot_node
        .session
        .declare_subscriber("node/boot/*")
        .reliable()
        .res()
        .unwrap();
    let node_subscriber = boot_node
        .session
        .declare_subscriber(format!("node/{}/*", boot_node.zid))
        .reliable()
        .res()
        .unwrap();

    //set boot node point
    boot_node.site = (50., 50.);
    //add boot node to cluster
    let mut cluster = OrderedMapPairs::new();
    cluster.insert(boot_node.zid.to_string(), boot_node.site);

    //draw initial voronoi
    let diagram = Voronoi::new(boot_node.site, &boot_node.neighbours);
    let polygon = diagram.diagram.cells()[0]
        .points()
        .iter()
        .map(|x| (x.x, x.y))
        .collect();
    let mut polygon_list = OrderedMapPolygon::new();
    polygon_list.insert(boot_node.zid.to_string(), polygon);
    draw_voronoi_full(&cluster, &polygon_list, "initial");
    let mut draw_count = 1;

    let mut correct_polygon_list = OrderedMapPolygon::new();
    loop {
        // Handle messages in the queue
        if let Ok(sample) = boot_subscriber.try_recv() {
            let mut expected_counter = -1;
            let mut counter = 0;

            boot_callback(sample, &mut boot_node, &mut polygon_list, &mut cluster);
            // Process the message here

            while expected_counter != counter {
                while let Ok(sample) = counter_subscriber.try_recv() {
                    counter_callback(
                        sample,
                        &mut expected_counter,
                        &mut counter,
                        &mut polygon_list,
                    );
                    // Process the message here
                }
                while let Ok(sample) = node_subscriber.try_recv() {
                    node_callback(sample, &mut boot_node);
                    // Process the message here
                }
            }
            //redraw distributed voronoi
            draw_voronoi_full(
                &cluster,
                &polygon_list,
                format!("voronoi{}", draw_count).as_str(),
            );

            //draw correct voronoi
            let mut temp_cluster = cluster.clone();
            temp_cluster.remove(boot_node.zid.as_str());
            let hash_map: HashMap<String, (f64, f64)> = temp_cluster.into_iter().collect();
            let temphash = SiteIdList { sites: hash_map };
            let diagram = Voronoi::new(*cluster.values().next().unwrap(), &temphash);
            for (i, cell) in diagram.diagram.cells().iter().enumerate() {
                let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
                correct_polygon_list.insert(format!("{i}").to_string(), polygon);
            }
            draw_voronoi_full(
                &cluster,
                &correct_polygon_list,
                format!("confirm{}", draw_count).as_str(),
            );
            draw_count += 1;
        }
        // Perform other tasks here
        // Wait for some time before starting to handle messages again
        //sleep(Duration::from_secs(1));
    }
}
