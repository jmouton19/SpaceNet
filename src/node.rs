use crate::handlers::{boot_callback, counter_callback, node_callback};
use crate::message::NewNodeRequest;
use crate::utils::{draw_voronoi_full, Voronoi};
use linked_hash_map::LinkedHashMap;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
pub use zenoh::prelude::sync::*;
use zenoh::subscriber::Subscriber;

pub type OrderedMapPairs = LinkedHashMap<String, (f64, f64)>;
pub type OrderedMapPolygon = LinkedHashMap<String, Vec<(f64, f64)>>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteIdList {
    pub sites: HashMap<String, (f64, f64)>,
}

//#[derive(Clone)]
pub struct Node<'a> {
    pub session: Arc<Session>,
    pub site: (f64, f64),
    pub neighbours: SiteIdList,
    pub zid: String,
    pub received_counter: i32,
    pub expected_counter: i32,
    pub running: bool,
    sub: Subscriber<'a, flume::Receiver<Sample>>,
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
        let message = json!(NewNodeRequest {
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
            sub: node_subscription,
        }
    }

    pub fn run(&mut self) {
        while let Ok(sample) = self.sub.try_recv() {
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

            //draw correct voronoi
            let mut temp_cluster = self.cluster.clone();
            temp_cluster.remove(boot_node.zid.as_str());
            let hash_map: HashMap<String, (f64, f64)> = temp_cluster.into_iter().collect();
            let temphash = SiteIdList { sites: hash_map };
            let diagram = Voronoi::new(*self.cluster.values().next().unwrap(), &temphash);
            for (i, cell) in diagram.diagram.cells().iter().enumerate() {
                let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
                self.correct_polygon_list
                    .insert(format!("{i}").to_string(), polygon);
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

impl SiteIdList {
    pub fn new() -> SiteIdList {
        SiteIdList {
            sites: HashMap::new(),
        }
    }

    pub fn closest_point(&mut self, site: (f64, f64)) -> String {
        let mut closest_zid = "";
        let mut min_distance = f64::INFINITY;

        for (zid, map_point) in self.sites.iter() {
            let distance = ((map_point.0 - site.0).powi(2) + (map_point.1 - site.1).powi(2)).sqrt();
            if distance < min_distance {
                min_distance = distance;
                closest_zid = zid;
            }
        }

        closest_zid.to_string()
    }

    pub fn contains(&mut self, site: (f64, f64)) -> bool {
        self.sites.values().any(|v| *v == site)
    }
}

impl Default for SiteIdList {
    fn default() -> Self {
        Self::new()
    }
}
