use crate::node::{NodeData, NodeStatus, SyncResolve};
use crate::sse::Player;
use bincode::serialize;
use std::sync::Arc;
use zenoh::Session;

#[derive(PartialEq, Clone, Debug)]
pub enum ApiMessage {
    GetStatus,
    GetNeighbours,
    GetPolygon,
    IsNeighbour(String),
    GetSite,
    SetStatus(NodeStatus),
    SetSite((f64, f64)),
    AddPlayer(Player),
    RemovePlayer(String),
    UpdatePlayer(Player),
}
#[derive(PartialEq, Clone, Debug)]
pub enum ApiResponse {
    GetStatusResponse(NodeStatus),
    GetNeighboursResponse(Vec<(String, (f64, f64))>),
    GetSiteResponse((f64, f64)),
    GetPolygonResponse(Vec<(f64, f64)>),
    IsNeighbourResponse(bool),
}

pub fn node_api_matcher(
    api_message: ApiMessage,
    node_data: &mut NodeData,
    api_responder_tx: &flume::Sender<ApiResponse>,
    session: &Arc<Session>,
    cluster_name:&str,
) {
    match api_message {
        ApiMessage::GetStatus => {
            let api_response = ApiResponse::GetStatusResponse(node_data.status.clone());
            api_responder_tx.send(api_response).unwrap();
        }
        ApiMessage::GetNeighbours => {
            let api_response = ApiResponse::GetNeighboursResponse(
                node_data.neighbours.clone().into_iter().collect(),
            );
            api_responder_tx.send(api_response).unwrap();
        }
        ApiMessage::GetPolygon => {
            let api_response = ApiResponse::GetPolygonResponse(node_data.polygon.clone());
            api_responder_tx.send(api_response).unwrap();
        }
        ApiMessage::IsNeighbour(zid) => {
            let api_response =
                ApiResponse::IsNeighbourResponse(node_data.neighbours.contains_key(zid.as_str()));
            api_responder_tx.send(api_response).unwrap();
        }
        ApiMessage::GetSite => {
            let api_response = ApiResponse::GetSiteResponse(node_data.site);
            api_responder_tx.send(api_response).unwrap();
        }
        ApiMessage::SetStatus(status) => {
            node_data.status = status;
        }
        ApiMessage::SetSite(site) => {
            node_data.site = site;
        }
        ApiMessage::AddPlayer(player) => {
            let message = serialize(&player).unwrap();
            session
                .put(format!("{}/sse/player_add",cluster_name), message)
                .res_sync()
                .unwrap();
            node_data
                .players
                .insert(player.player_id, (player.x, player.y));
        }
        ApiMessage::RemovePlayer(player_id) => {
            let message = serialize(&player_id).unwrap();
            session
                .put(format!("{}/sse/remove_player",cluster_name), message)
                .res_sync()
                .unwrap();
            node_data.players.remove(&player_id);
        }

        ApiMessage::UpdatePlayer(player) => {
            let message = serialize(&player).unwrap();
            session
                .put(format!("{}/sse/player_update",cluster_name), message)
                .res_sync()
                .unwrap();
            node_data
                .players
                .insert(player.player_id, (player.x, player.y));
        }
    };
}
