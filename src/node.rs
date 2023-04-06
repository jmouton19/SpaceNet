use crate::handlers::{boot_callback, counter_callback, node_callback};
use crate::message::DefaultMessage;
use crate::types::{OrderedMapPairs, OrderedMapPolygon, SiteIdList};
use crate::utils::{draw_voronoi_full, Voronoi};
use async_std::io::ReadExt;
pub use async_std::sync::Arc;
use async_std::{io, task};
use serde_json::json;
use std::collections::HashMap;
pub use zenoh::prelude::sync::*;
use zenoh::subscriber::Subscriber;

//#[derive(Clone)]
pub struct Node<'a> {
    pub session: Arc<Session>,
    pub site: (f64, f64),
    pub neighbours: SiteIdList,
    pub zid: String,
    pub received_counter: i32,
    pub expected_counter: i32,
    pub running: bool,
    subscription: Subscriber<'a, flume::Receiver<Sample>>,
}

// #[derive(Clone)]
pub struct BootNode<'a> {
    pub node: Node<'a>,
    pub received_counter: i32,
    pub expected_counter: i32,
    pub running: bool,
    sub_boot: Subscriber<'a, flume::Receiver<Sample>>,
    sub_counter: Subscriber<'a, flume::Receiver<Sample>>,
    pub cluster: OrderedMapPairs,
    pub polygon_list: OrderedMapPolygon,
    pub correct_polygon_list: OrderedMapPolygon,
    pub draw_count: i32,
}

impl Node<'_> {
    pub fn new(config: Config) -> Self {
        let session = zenoh::open(config).res().unwrap().into_arc();
        let zid = session.zid().to_string();
        let node_subscription = session
            .declare_subscriber(format!("node/{}/*", zid))
            .reliable()
            .res()
            .unwrap();
        let message = json!(DefaultMessage {
            sender_id: zid.clone(),
        });
        session.put("node/boot/new", message).res().unwrap();
        Self {
            zid,
            session,
            site: (-1., -1.),
            neighbours: SiteIdList::new(),
            received_counter: 0,
            expected_counter: -1,
            running: true,
            subscription: node_subscription,
        }
    }

    pub fn run(&mut self) {
        while let Ok(sample) = self.subscription.try_recv() {
            if !self.running {
                break;
            }
            node_callback(sample, self);
            // Process the message here
        }
    }
}

impl<'a> BootNode<'a> {
    pub fn new_with_node(mut node: Node<'a>) -> Self {
        let counter_subscriber = node
            .session
            .declare_subscriber("counter/*")
            .reliable()
            .res()
            .unwrap();
        let boot_subscriber = node
            .session
            .declare_subscriber("node/boot/*")
            .reliable()
            .res()
            .unwrap();
        node.site = (50., 50.);
        let mut cluster = OrderedMapPairs::new();
        cluster.insert(node.zid.to_string(), node.site);

        let diagram = Voronoi::new(node.site, &node.neighbours);
        let polygon = diagram.diagram.cells()[0]
            .points()
            .iter()
            .map(|x| (x.x, x.y))
            .collect();

        let mut polygon_list = OrderedMapPolygon::new();
        polygon_list.insert(node.zid.to_string(), polygon);
        draw_voronoi_full(&cluster, &polygon_list, "initial");
        Self {
            node,
            received_counter: 0,
            expected_counter: -1,
            running: true,
            sub_boot: boot_subscriber,
            sub_counter: counter_subscriber,
            cluster,
            polygon_list,
            correct_polygon_list: OrderedMapPolygon::new(),
            draw_count: 1,
        }
    }

    // pub fn new_without_node() -> Self {
    //     Self { node: None, received_counter: 0, expected_counter: -1, running: true }
    // }

    pub fn run(&mut self) {
        let boot_node = &mut self.node;

        if let Ok(sample) = self.sub_boot.try_recv() {
            self.expected_counter = -1;
            self.received_counter = 0;

            boot_callback(sample, boot_node, &mut self.polygon_list, &mut self.cluster);
            // Process the message here

            while self.expected_counter != self.received_counter {
                while let Ok(sample) = self.sub_counter.try_recv() {
                    counter_callback(
                        sample,
                        &mut self.expected_counter,
                        &mut self.received_counter,
                        &mut self.polygon_list,
                    );
                    // Process the message here
                }
                boot_node.run();
            }
            //redraw distributed voronoi
            draw_voronoi_full(
                &self.cluster,
                &self.polygon_list,
                format!("voronoi{}", self.draw_count).as_str(),
            );

            //correct voronoi
            let mut temp_cluster = self.cluster.clone();
            temp_cluster.remove(boot_node.zid.as_str());
            let hash_map: HashMap<String, (f64, f64)> = temp_cluster.into_iter().collect();
            let diagram = Voronoi::new(*self.cluster.values().next().unwrap(), &hash_map);
            for (i, cell) in diagram.diagram.cells().iter().enumerate() {
                let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
                self.correct_polygon_list
                    .insert(format!("{i}").to_string(), polygon);
            }
            // draw_voronoi_full(
            //     &self.cluster,
            //     &self.correct_polygon_list,
            //     format!("confirm{}", self.draw_count).as_str(),
            // );
            self.draw_count += 1;
        }
    }
}

pub fn leave_on_pressed(session: Arc<Session>, char: char) {
    task::spawn(async move {
        let mut buffer = [0; 1];
        loop {
            // Read a single byte from stdin
            if let Ok(()) = io::stdin().read_exact(&mut buffer).await {
                if buffer[0] == char as u8 {
                    // Call the function when the user presses 'q'
                    let message = json!(DefaultMessage {
                        sender_id: session.zid().to_string(),
                    });
                    session
                        .put("node/boot/leave_request", message)
                        .res()
                        .unwrap();
                    break;
                }
            }
        }
    });
}
