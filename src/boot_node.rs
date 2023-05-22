use crate::handlers::boot::handle_join_request::handle_join_request;
use crate::handlers::boot::handle_leave_request::handle_leave_request;
use crate::handlers::boot::handle_task_completed::handle_task_completed;
use crate::handlers::boot::set_expected_counter::set_expected_counter;
use crate::node::SyncResolve;
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use crate::utils::{draw_voronoi_full, Voronoi};
use libc::sleep;
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub use zenoh::prelude::sync::*;
use zenoh::prelude::Sample;
use zenoh::subscriber::Subscriber;

/// A boot node in a network acts as the entry point for new nodes to join the cluster. It also acts as a central point for nodes to leave the cluster. Stores the site and polygon information of all nodes in the network. Constructs the distributed voronoi diagram from received polygons as well as a centralized voronoi diagram.
pub struct BootNode {
    pub(crate) session: Arc<Session>,
    pub(crate) boot_node_data: Arc<Mutex<BootNodeData>>,
    // sub_boot: Subscriber<'a, flume::Receiver<Sample>>,
    // sub_counter: Subscriber<'a, flume::Receiver<Sample>>,
}

pub struct BootNodeData {
    pub(crate) zid: String,
    pub(crate) cluster_name: String,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    pub cluster: OrderedMapPairs,
    pub polygon_list: OrderedMapPolygon,
    pub correct_polygon_list: OrderedMapPolygon,
    pub draw_count: i32,
    pub(crate) centralized_voronoi: bool,
}

impl BootNode {
    pub fn new(cluster_name: &str, centralized_voronoi: bool) -> Self {
        let session = zenoh::open(Config::default()).res().unwrap().into_arc();
        let zid = session.zid().to_string();

        let boot_node_data = Arc::new(Mutex::new(BootNodeData::new(
            cluster_name,
            centralized_voronoi,
            zid,
        )));
        Self {
            session,
            boot_node_data,
        }
    }

    /// Process the current messages that are in the subscription channel queue one at a time. Handles each topic with a different [handler](crate::handlers::boot).
    pub fn start(&mut self) {
        let boot_node_data_clone = Arc::clone(&self.boot_node_data);
        let session_clone = Arc::clone(&self.session);

        thread::spawn(move || {
            let guard = boot_node_data_clone.lock().unwrap();
            let counter_subscriber = session_clone
                .declare_subscriber(format!("{}/counter/*", guard.cluster_name))
                .reliable()
                .res()
                .unwrap();
            let boot_subscriber = session_clone
                .declare_subscriber(format!("{}/node/boot/*", guard.cluster_name))
                .reliable()
                .res()
                .unwrap();
            drop(guard);

            loop {
                while let Ok(sample) = boot_subscriber.try_recv() {
                    let mut boot_node_data_guard_clone = boot_node_data_clone.lock().unwrap();
                    boot_node_data_guard_clone.expected_counter = -1;
                    boot_node_data_guard_clone.received_counter = 0;

                    let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
                    println!("Message received on topic... {:?}", topic);
                    let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                    let mut rng = rand::thread_rng();
                    let delay = rng.gen_range(1..=10);
                    thread::sleep(Duration::from_millis(delay));
                    match topic {
                        "new" => {
                            handle_join_request(
                                payload,
                                boot_node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        "leave_request" => {
                            handle_leave_request(
                                payload,
                                boot_node_data_guard_clone,
                                session_clone.clone(),
                            );
                        }
                        _ => println!("UNKNOWN BOOT TOPIC"),
                    }
                    let mut expected_counter =
                        boot_node_data_clone.lock().unwrap().expected_counter;
                    let mut received_counter =
                        boot_node_data_clone.lock().unwrap().received_counter;

                    while expected_counter != received_counter {
                        while let Ok(sample) = counter_subscriber.try_recv() {
                            let mut boot_node_data_guard_clone =
                                boot_node_data_clone.lock().unwrap();
                            let topic = sample.key_expr.split('/').nth(2).unwrap_or("");
                            println!("Message received on topic... {:?}", topic);
                            let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                            let mut rng = rand::thread_rng();
                            let delay = rng.gen_range(1..=10);
                            thread::sleep(Duration::from_millis(delay));
                            match topic {
                                "expected_wait" => {
                                    set_expected_counter(
                                        payload,
                                        &mut boot_node_data_guard_clone.expected_counter,
                                    );
                                }
                                "complete" => {
                                    handle_task_completed(payload, boot_node_data_guard_clone);
                                }
                                _ => println!("UNKNOWN COUNTER TOPIC"),
                            }
                        }
                        expected_counter = boot_node_data_clone.lock().unwrap().expected_counter;
                        received_counter = boot_node_data_clone.lock().unwrap().received_counter;
                    }

                    let mut guard = boot_node_data_clone.lock().unwrap();
                    if !guard.cluster.is_empty() {
                        //redraw distributed voronoi
                        draw_voronoi_full(
                            &guard.cluster,
                            &guard.polygon_list,
                            format!("{}voronoi", guard.draw_count).as_str(),
                        );
                        if guard.centralized_voronoi {
                            //correct voronoi
                            let mut hash_map = guard.cluster.clone();
                            guard.correct_polygon_list = OrderedMapPolygon::new();
                            let (first_zid, first_site) = hash_map
                                .iter()
                                .next()
                                .map(|(k, v)| (k.clone(), *v))
                                .unwrap();
                            hash_map.swap_remove_index(0);

                            let diagram = Voronoi::new((first_zid, first_site), &hash_map);

                            for (i, cell) in diagram.diagram.cells().iter().enumerate() {
                                let polygon = cell.points().iter().map(|x| (x.x, x.y)).collect();
                                let site_id = diagram.input.keys().nth(i).unwrap();
                                guard
                                    .correct_polygon_list
                                    .insert(site_id.to_string(), polygon);
                            }
                            draw_voronoi_full(
                                &guard.cluster,
                                &guard.correct_polygon_list,
                                format!("{}confirm", guard.draw_count).as_str(),
                            );
                        }
                        guard.draw_count += 1;
                        drop(guard);
                    };
                }
            }
        });
    }

    /// Get the zid of the node
    pub fn get_zid(&self) -> String {
        self.boot_node_data.lock().unwrap().zid.clone()
    }
}

impl BootNodeData {
    /// Creates a new boot node instance with a [session](https://docs.rs/zenoh/0.7.0-rc/zenoh/struct.Session.html).
    /// Opens a subscription on topic `{cluster}/boot/*` to receive incoming messages from nodes and a subscription on`{cluster}/counter/*` to count the number of messages its received since processing the current message on the `{cluster}/boot/*` topic.
    pub fn new(cluster_name: &str, centralized_voronoi: bool, zid: String) -> Self {
        Self {
            zid,
            received_counter: 0,
            expected_counter: -1,
            cluster: OrderedMapPairs::new(),
            polygon_list: OrderedMapPolygon::new(),
            correct_polygon_list: OrderedMapPolygon::new(),
            centralized_voronoi,
            draw_count: 0,
            cluster_name: cluster_name.to_string(),
        }
    }
}
