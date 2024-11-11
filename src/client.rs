// use kube::config::Kubeconfig;
use kube::{
    config::{KubeConfigOptions, Kubeconfig},
    Config,
};
use kube_client::Client;

pub struct MKubeClient {}

impl MKubeClient {
    pub async fn new() -> Result<Client, Box<dyn std::error::Error>> {
        let kubeconfig = Kubeconfig::read_from("src/k3s.yaml").unwrap();
        let config = Config::from_custom_kubeconfig(kubeconfig, &KubeConfigOptions::default()).await?;
        let client = Client::try_from(config)?;
        Ok(client)
    }
}
