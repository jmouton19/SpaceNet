use crate::node::{NodeData, NodeStatus};
use crate::sse::Player;

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
            node_data.players.insert(player.player_id, (player.x, player.y));
        }
        ApiMessage::RemovePlayer(player_id) => {
            node_data.players.remove(&player_id);
        }
        ApiMessage::UpdatePlayer(player) => {
            node_data.players.insert(player.player_id, (player.x, player.y));
        }
    };
}
