use crate::boot_node::BootNodeData;

#[derive(PartialEq, Clone, Debug)]
pub enum BootApiMessage {
    GetDrawCount,
    GetPolygonList,
    GetCorrectPolygonList,
    GetCluster,
}
#[derive(PartialEq, Clone, Debug)]
pub enum BootApiResponse {
    GetDrawCount(i32),
    GetPolygonList(Vec<(String, Vec<(f64, f64)>)>),
    GetCorrectPolygonList(Vec<(String, Vec<(f64, f64)>)>),
    GetCluster(Vec<(String, (f64, f64))>),
}

pub fn boot_api_matcher(
    api_message: BootApiMessage,
    boot_node_data: &mut BootNodeData,
    api_responder_tx: &flume::Sender<BootApiResponse>,
) {
    match api_message {
        BootApiMessage::GetCluster => {
            let api_response =
                BootApiResponse::GetCluster(boot_node_data.cluster.clone().into_iter().collect());
            api_responder_tx.send(api_response).unwrap();
        }
        BootApiMessage::GetPolygonList => {
            let api_response = BootApiResponse::GetPolygonList(
                boot_node_data.polygon_list.clone().into_iter().collect(),
            );
            api_responder_tx.send(api_response).unwrap();
        }

        BootApiMessage::GetCorrectPolygonList => {
            let api_response = BootApiResponse::GetCorrectPolygonList(
                boot_node_data
                    .correct_polygon_list
                    .clone()
                    .into_iter()
                    .collect(),
            );
            api_responder_tx.send(api_response).unwrap();
        }
        BootApiMessage::GetDrawCount => {
            let api_response = BootApiResponse::GetDrawCount(boot_node_data.draw_count);
            api_responder_tx.send(api_response).unwrap();
        }
    };
}
