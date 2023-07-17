use crate::message::NewVoronoiResponse;
use crate::node::SyncResolve;
use bincode::deserialize;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use uuid::Uuid;
use warp::{sse::Event, Filter};
use zenoh::prelude::r#async::AsyncResolve;
use zenoh::Session;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Player {
    pub(crate) player_id: String,
    pub(crate) x: f64,
    pub(crate) y: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct PlayerUpdate {
    pub(crate) player: Player,
    pub(crate) sender_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Initialize {
    pub(crate) players: Vec<Player>,
    pub(crate) polygon: Vec<(f64, f64)>,
    pub(crate) site: (f64, f64),
    pub(crate) sender_id: String,
}

fn sse_empty() -> Result<Event, Infallible> {
    Ok(warp::sse::Event::default().event("empty"))
}

fn sse_node_leave(payload: &[u8]) -> Result<Event, Infallible> {
    let data: String = deserialize(payload).unwrap();
    let data_string = serde_json::to_string(&data).unwrap();
    Ok(warp::sse::Event::default()
        .event("node_leave")
        .data(data_string))
}

fn sse_initialize(payload: &[u8]) -> Result<Event, Infallible> {
    let data: Initialize = deserialize(payload).unwrap();
    let data_string = serde_json::to_string(&data).unwrap();
    Ok(warp::sse::Event::default()
        .event("initialize")
        .data(data_string))
}

fn sse_polygon_update(payload: &[u8]) -> Result<Event, Infallible> {
    let data: NewVoronoiResponse = deserialize(payload).unwrap();
    let data_string = serde_json::to_string(&data).unwrap();
    Ok(warp::sse::Event::default()
        .event("polygon_update")
        .data(data_string))
}

fn sse_player_add(payload: &[u8]) -> Result<Event, Infallible> {
    let data: PlayerUpdate = deserialize(payload).unwrap();
    let data_string = serde_json::to_string(&data).unwrap();
    println!("sse_player_add: {}", data_string);
    Ok(warp::sse::Event::default()
        .event("player_add")
        .data(data_string))
}
fn sse_player_update(payload: &[u8]) -> Result<Event, Infallible> {
    let data: PlayerUpdate = deserialize(payload).unwrap();
    let data_string = serde_json::to_string(&data).unwrap();
    Ok(warp::sse::Event::default()
        .event("player_update")
        .data(data_string))
}
fn sse_player_remove(payload: &[u8]) -> Result<Event, Infallible> {
    let data: String = deserialize(payload).unwrap();
    let data_string = serde_json::to_string(&data).unwrap();
    Ok(warp::sse::Event::default()
        .event("remove_player")
        .data(data_string))
}

pub fn sse_server(session: Arc<Session>, cluster_name: String) {
    println!("STARTING SSE SERVER ON: http://127.0.0.1:3030/spacenet");
    let cluster_name_clone = cluster_name.clone();
    async_std::task::spawn(async move {
        let api_filter = warp::path("api").and(warp::get()).map(move || {
            let (zenoh_tx, zenoh_rx) = flume::unbounded();
            let session_clone = session.clone();
            let cluster_name_clone = cluster_name.clone();
            let sse_id = Uuid::new_v4();
            async_std::task::spawn(async move {
                let subscriber = session_clone
                    .declare_subscriber(format!("{}/sse/event/**", cluster_name_clone))
                    .with(flume::unbounded())
                    .res_async()
                    .await
                    .unwrap();
                while let Ok(sample) = subscriber.recv_async().await {
                    println!("IM STILL ON");
                    if zenoh_tx.send(sample).is_err() {
                        break;
                    }
                }
            });

            let stream = zenoh_rx.into_stream();
            let event_stream = stream.map(move |sample| {
                let sse_id = sse_id.to_string();
                let topic = sample.key_expr.split('/').nth(3).unwrap_or("");
                // let contiguous_payload = sample.value.payload.contiguous();
                // let payload = contiguous_payload.as_ref();
                let payload = sample.value.payload.get_zslice(0).unwrap().as_ref();
                match topic {
                    "initialize" => {
                        let id = sample.key_expr.split('/').nth(4).unwrap_or("");
                        if id == &sse_id {
                            sse_initialize(payload)
                        } else {
                            sse_empty()
                        }
                    }
                    "player_add" => sse_player_add(payload),
                    "player_update" => sse_player_update(payload),
                    "remove_player" => sse_player_remove(payload),
                    "polygon_update" => sse_polygon_update(payload),
                    "node_leave" => sse_node_leave(payload),
                    _ => sse_empty(),
                }
            });

            session
                .put(
                    format!("{}/sse/get/{}", cluster_name, sse_id.to_string()),
                    "",
                )
                .res_sync()
                .unwrap();
            warp::sse::reply(warp::sse::keep_alive().stream(event_stream))
        });

        let html = include_bytes!("index.html");
        let html_filter = warp::path("spacenet")
            .map(move || warp::reply::html(std::str::from_utf8(html).unwrap()));

        let js = include_bytes!("script.js");
        let js_filter = warp::path("script.js")
            .map(move || {
                let js_str = std::str::from_utf8(js).unwrap();
                warp::reply::with_header(js_str, "Content-Type", "application/javascript")
            });

        let html_filter = html_filter.or(js_filter);
        let routes= html_filter.or(api_filter);

        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    });
}
