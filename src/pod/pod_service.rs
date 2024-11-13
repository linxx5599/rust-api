use k8s_openapi::api::core::v1::Pod;
use kube::{api::ObjectList, Api};
use serde_json::{json, to_value, Value};

use crate::kube_client;
use crate::common_mod::get_root_error;

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
            let mut msg = String::from("504: Gateway Timeout");
            msg.push_str(&get_root_error(&err).to_string());
            json!(&msg)
        },
    }
}
