use crate::handlers::{boot_callback, counter_callback, node_callback};
use crate::message::DefaultMessage;
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use crate::utils::{draw_voronoi_full, Voronoi};
use async_std::io::ReadExt;
pub use async_std::sync::Arc;
use async_std::{io, task};
use indexmap::IndexMap;
use serde_json::json;
pub use zenoh::prelude::sync::*;
use zenoh::subscriber::Subscriber;

//#[derive(Clone)]
pub struct Node<'a> {
    pub(crate) cluster: String,
    pub(crate) session: Arc<Session>,
    pub(crate) site: (f64, f64),
    pub(crate) neighbours: OrderedMapPairs,
    pub(crate) zid: String,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    pub(crate) running: bool,
    subscription: Subscriber<'a, flume::Receiver<Sample>>,
}

// #[derive(Clone)]
pub struct BootNode<'a> {
    pub node: Node<'a>,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    sub_boot: Subscriber<'a, flume::Receiver<Sample>>,
    sub_counter: Subscriber<'a, flume::Receiver<Sample>>,
    pub cluster: OrderedMapPairs,
    pub polygon_list: OrderedMapPolygon,
    pub correct_polygon_list: OrderedMapPolygon,
    pub draw_count: i32,
}

impl Node<'_> {
    pub fn new(config: Config,cluster:&str) -> Self {
        let session = zenoh::open(config).res().unwrap().into_arc();
        let zid = session.zid().to_string();
        let node_subscription = session
            .declare_subscriber(format!("{}/node/{}/*", cluster,zid))
            .reliable()
            .res()
            .unwrap();
        Self {
            cluster:cluster.to_string(),
            zid,
            session,
            site: (-1., -1.),
            neighbours: OrderedMapPairs::new(),
            received_counter: 0,
            expected_counter: -1,
            running: true,
            subscription: node_subscription,
        }
    }

    pub fn join(& self){
        let message = json!(DefaultMessage {
            sender_id: self.zid.clone(),
        });
        self.session.put(format!("{}/node/boot/new",self.cluster), message).res().unwrap();
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

    pub fn leave_on_pressed(self, key: char) -> Self {
        let session = self.session.clone();
        let zid = self.zid.clone();
        let cluster=self.cluster.clone();
        task::spawn(async move {
            let mut buffer = [0; 1];
            loop {
                // Read a single byte from stdin
                if let Ok(()) = io::stdin().read_exact(&mut buffer).await {
                    if buffer[0] == key as u8 {
                        // Call the function when the user presses 'q'
                        let message = json!(DefaultMessage { sender_id: zid });
                        session
                            .put(format!("{}/node/boot/leave_request",cluster), message)
                            .res()
                            .unwrap();
                        break;
                    }
                }
            }
        });
        self
    }

    pub fn get_zid(& self) -> &str {
        self.zid.as_str()
    }

    pub fn is_running(& self) -> bool {
        self.running
    }
}

impl<'a> BootNode<'a> {
    pub fn new_with_node(mut node: Node<'a>) -> Self {
        let counter_subscriber = node
            .session
            .declare_subscriber(format!("{}/counter/*",node.cluster))
            .reliable()
            .res()
            .unwrap();
        let boot_subscriber = node
            .session
            .declare_subscriber(format!("{}/node/boot/*",node.cluster))
            .reliable()
            .res()
            .unwrap();
        node.site = (50., 50.);
        let mut cluster = OrderedMapPairs::new();
        cluster.insert(node.zid.to_string(), node.site);

        let diagram = Voronoi::new((node.zid.clone(), node.site), &node.neighbours);
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
            self.correct_polygon_list = OrderedMapPolygon::new();
            let hash_map: IndexMap<String, (f64, f64)> = self
                .cluster
                .clone()
                .into_iter()
                .filter(|(k, _)| *k != boot_node.zid.as_str())
                .collect();
            let diagram = Voronoi::new((boot_node.zid.clone(), boot_node.site), &hash_map);
            for (i, cell) in diagram.diagram.cells().iter().enumerate() {
                let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
                let site_id = diagram.input.keys().nth(i).unwrap();
                self.correct_polygon_list
                    .insert(site_id.to_string(), polygon);
            }
            draw_voronoi_full(
                &self.cluster,
                &self.correct_polygon_list,
                format!("confirm{}", self.draw_count).as_str(),
            );
            self.draw_count += 1;
        }
    }
}
