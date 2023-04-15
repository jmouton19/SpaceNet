use crate::handlers::boot::handle_join_request::handle_join_request;
use crate::handlers::boot::handle_leave_request::handle_leave_request;
use crate::handlers::boot::handle_task_completed::handle_task_completed;
use crate::handlers::boot::set_expected_counter::set_expected_counter;
use crate::node::SyncResolve;
use crate::types::{OrderedMapPairs, OrderedMapPolygon};
use crate::utils::{draw_voronoi_full, Voronoi};
use std::sync::Arc;
pub use zenoh::prelude::sync::*;
use zenoh::prelude::Sample;
use zenoh::subscriber::Subscriber;

/// BootNode struct
pub struct BootNode<'a> {
    pub(crate) session: Arc<Session>,
    pub(crate) zid: String,
    pub(crate) cluster_name: String,
    pub(crate) received_counter: i32,
    pub(crate) expected_counter: i32,
    sub_boot: Subscriber<'a, flume::Receiver<Sample>>,
    sub_counter: Subscriber<'a, flume::Receiver<Sample>>,
    pub cluster: OrderedMapPairs,
    pub polygon_list: OrderedMapPolygon,
    pub correct_polygon_list: OrderedMapPolygon,
    pub draw_count: i32,
}

impl BootNode<'_> {
    /// Create a new boot node instance with a node
    pub fn new(config: Config, cluster_name: &str) -> Self {
        let session = zenoh::open(config).res().unwrap().into_arc();
        let zid = session.zid().to_string();

        let counter_subscriber = session
            .declare_subscriber(format!("{}/counter/*", cluster_name))
            .reliable()
            .res()
            .unwrap();
        let boot_subscriber = session
            .declare_subscriber(format!("{}/node/boot/*", cluster_name))
            .reliable()
            .res()
            .unwrap();

        Self {
            session,
            zid,
            received_counter: 0,
            expected_counter: -1,
            sub_boot: boot_subscriber,
            sub_counter: counter_subscriber,
            cluster: OrderedMapPairs::new(),
            polygon_list: OrderedMapPolygon::new(),
            correct_polygon_list: OrderedMapPolygon::new(),
            draw_count: 0,
            cluster_name: cluster_name.to_string(),
        }
    }

    pub fn run(&mut self) {
        if let Ok(sample) = self.sub_boot.try_recv() {
            self.expected_counter = -1;
            self.received_counter = 0;

            let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
            println!("Message received on topic... {:?}", topic);
            let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
            // if sample.value.payload.is_contiguous() {
            //     let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
            // } else {
            //     let payload = sample.value.payload.contiguous();
            // }
            match topic {
                "new" => {
                    handle_join_request(payload, self);
                }
                "leave_request" => {
                    handle_leave_request(payload, self);
                }
                _ => println!("UNKNOWN BOOT TOPIC"),
            }

            while self.expected_counter != self.received_counter {
                while let Ok(sample) = self.sub_counter.try_recv() {
                    let topic = sample.key_expr.split('/').nth(2).unwrap_or("");
                    println!("Message received on topic... {:?}", topic);
                    let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                    match topic {
                        "expected_wait" => {
                            set_expected_counter(payload, &mut self.expected_counter);
                        }
                        "complete" => {
                            handle_task_completed(
                                payload,
                                &mut self.received_counter,
                                &mut self.polygon_list,
                                &mut self.cluster,
                            );
                        }
                        _ => println!("UNKNOWN COUNTER TOPIC"),
                    }
                }
            }

            if !self.cluster.is_empty() {
                //redraw distributed voronoi
                draw_voronoi_full(
                    &self.cluster,
                    &self.polygon_list,
                    format!("voronoi{}", self.draw_count).as_str(),
                );
                //correct voronoi
                let mut hash_map = self.cluster.clone();
                self.correct_polygon_list = OrderedMapPolygon::new();
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
                    self.correct_polygon_list
                        .insert(site_id.to_string(), polygon);
                }
                draw_voronoi_full(
                    &self.cluster,
                    &self.correct_polygon_list,
                    format!("confirm{}", self.draw_count).as_str(),
                );
                self.draw_count += 1;
            };
        }
    }

    pub fn get_zid(&self) -> String {
        self.zid.clone()
    }
}
