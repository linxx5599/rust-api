use k8s_openapi::api::core::v1::Node;
use kube::{api::ObjectList, Api};
use serde_json::{json, to_value, Value};

#[path = "../client.rs"]
mod kube_client;

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
            // 处理错误
            eprintln!("错误: {:?}", err);
            // 您可以返回一个默认值或以不同的方式处理错误
            return json!("500: Internal Server Error");
        }
    }
}
