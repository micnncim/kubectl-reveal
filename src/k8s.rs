use std::ops::Deref;

use k8s_openapi::api::core::v1::Secret;
use kube::config::KubeConfigOptions;
use kube::{Api, Client as KubeClient, Config};

pub struct Client {
    client: KubeClient,
}

impl Client {
    pub async fn new(context: Option<String>) -> anyhow::Result<Self> {
        let config = if let Some(context_name) = context {
            create_config(&context_name).await?
        } else {
            Config::infer().await?
        };

        let client = KubeClient::try_from(config)?;

        Ok(Self { client })
    }

    pub async fn get_secret(
        &self,
        name: String,
        namespace: Option<String>,
    ) -> anyhow::Result<Secret> {
        let secrets: Api<Secret> = Api::namespaced(
            self.client.clone(),
            namespace.unwrap_or_else(|| "default".to_string()).as_str(),
        );

        secrets.get(name.deref()).await.map_err(Into::into)
    }
}

async fn create_config(context_name: &str) -> anyhow::Result<Config> {
    let options = KubeConfigOptions {
        context: Some(context_name.to_string()),
        ..Default::default()
    };
    Config::from_kubeconfig(&options).await.map_err(Into::into)
}
