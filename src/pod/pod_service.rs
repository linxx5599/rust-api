use k8s_openapi::api::core::v1::Pod;
use kube::{api::ObjectList, Api};
use serde_json::{json, to_value, Value};

#[path = "../client.rs"]
mod kube_client;

pub async fn get_pod() -> Value {
    let client = kube_client::MKubeClient::new().await.unwrap();
    //查询k3s下所有的pods
    let result: Result<ObjectList<Pod>, kube::Error> =
        Api::all(client).list(&Default::default()).await;
    match result {
        Ok(pods) => {
            // 处理成功的结果
            let pod_value = to_value(&pods).unwrap();
            return pod_value;
        }
        Err(err) => {
            // 处理错误
            eprintln!("错误: {:?}", err);
            // 您可以返回一个默认值或以不同的方式处理错误
            return json!({ "data": null, "status": 500, "message": "内部服务器错误" });
        }
    }
}
