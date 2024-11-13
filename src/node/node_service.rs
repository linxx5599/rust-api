use k8s_openapi::api::core::v1::Node;
use kube::{api::ObjectList, Api};
use serde_json::{json, to_value, Value};

#[path = "../client.rs"]
mod kube_client;

fn get_root_error(err: &dyn std::error::Error) -> &dyn std::error::Error {
    let mut current_err = err;
    while let Some(source) = current_err.source() {
        current_err = source;
    }
    current_err
}

pub async fn get_node() -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的nodes
    let result: Result<ObjectList<Node>, kube::Error> =
        Api::all(client).list(&Default::default()).await;
    match result {
        Ok(nodes) => {
            // 处理成功的结果
            let node_value = to_value(&nodes).unwrap();
            return node_value;
        }
        Err(err) => {
            let mut msg = String::from("504: Gateway Timeout");
            msg.push_str(&get_root_error(&err).to_string());
            json!(&msg)
        },
    }
}
