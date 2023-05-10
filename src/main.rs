use k8s_openapi::api::apps::v1::Deployment;
use kube::Api;
use std::time::Duration;

mod k8s;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = config::get_environment();
    let client = k8s::get_client();
    let deployments: Api<Deployment> = Api::namespaced(client, &env.namespace);

    loop {
        match deployments.get(&env.deployment).await {
            Ok(depl_info) => {
                if let Some(spec) = depl_info.spec {
                    println!("{:?}", spec);
                }
            },
            Err(e) => {
                eprintln!("Error getting deployment: {}", e);
            },
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
